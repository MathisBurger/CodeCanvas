"use client";
import { Badge, Pagination, Tabs, Group, Button, Stack } from "@mantine/core";
import React, { useState } from "react";
import {
  Group as TaskyGroup, GroupJoinRequestPolicy,
  GroupJoinRequestResponse,
  TaskyUser,
} from "@/service/types/tasky";
import EntityList, {
  EntityListCol,
  EntityListRowAction,
} from "@/components/EntityList";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {UserRoles} from "@/service/types/usernator";
import GroupAssignmentsTab from "@/components/assignments/GroupAssignmentsTab";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";
import GroupAssignmentWishesTab from "@/components/group/GroupAssignmentWishesTab";
import {useTranslation} from "react-i18next";
import EnlistUserModal from "@/components/group/EnlistUserModal";
import {showNotification} from "@mantine/notifications";

const MembersComponent: React.FC<{ members: TaskyUser[], group: TaskyGroup, refetch: () => void }> = ({ members, group, refetch }) => {
  const { t } = useTranslation(["common", "group"]);
  const {user} = useCurrentUser();
  const api = useApiServiceClient();
  const [enlistModalOpen, setEnlistModalOpen] = useState<boolean>(false);

  const removeUser = async (memberId: number) => {
    try {
      await api.removeUserFromGroup(group.id, memberId);
      refetch();
    } catch (e: any) {
      showNotification({
        title: t('messages.error'),
        message: e.message ?? "",
      });
    }
  }

  const cols: EntityListCol[] = [
    {
      field: "id",
      label: t("cols.id"),
    },
    {
      field: "username",
      label: t("cols.username"),
    },
  ];

  const rowActions: EntityListRowAction[] = [
    {
      name: t('common:actions.remove'),
      color: 'red',
      onClick: (row) => removeUser(row.id),
      auth: [UserRoles.Tutor, UserRoles.Admin],
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
      authFunc: (_) => (isGranted(user, [UserRoles.Tutor]) && group.tutor.id === user?.id) || isGranted(user, [UserRoles.Admin])
    }
  ];

  return (
      <Stack gap={10}>
        <Group justify="flex-end">
          {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
              <Button onClick={() => setEnlistModalOpen(true)}>{t('group:actions.enlist-user')}</Button>
          )}
        </Group>
        <EntityList cols={cols} rows={members} rowActions={rowActions} />
        {enlistModalOpen && (
            <EnlistUserModal onClose={() => setEnlistModalOpen(false)} groupId={group.id} refetch={refetch} />
        )}
      </Stack>
  );
};

export const JoinRequestsComponent: React.FC<{
  group: TaskyGroup | null;
  refetchParent: () => void;
}> = ({ group, refetchParent }) => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const [requests, refetch] = useClientQuery<GroupJoinRequestResponse>(
    () => api.getGroupJoinRequests(group?.id ?? -1, page),
    [group?.id, page],
  );
  const { t } = useTranslation("common");

  const cols: EntityListCol[] = [
    {
      field: "id",
      label: t("cols.id"),
    },
    {
      field: "username",
      label: t("cols.username"),
      getter: (row) => row.requestor.username,
    },
  ];

  const actions: EntityListRowAction[] = [
    {
      name: t("actions.approve"),
      color: "green",
      onClick: (row) =>
        api.approveGroupJoinRequest(row.group_id, row.id).then(() => {
          refetch();
          refetchParent();
        }),
      auth: [UserRoles.Tutor, UserRoles.Admin],
    },
    {
      name: t("actions.reject"),
      color: "red",
      onClick: (row) =>
        api.rejectGroupJoinRequest(row.group_id, row.id).then(() => {
          refetch();
          refetchParent();
        }),
      auth: [UserRoles.Tutor, UserRoles.Admin],
    },
  ];

  return (
    <>
      <EntityList
        cols={cols}
        rows={requests ? (requests as GroupJoinRequestResponse).requests : []}
        rowActions={actions}
      />
      <Pagination
        total={Math.ceil((requests?.total ?? 0) / 50)}
        value={page}
        onChange={setPage}
      />
    </>
  );
};

export const TabsComponent: React.FC<{
  group: TaskyGroup | null;
  refetch: () => void;
}> = ({ group, refetch }) => {
  const { user } = useCurrentUser();
  const { t } = useTranslation("group");

  return (
    <Tabs defaultValue="assignments" style={{ marginTop: "2em" }}>
      <Tabs.List>
        <Tabs.Tab value="assignments">{t("tabs.assignments")}</Tabs.Tab>
        <Tabs.Tab value="members">{t("tabs.members")}</Tabs.Tab>
        <Tabs.Tab value="assignmentWishes">
          {t("tabs.assignment-wishes")}
        </Tabs.Tab>
        {isGranted(user, [UserRoles.Admin, UserRoles.Tutor]) && group?.join_policy === GroupJoinRequestPolicy.Request &&  (
          <Tabs.Tab
            value="joinRequests"
            rightSection={
              group && group.request_count > 0 ? (
                <Badge color="red">{group.request_count}</Badge>
              ) : null
            }
          >
            {t("tabs.join-requests")}
          </Tabs.Tab>
        )}
      </Tabs.List>
      <div style={{ marginTop: "2em" }}>
        <Tabs.Panel value="assignments">
          <GroupAssignmentsTab group={group} />
        </Tabs.Panel>
        <Tabs.Panel value="members">
          {group && (
              <MembersComponent members={group?.members ?? []} group={group} refetch={refetch} />
          )}
        </Tabs.Panel>
        <Tabs.Panel value="assignmentWishes">
          {group !== null && <GroupAssignmentWishesTab group={group} />}
        </Tabs.Panel>
        {isGranted(user, [UserRoles.Admin, UserRoles.Tutor]) && (
          <Tabs.Panel value="joinRequests">
            {group !== null && group !== undefined && group.join_policy === GroupJoinRequestPolicy.Request &&  (
              <JoinRequestsComponent group={group} refetchParent={refetch} />
            )}
          </Tabs.Panel>
        )}
      </div>
    </Tabs>
  );
};
