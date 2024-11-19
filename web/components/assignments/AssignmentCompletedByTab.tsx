import EntityList, { EntityListCol } from "@/components/EntityList";
import { useTranslation } from "react-i18next";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import React, {useState} from "react";
import {Pagination} from "@mantine/core";

interface AssignmentCompletedByTabProps {
  groupId: number;
  assignmentId: number;
}

const AssignmentCompletedByTab = ({
    groupId,
  assignmentId,
}: AssignmentCompletedByTabProps) => {
  const { t } = useTranslation("common");
  const api = useApiServiceClient();
  const [page, setPage] = useState<number>(1);
  const [completedBy] = useClientQuery(() => api.getAssignmentCompletions(groupId, assignmentId, page), [groupId, assignmentId, page]);

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

  return (
      <>
        <EntityList cols={cols} rows={completedBy?.completions ?? []} />
        <Pagination
            total={Math.ceil((completedBy?.total ?? 0) / 50)}
            value={page}
            onChange={setPage}
        />
      </>
  );
};

export default AssignmentCompletedByTab;
