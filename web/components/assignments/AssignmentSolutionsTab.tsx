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
import {useTranslation} from "react-i18next";

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
  const {t} = useTranslation(['common', 'solution', 'assignment']);

  const cols: EntityListCol[] = [
    {
      field: "id",
      label: t('cols.id'),
    },
    {
      field: "submitter",
      label: t('cols.submitter'),
      getter: (row) => row.submitter.username,
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
