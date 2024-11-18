"use client";
import {Container, Pagination, TextInput, Title} from "@mantine/core";
import { GroupsResponse } from "@/service/types/tasky";
import GroupsDisplayComponent from "@/app/groups/displayComponent";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import {useDebouncedValue} from "@mantine/hooks";

const GroupsPage = () => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState<string>("");

  const [debouncedSearch] = useDebouncedValue(search, 500);

  const [groups, refetch] = useClientQuery<GroupsResponse>(
    () => api.getGroups(page, debouncedSearch),
    [page, debouncedSearch],
  );
  const { t } = useTranslation(["group", "common"]);

  return (
    <Container fluid>
      <Title>{t("groups")}</Title>
        <TextInput
            label={t('common:fields.search')}
            value={search}
            onChange={(e) => setSearch(e.target.value)}
        />
      <GroupsDisplayComponent
        groups={groups?.groups ?? []}
        page="groups"
        refetch={refetch}
      />
      <Pagination
        total={Math.ceil((groups?.total ?? 0) / 50)}
        value={page}
        onChange={setPage}
      />
    </Container>
  );
};

export default GroupsPage;
