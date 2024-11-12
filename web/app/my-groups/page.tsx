"use client";
import { Button, Container, Pagination, Title } from "@mantine/core";
import { GroupsResponse } from "@/service/types/tasky";
import GroupsDisplayComponent from "@/app/groups/displayComponent";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import { useState } from "react";
import CreateGroupModal from "@/components/group/CreateGroupModal";
import useCurrentUser from "@/hooks/useCurrentUser";
import { isGranted } from "@/service/auth";
import { UserRoles } from "@/service/types/usernator";
import { useTranslation } from "react-i18next";

const GroupsPage = () => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const [groups] = useClientQuery<GroupsResponse>(
    () => api.getMyGroups(page),
    [page],
  );
  const [createModalOpen, setCreateModalOpen] = useState(false);
  const { user } = useCurrentUser();
  const { t } = useTranslation("group");

  return (
    <Container fluid>
      <Title>{t("my-groups")}</Title>
      {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
        <Button onClick={() => setCreateModalOpen(true)}>
          {t("actions.create-group")}
        </Button>
      )}
      <GroupsDisplayComponent groups={groups?.groups ?? []} page="my-groups" />
      <Pagination
        total={Math.ceil((groups?.total ?? 0) / 50)}
        value={page}
        onChange={setPage}
      />
      {createModalOpen && (
        <CreateGroupModal onClose={() => setCreateModalOpen(false)} />
      )}
    </Container>
  );
};

export default GroupsPage;
