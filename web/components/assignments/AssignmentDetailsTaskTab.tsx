import { Assignment } from "@/service/types/tasky"
import {Title} from "@mantine/core";
import RichTextDisplay from "@/components/display/RichTextDisplay";
import FileStructure from "@/components/FileStructure";


interface AssignmentDetailsTaskProps {
    assignment: Assignment|null;
}

const AssignmentDetailsTaskTab = ({assignment}: AssignmentDetailsTaskProps) => {

    return (
        <>
            <Title order={3}>Task</Title>
            <RichTextDisplay content={assignment?.description ?? ""} fullSize={true} />
            <Title order={3}>Required files</Title>
            {assignment !== null && assignment.file_structure !== null && (
                <FileStructure structure={assignment.file_structure} editable={false} displayMode="task" />
            )}
        </>
    )
}

export default AssignmentDetailsTaskTab;
