import { TaskyUser } from "@/service/types/tasky"
import EntityList, {EntityListCol} from "@/components/EntityList";

interface AssignmentCompletedByTabProps {
    completedBy: TaskyUser[];
}

const AssignmentCompletedByTab = ({completedBy}: AssignmentCompletedByTabProps) => {

    const cols: EntityListCol[] = [
        {
            field: 'id',
            label: 'ID'
        },
        {
            field: 'username',
            label: 'Username'
        }
    ]

    return (
        <EntityList cols={cols} rows={completedBy} />
    )
}

export default AssignmentCompletedByTab;
