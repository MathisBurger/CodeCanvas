import EntityList, {
  EntityListCol,
  EntityListRowAction,
} from "@/components/EntityList";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import { SolutionsResponse } from "@/service/types/tasky";
import { useRouter } from "next/navigation";
import { UserRoles } from "@/service/types/usernator";
import SolutionBadge from "@/components/solution/SolutionBadge";
import {useState} from "react";
import {Container, Pagination} from "@mantine/core";

interface AssignmentSolutionsTabProps {
  assignmentId: number;
}

const AssignmentSolutionsTab = ({
  assignmentId,
}: AssignmentSolutionsTabProps) => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const router = useRouter();
  const [solutions] = useClientQuery<SolutionsResponse>(
    () => api.getSolutionsForAssignment(assignmentId, page),
    [assignmentId, page],
  );

  const cols: EntityListCol[] = [
    {
      field: "id",
      label: "ID",
    },
    {
      field: "submitter",
      label: "Submitter",
      getter: (row) => row.submitter.username,
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
      auth: [UserRoles.Admin, UserRoles.Tutor],
    },
  ];

  return (
    <Container fluid>
      <EntityList
          cols={cols}
          rows={solutions?.solutions ?? []}
          rowActions={rowActions}
      />
      <Pagination total={Math.ceil((solutions?.total ?? 0) / 50)} value={page} onChange={setPage} />
    </Container>
  );
};

export default AssignmentSolutionsTab;
