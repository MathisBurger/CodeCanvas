import { Group } from "@/service/types/tasky";
import {
  Button,
  Container,
  Group as GroupComponent,
  Pagination,
} from "@mantine/core";
import { IconPlus } from "@tabler/icons-react";
import { useState } from "react";
import CreateAssignmentWishModal from "@/components/group/CreateAssignmentWishModal";
import EntityList, {
  EntityListCol,
  EntityListRowAction,
} from "@/components/EntityList";
import useClientQuery from "@/hooks/useClientQuery";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { UserRoles } from "@/service/types/usernator";
import { showNotification } from "@mantine/notifications";
import { useTranslation } from "react-i18next";

interface GroupAssignmentWishesTabProps {
  group: Group;
}

const GroupAssignmentWishesTab = ({ group }: GroupAssignmentWishesTabProps) => {
  const [createModalOpen, setCreateModalOpen] = useState(false);
  const { t } = useTranslation(["common", "assignment"]);
  const [page, setPage] = useState(1);
  const api = useApiServiceClient();
  const [wishes, refetch] = useClientQuery(
    () => api.getAssignmentWishes(group.id, page),
    [group, page],
  );

  const cols: EntityListCol[] = [
    {
      field: "title",
      label: t("fields.title"),
    },
    {
      field: "description",
      label: t("fields.description"),
    },
  ];

  const rowActions: EntityListRowAction[] = [
    {
      auth: [UserRoles.Admin, UserRoles.Tutor],
      name: t("actions.delete"),
      onClick: async (row) => {
        try {
          await api.deleteAssignmentWish(group.id, row.id);
          refetch();
        } catch (e: any) {
          showNotification({
            title: t("messages.error"),
            message: e?.message ?? "",
          });
        }
      },
      color: "red",
    },
  ];

  return (
    <>
      <Container fluid pb={30}>
        <GroupComponent justify="end" mb={20}>
          <Button onClick={() => setCreateModalOpen(true)}>
            <IconPlus />
            &nbsp;{t("assignment:titles.create-wish")}
          </Button>
        </GroupComponent>
        <EntityList
          cols={cols}
          rows={wishes?.results ?? []}
          rowActions={rowActions}
        />
        <Pagination
          total={Math.ceil((wishes?.total ?? 0) / 50)}
          value={page}
          onChange={setPage}
        />
      </Container>
      {createModalOpen && (
        <CreateAssignmentWishModal
          onClose={() => setCreateModalOpen(false)}
          refetch={refetch}
          groupId={group.id}
        />
      )}
    </>
  );
};

export default GroupAssignmentWishesTab;
