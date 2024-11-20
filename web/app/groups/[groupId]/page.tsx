"use client";
import {Badge, Button, Container, Group, Title} from "@mantine/core";
import { Group as GroupType } from "@/service/types/tasky";
import { TabsComponent } from "./client";
import useClientQuery from "@/hooks/useClientQuery";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import CentralLoading from "@/components/CentralLoading";
import { useSpotlightStage2 } from "@/hooks/spotlight/stage2";
import {useEffect, useState} from "react";
import { useTranslation } from "react-i18next";
import GroupJoinPolicyBadge from "@/components/group/GroupJoinPolicyBadge";
import UpdateGroupModal from "@/components/group/UpdateGroupModal";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";
import {UserRoles} from "@/service/types/usernator";
import LeaveGroupModal from "@/components/group/LeaveGroupModal";
import DeleteGroupModal from "@/components/group/DeleteGroupModal";
import VerifiedBadge from "@/components/VerifiedBadge";
import NavigateBack from "@/components/NavigateBack";
import CreateGroupNotificationModal from "@/components/CreateGroupNotificationModal";

const GroupDetailsPage = ({ params }: { params: { groupId: string } }) => {
  const id = parseInt(`${params.groupId}`, 10);
  const api = useApiServiceClient();
  const {user} = useCurrentUser();
  const [group, refetch] = useClientQuery<GroupType>(() => api.getGroup(id));
  const { addGroup } = useSpotlightStage2();
  const [updateModalOpen, setUpdateModalOpen] = useState<boolean>(false);
  const [leaveModalOpen, setLeaveModalOpen] = useState<boolean>(false);
  const [deleteModalOpen, setDeleteModalOpen] = useState<boolean>(false);
  const [createNotificationModalOpen, setCreateNotificationModalOpen] = useState<boolean>(false);
  const { t } = useTranslation("common");

  const changeVerifiedState = async () => {
    if (group) {
      if (group.verified) {
        await api.unverify(group.id);
      } else {
        await api.verify(group.id);
      }
      refetch();
    }
  }

  useEffect(() => {
    if (group) {
      addGroup(group);
    }
  }, [addGroup, group]);

  if (isNaN(id)) {
    return (
      <Container fluid>
        <Title>{t("invalid-group-id")}</Title>
      </Container>
    );
  }

  return (
    <Container fluid>
      <NavigateBack />
      <Group>
        <Title>{group?.title ?? "Loading"}</Title>
        <Badge>{group?.tutor?.username ?? "Loading"}</Badge>
        {group?.join_policy && (
            <GroupJoinPolicyBadge policy={group.join_policy} />
        )}
        {group?.verified && (
            <VerifiedBadge />
        )}
        {(isGranted(user, [UserRoles.Admin]) || group?.tutor.id === user?.id) && (
          <>
            <Button onClick={() => setUpdateModalOpen(true)}>{t('common:titles.update-group')}</Button>
            <Button color="red" onClick={() => setDeleteModalOpen(true)}>{t('common:actions.delete')}</Button>

            <Button color="pink" onClick={() => setCreateNotificationModalOpen(true)}>{t('common:actions.create-notification')}</Button>
          </>
        )}
        {isGranted(user, [UserRoles.Student]) && (
            <Button color="red" onClick={() => setLeaveModalOpen(true)}>{t('group:actions.leave')}</Button>
        )}
        {isGranted(user, [UserRoles.Admin]) && (
            <Button color="cyan" onClick={changeVerifiedState}>{group?.verified ? t('group:actions.unverify') : t('group:actions.verify')}</Button>
        )}
      </Group>
      {group === null ? (
        <CentralLoading />
      ) : (
        <TabsComponent group={group} refetch={refetch} />
      )}
      {updateModalOpen && group !== null && (
          <UpdateGroupModal
              group={group}
              onClose={() => setUpdateModalOpen(false)}
              refetch={refetch}
          />
      )}
      {leaveModalOpen && group && (
          <LeaveGroupModal groupId={group.id} onClose={() => setLeaveModalOpen(false)} />
      )}
      {deleteModalOpen && group !== null && (
          <DeleteGroupModal groupId={group.id} onClose={() => setDeleteModalOpen(false)} />
      )}
      {createNotificationModalOpen && group !== null && (
          <CreateGroupNotificationModal groupId={group.id} onClose={() => setCreateNotificationModalOpen(false)} />
      )}
    </Container>
  );
};

export default GroupDetailsPage;
