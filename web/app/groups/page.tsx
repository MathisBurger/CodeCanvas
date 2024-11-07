"use client";
import {Container, Pagination, Title} from "@mantine/core";
import { GroupsResponse } from "@/service/types/tasky";
import GroupsDisplayComponent from "@/app/groups/displayComponent";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {useState} from "react";
import {useTranslation} from "react-i18next";

const GroupsPage = () => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const [groups, refetch] = useClientQuery<GroupsResponse>(() =>
    api.getGroups(page),
  [page]);
  const {t} = useTranslation('groups');

  return (
    <Container fluid>
      <Title>{t('groups')}</Title>
      <GroupsDisplayComponent
        groups={groups?.groups ?? []}
        page="groups"
        refetch={refetch}
      />
        <Pagination total={Math.ceil((groups?.total ?? 0) / 50)} value={page} onChange={setPage} />
    </Container>
  );
};

export default GroupsPage;
