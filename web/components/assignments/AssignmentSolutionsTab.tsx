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

interface AssignmentSolutionsTabProps {
  assignmentId: number;
}

const AssignmentSolutionsTab = ({
  assignmentId,
}: AssignmentSolutionsTabProps) => {
  const api = useApiServiceClient();
  const router = useRouter();
  const [solutions] = useClientQuery<SolutionsResponse>(
    () => api.getSolutionsForAssignment(assignmentId),
    [assignmentId],
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
      render: (value, row) => (
        <SolutionBadge status={value as string} job={row.job} />
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
    <EntityList
      cols={cols}
      rows={solutions?.solutions ?? []}
      rowActions={rowActions}
    />
  );
};

export default AssignmentSolutionsTab;
