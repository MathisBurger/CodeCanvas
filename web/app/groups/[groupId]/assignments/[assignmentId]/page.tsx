'use client';
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {Badge, Button, Container, Group, Title} from "@mantine/core";
import AssignmentDateDisplay from "@/components/assignments/AssignmentDateDisplay";
import NavigateBack from "@/components/NavigateBack";
import RichTextDisplay from "@/components/display/RichTextDisplay";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";
import {UserRoles} from "@/service/types/usernator";
import {useState} from "react";
import CreateOrUpdateAssignmentModal from "@/components/assignments/CreateOrUpdateAssignmentModal";
import CentralLoading from "@/components/CentralLoading";
import AssignmentCreateOrUpdateCodeTestModal from "@/components/assignments/AssignmentCreateOrUpdateCodeTestModal";
import {AssignmentLanguage} from "@/service/types/tasky";


const AssignmentDetailsPage = ({params}: {params: {groupId: string, assignmentId: string}}) => {

    const groupId = parseInt(`${params.groupId}`, 10);
    const assignmentId = parseInt(`${params.assignmentId}`, 10);
    const api = useApiServiceClient();
    const {user} = useCurrentUser();
    const [updateModalOpen, setUpdateModalOpen] = useState(false);
    const [fileStructureModalOpen, setFileStructureModalOpen] = useState(false);
    const [assignment, refetch] = useClientQuery(() => api.getAssignmentForGroup(groupId, assignmentId), [assignmentId, groupId]);

    if (isNaN(groupId) || isNaN(assignmentId)) {
        return (
            <Container fluid>
                <Title>Invalid Group ID</Title>
            </Container>
        )
    }

    if (assignment === null) {
        return <CentralLoading />;
    }

    return (
        <Container fluid>
            <NavigateBack />
            <Group>
                <Title order={1}>{assignment?.title}</Title>
                <Badge color="indigo">{assignment?.language}</Badge>
                <AssignmentDateDisplay date={assignment?.due_date ?? ""} />
                {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
                    <Button onClick={() => setUpdateModalOpen(true)}>Edit</Button>
                )}
                {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && assignment.file_structure === null && assignment.language !== AssignmentLanguage.QuestionBased && (
                    <Button onClick={() => setFileStructureModalOpen(true)}>Create code tests</Button>
                )}
                {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && assignment.file_structure !== null && assignment.language !== AssignmentLanguage.QuestionBased && (
                    <Button>Show code tests</Button>
                )}
            </Group>
            <RichTextDisplay content={assignment?.description ?? ""} fullSize={true} />
            {updateModalOpen && (
                <CreateOrUpdateAssignmentModal
                    groupId={groupId}
                    onClose={() => setUpdateModalOpen(false)}
                    refetch={refetch}
                    action="update"
                    assignment={assignment ?? undefined}
                />
            )}
            {fileStructureModalOpen && (
                <AssignmentCreateOrUpdateCodeTestModal
                    onClose={() => setFileStructureModalOpen(false)}
                    groupId={groupId}
                    assignmentId={assignmentId}
                    refetch={refetch}
                />
            )}
        </Container>
    )
}

export default AssignmentDetailsPage;
