'use client';
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {Badge, Button, Container, Group, Tabs, Title} from "@mantine/core";
import useClientQuery from "@/hooks/useClientQuery";
import {Solution} from "@/service/types/tasky";
import CentralLoading from "@/components/CentralLoading";
import FileStructureDisplay from "@/components/FileStructureDisplay";
import JobResultDisplay from "@/components/JobResultDisplay";
import useCurrentUser from "@/hooks/useCurrentUser";
import {useState} from "react";
import {isGranted} from "@/service/auth";
import {UserRoles} from "@/service/types/usernator";
import ExecutorUIDisplay from "@/components/solution/ExecutorUIDisplay";
import SolutionBadge from "@/components/solution/SolutionBadge";
import NavigateBack from "@/components/NavigateBack";

const SolutionDetailsPage = ({params}: {params: {id: string}}) => {

    const id = parseInt(`${params.id}`, 10);
    const api = useApiServiceClient();
    const {user} = useCurrentUser();
    const [executorModalOpen, setExecutorModalOpen] = useState(false);
    const [solution, refetch] = useClientQuery<Solution>(() => api.getSolution(id));
    console.log(solution)
    const approve = async () => {
        await api.approveSolution(id);
        refetch();
    }

    const reject = async () => {
        await api.rejectSolution(id);
        refetch();
    }

    if (isNaN(id)) {
        return (
            <Container fluid>
                <Title>Invalid Solution ID</Title>
            </Container>
        )
    }

    if (solution === null) {
        return (
            <CentralLoading />
        );
    }

    return (
        <Container fluid>
            <NavigateBack />
            <Group>
                <Title>{solution.assignment.title} - {solution.id}</Title>
                <Badge color="indigo">{solution.submitter.username}</Badge>
                <SolutionBadge status={solution.approval_status} job={solution.job ?? undefined} />
                {isGranted(user, [UserRoles.Admin]) && (
                    <Button onClick={() => setExecutorModalOpen(true)}>Executor UI</Button>
                )}
                {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (solution.approval_status === null || solution.approval_status === "PENDING") && (
                    <>
                        <Button color="lime" onClick={approve}>Approve</Button>
                        <Button color="red" onClick={reject}>Reject</Button>
                    </>
                )}
            </Group>
            <Tabs mt={20} defaultValue="log">
                <Tabs.List>
                    <Tabs.Tab value="log">Log</Tabs.Tab>
                    <Tabs.Tab value="code">Code</Tabs.Tab>
                </Tabs.List>
                <Tabs.Panel value="log" mt={10}>
                    {solution.job !== null && (
                        <JobResultDisplay job={solution.job!} />
                    )}
                </Tabs.Panel>
                <Tabs.Panel value="code" mt={10}>
                    {solution.file_structure !== undefined && (
                        <FileStructureDisplay
                            structure={solution.file_structure}
                            assignmentId={solution.assignment.id}
                            solutionId={solution.id}
                        />
                    )}
                </Tabs.Panel>
            </Tabs>
            {executorModalOpen && solution.job !== undefined && solution.job !== null && (
                <ExecutorUIDisplay jobId={solution.job?.id} onClose={() => setExecutorModalOpen(false)} />
            )}
        </Container>
    )
}

export default SolutionDetailsPage;
