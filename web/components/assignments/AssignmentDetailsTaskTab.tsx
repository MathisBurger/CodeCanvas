import {Assignment} from "@/service/types/tasky"
import {Button, Title} from "@mantine/core";
import RichTextDisplay from "@/components/display/RichTextDisplay";
import FileStructure from "@/components/FileStructure";
import {useMemo, useState} from "react";
import {isGranted} from "@/service/auth";
import useCurrentUser from "@/hooks/useCurrentUser";
import {UserRoles} from "@/service/types/usernator";
import CreateTaskCodeModal from "@/components/assignments/CreateSolutionModal";


interface AssignmentDetailsTaskProps {
    assignment: Assignment|null;
}

const AssignmentDetailsTaskTab = ({assignment}: AssignmentDetailsTaskProps) => {

    const {user} = useCurrentUser();
    const [createSolutionModalOpen, setCreateSolutionModalOpen] = useState(false);

    const assignmentCompleted = useMemo<boolean>(() => {
        return (assignment?.completed_by ?? []).map((u) => u.id).indexOf(user?.id ?? -1) > -1;
    }, [user, assignment]);

    return (
        <>
            <Title order={3}>Task</Title>
            <RichTextDisplay content={assignment?.description ?? ""} fullSize={true} />
            <Title order={3} mb={10}>Required files</Title>
            {assignment !== null && assignment.file_structure !== null && (
                <FileStructure structure={assignment.file_structure} editable={false} displayMode="task" />
            )}
            {!assignmentCompleted && isGranted(user, [UserRoles.Student]) && (
                <Button
                    color="lime"
                    mt={20}
                    onClick={() => setCreateSolutionModalOpen(true)}
                >
                    Create Solution
                </Button>
            )}
            {createSolutionModalOpen && assignment !== null && (
                <CreateTaskCodeModal
                    onClose={() => setCreateSolutionModalOpen(false)}
                    assignment={assignment}
                />
            )}
        </>
    )
}

export default AssignmentDetailsTaskTab;
