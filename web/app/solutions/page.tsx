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

const PersonalSolutionsPage = () => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const router = useRouter();
  const [solutions] = useClientQuery<SolutionsResponse>(() =>
    api.getPersonalSolutions(page), [page]
  );

  const cols: EntityListCol[] = [
    {
      field: "id",
      label: "ID",
    },
    {
      field: "assignment",
      label: "Assignment",
      getter: (row) => row.assignment.title,
    },
    {
      field: "approval_status",
      label: "Approval Status",
      render: (value) => (
        <SolutionBadge status={value as string} />
      ),
    },
  ];

  const rowActions: EntityListRowAction[] = [
    {
      name: "View",
      onClick: (row) => router.push(`/solutions/${row.id}`),
      color: undefined,
      auth: [UserRoles.Student],
    },
  ];

  return (
    <Container fluid>
      <Title order={1} mb={20}>
        Personal solutions
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
