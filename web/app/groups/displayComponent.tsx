"use client";
import EntityList, {
  EntityListCol,
  EntityListRowAction,
} from "@/components/EntityList";
import { useRouter } from "next/navigation";
import { MinifiedGroup } from "@/service/types/tasky";
import { UserRoles } from "@/service/types/usernator";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { notifications } from "@mantine/notifications";
import useCurrentUser from "@/hooks/useCurrentUser";
import { isGranted } from "@/service/auth";
import {useTranslation} from "react-i18next";

interface DisplayComponentProps {
  groups: MinifiedGroup[];
  refetch?: () => void;
  page: "my-groups" | "groups";
}

const GroupsDisplayComponent = ({
  groups,
  page,
  refetch,
}: DisplayComponentProps) => {
  const router = useRouter();
  const {t} = useTranslation(['common', 'group']);
  const cols: EntityListCol[] = [
    {
      field: "id",
      label: t('cols.id'),
    },
    {
      field: "title",
      label: t('group:cols.title'),
    },
    {
      field: "member_count",
      label: t('group:cols.members-count'),
    },
    {
      field: "tutor",
      label: t('group:cols.tutor'),
      getter: (row) => row.tutor.username,
    },
  ];
  const api = useApiServiceClient();
  const { user } = useCurrentUser();

  const actions: EntityListRowAction[] = [
    {
      color: "blue",
      name: t('common:actions.view'),
      onClick: (row) => router.push(`/groups/${row.id}`),
      auth: [UserRoles.Admin, UserRoles.Tutor, UserRoles.Student],
      authFunc: (row) =>
        page === "groups"
          ? isGranted(user, [UserRoles.Admin]) || (isGranted(user, [UserRoles.Tutor]) && (user?.groups.map((g) => g.id) ?? []).indexOf(row.id) > -1)
          : true,
    },
    {
      color: "blue",
      name: t('group:actions.request-join'),
      onClick: (row) =>
        api.createGroupJoinRequest(row.id).then(() => {
          notifications.show({
            title: t('messages.join-request-created-title'),
            message: t('messages.join-request-created-text') + row.title,
          });
          if (refetch) refetch();
        }),
      auth: [UserRoles.Student],
      authFunc: (row) =>
        (user?.groups ?? []).map((g) => g.id).indexOf(row.id) === -1 &&
        page === "groups",
    },
  ];

  return <EntityList cols={cols} rows={groups} rowActions={actions} />;
};

export default GroupsDisplayComponent;
