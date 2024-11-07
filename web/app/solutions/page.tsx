"use client";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import { SolutionsResponse } from "@/service/types/tasky";
import {Container, Pagination, Title} from "@mantine/core";
import EntityList, {
  EntityListCol,
  EntityListRowAction,
} from "@/components/EntityList";
import { UserRoles } from "@/service/types/usernator";
import SolutionBadge from "@/components/solution/SolutionBadge";
import { useRouter } from "next/navigation";
import {useState} from "react";
import {useTranslation} from "react-i18next";

const PersonalSolutionsPage = () => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const router = useRouter();
  const [solutions] = useClientQuery<SolutionsResponse>(() =>
    api.getPersonalSolutions(page), [page]
  );
  const {t} = useTranslation(['common', 'solution']);

  const cols: EntityListCol[] = [
    {
      field: "id",
      label: t('cols.id'),
    },
    {
      field: "assignment",
      label: t('cols.assignment'),
      getter: (row) => row.assignment.title,
    },
    {
      field: "approval_status",
      label: t('cols.approval-status'),
      render: (value) => (
        <SolutionBadge status={value as string} />
      ),
    },
  ];

  const rowActions: EntityListRowAction[] = [
    {
      name: t('actions.view'),
      onClick: (row) => router.push(`/solutions/${row.id}`),
      color: undefined,
      auth: [UserRoles.Student],
    },
  ];

  return (
    <Container fluid>
      <Title order={1} mb={20}>
        {t('personal-solutions')}
      </Title>
      <EntityList
        cols={cols}
        rowActions={rowActions}
        rows={solutions?.solutions ?? []}
      />
      <Pagination total={Math.ceil((solutions?.total ?? 0) / 50)} value={page} onChange={setPage} />
    </Container>
  );
};

export default PersonalSolutionsPage;
