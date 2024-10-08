'use client';
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {Badge, Button, Container, Group, Tabs, Title} from "@mantine/core";
import AssignmentDateDisplay from "@/components/assignments/AssignmentDateDisplay";
import NavigateBack from "@/components/NavigateBack";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";
import {UserRoles} from "@/service/types/usernator";
import {useState} from "react";
import CreateOrUpdateAssignmentModal from "@/components/assignments/CreateOrUpdateAssignmentModal";
import CentralLoading from "@/components/CentralLoading";
import AssignmentCreateOrUpdateCodeTestModal from "@/components/assignments/AssignmentCreateOrUpdateCodeTestModal";
import {AssignmentLanguage} from "@/service/types/tasky";
import FileStructureDisplay from "@/components/FileStructureDisplay";
import AssignmentDetailsTaskTab from "@/components/assignments/AssignmentDetailsTaskTab";
import AssignmentSolutionsTab from "@/components/assignments/AssignmentSolutionsTab";
import AssignmentCompletedByTab from "@/components/assignments/AssignmentCompletedByTab";


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
            </Group>
            <Tabs defaultValue="task">
                <Tabs.List>
                    <Tabs.Tab value="task">Task</Tabs.Tab>
                    {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && assignment.file_structure !== null && assignment.language !== AssignmentLanguage.QuestionBased && (
                        <Tabs.Tab value="codeTests">Code Tests</Tabs.Tab>
                    )}
                    {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
                        <>
                            <Tabs.Tab value="solutions">Solutions</Tabs.Tab>
                            <Tabs.Tab value="completedBy">Completed by</Tabs.Tab>
                        </>
                    )}
                </Tabs.List>
                <Tabs.Panel mt={20} value="task">
                    <AssignmentDetailsTaskTab assignment={Object.assign({}, assignment)} />
                </Tabs.Panel>
                {assignment.file_structure !== null && isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) &&  (
                    <Tabs.Panel value="codeTests" mt={20}>
                        <FileStructureDisplay
                            structure={Object.assign({}, assignment.file_structure)}
                            groupId={groupId}
                            assignmentId={assignmentId}
                        />
                    </Tabs.Panel>
                )}
                {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
                    <>
                        <Tabs.Panel value="solutions">
                            <AssignmentSolutionsTab assignmentId={assignmentId} />
                        </Tabs.Panel>
                        <Tabs.Panel value="completedBy">
                            <AssignmentCompletedByTab completedBy={assignment.completed_by} />
                        </Tabs.Panel>
                    </>
                )}
            </Tabs>

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
