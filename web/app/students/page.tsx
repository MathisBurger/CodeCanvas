"use client";
import {GetStudentsResponse, UserRoles} from "@/service/types/usernator";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {Container, Pagination, Title} from "@mantine/core";
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import useClientQuery from "@/hooks/useClientQuery";
import {useState} from "react";
import {useTranslation} from "react-i18next";
import {useRouter} from "next/navigation";

const StudentsPage = () => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const router = useRouter();
  const [students] = useClientQuery<GetStudentsResponse>(
    () => api.getStudents(page),
    [page],
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
    },
  ];

  const rowActions: EntityListRowAction[] = [
    {
      auth: [UserRoles.Admin],
      name: t('actions.view'),
      onClick: (row) => router.push(`/user-solutions/${row.id}`),
      color: 'indigo'
    }
  ];

  return (
    <Container fluid>
      <Title>{t("students")}</Title>
      <EntityList cols={cols} rows={students?.students ?? []} rowActions={rowActions} />
      <Pagination
        total={Math.ceil((students?.total ?? 0) / 50)}
        value={page}
        onChange={setPage}
      />
    </Container>
  );
};

export default StudentsPage;
