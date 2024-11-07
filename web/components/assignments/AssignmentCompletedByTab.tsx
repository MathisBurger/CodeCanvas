import { TaskyUser } from "@/service/types/tasky";
import EntityList, { EntityListCol } from "@/components/EntityList";
import {useTranslation} from "react-i18next";

interface AssignmentCompletedByTabProps {
  completedBy: TaskyUser[];
}

const AssignmentCompletedByTab = ({
  completedBy,
}: AssignmentCompletedByTabProps) => {

  const {t} = useTranslation('common');

  const cols: EntityListCol[] = [
    {
      field: "id",
      label: t('cols.id'),
    },
    {
      field: "username",
      label: t('cols.username'),
    },
  ];

  return <EntityList cols={cols} rows={completedBy} />;
};

export default AssignmentCompletedByTab;
