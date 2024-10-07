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

const getBadge = (status?: string): JSX.Element => {
    switch (status) {
        case "APPROVED":
            return <Badge color="green">{status}</Badge>;
        case "REJECTED":
            return <Badge color="red">{status}</Badge>;
        default:
            return <Badge color="yellow">{status ?? "PENDING"}</Badge>;
    }
}

const SolutionDetailsPage = ({params}: {params: {id: string}}) => {

    const id = parseInt(`${params.id}`, 10);
    const api = useApiServiceClient();
    const {user} = useCurrentUser();
    const [executorModalOpen, setExecutorModalOpen] = useState(false);
    const [solution] = useClientQuery<Solution>(() => api.getSolution(id));
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
            <Group>
                <Title>{solution.assignment.title} - {solution.id}</Title>
                <Badge color="indigo">{solution.submitter.username}</Badge>
                {getBadge(solution.approval_status)}
                {isGranted(user, [UserRoles.Admin]) && (
                    <Button onClick={() => setExecutorModalOpen(true)}>Executor UI</Button>
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