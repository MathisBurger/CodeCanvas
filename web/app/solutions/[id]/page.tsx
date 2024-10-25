"use client";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { Badge, Button, Container, Group, Tabs, Title } from "@mantine/core";
import useClientQuery from "@/hooks/useClientQuery";
import { AssignmentLanguage, Solution } from "@/service/types/tasky";
import CentralLoading from "@/components/CentralLoading";
import JobResultDisplay from "@/components/JobResultDisplay";
import useCurrentUser from "@/hooks/useCurrentUser";
import {useEffect, useState} from "react";
import { isGranted } from "@/service/auth";
import { UserRoles } from "@/service/types/usernator";
import ExecutorUIDisplay from "@/components/solution/ExecutorUIDisplay";
import SolutionBadge from "@/components/solution/SolutionBadge";
import NavigateBack from "@/components/NavigateBack";
import FileStructureDisplay from "@/components/FileStructureDisplay";
import QuestionAnswersDisplay from "@/components/solution/questions/QuestionAnswersDisplay";
import {useSpotlightStage2} from "@/hooks/spotlight/stage2";

// Every 30s
const REFETCH_INTERVAL = 1000 * 30;

const SolutionDetailsPage = ({ params }: { params: { id: string } }) => {
  const id = parseInt(`${params.id}`, 10);
  const api = useApiServiceClient();
  const { user } = useCurrentUser();
  const [executorModalOpen, setExecutorModalOpen] = useState(false);
  const [solution, refetch] = useClientQuery<Solution>(() =>
    api.getSolution(id),
  );

  const {addSolution} = useSpotlightStage2();
  useEffect(() => {
      if (solution) {
        addSolution(solution);
      }
  }, [addSolution, solution]);

  useEffect(() => {
    const fetcher = async () => {
      if (solution?.job && solution.job.execution.length > 0) {
        const exec = solution.job.execution[0];
        if (exec.error === null && exec.result === null && exec.state === "RUNNING") {
          setTimeout(() => {
            refetch();
            fetcher();
          }, REFETCH_INTERVAL);
        }
      }
    };
    fetcher();
  }, [solution, refetch]);

  console.log(solution);
  const approve = async () => {
    await api.approveSolution(id);
    refetch();
  };

  const reject = async () => {
    await api.rejectSolution(id);
    refetch();
  };

  if (isNaN(id)) {
    return (
      <Container fluid>
        <Title>Invalid Solution ID</Title>
      </Container>
    );
  }

  if (solution === null) {
    return <CentralLoading />;
  }

  return (
    <Container fluid>
      <NavigateBack />
      <Group>
        <Title>
          {solution.assignment.title} - {solution.id}
        </Title>
        <Badge color="indigo">{solution.submitter.username}</Badge>
        <SolutionBadge
          status={solution.approval_status}
          job={solution.job ?? undefined}
        />
        {isGranted(user, [UserRoles.Admin]) && (
          <Button onClick={() => setExecutorModalOpen(true)}>
            Executor UI
          </Button>
        )}
        {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) &&
          [null, "SUCCESSFUL", "FAILED", "PENDING"].indexOf(
            solution.approval_status,
          ) > -1 && (
            <>
              <Button color="lime" onClick={approve}>
                Approve
              </Button>
              <Button color="red" onClick={reject}>
                Reject
              </Button>
            </>
          )}
      </Group>
      <Tabs
        mt={20}
        defaultValue={
          solution.assignment.language === AssignmentLanguage.QuestionBased
            ? "answers"
            : "log"
        }
      >
        <Tabs.List>
          {solution.assignment.language === AssignmentLanguage.QuestionBased ? (
            <Tabs.Tab value="answers">Answers</Tabs.Tab>
          ) : (
            <>
              <Tabs.Tab value="log">Log</Tabs.Tab>
              <Tabs.Tab value="code">Code</Tabs.Tab>
            </>
          )}
        </Tabs.List>

        {solution.assignment.language === AssignmentLanguage.QuestionBased ? (
          <Tabs.Panel value="answers" mt={10}>
            <QuestionAnswersDisplay answers={solution.question_results} />
          </Tabs.Panel>
        ) : (
          <>
            <Tabs.Panel value="log" mt={10}>
              {solution.job !== null && (
                <JobResultDisplay job={solution.job!} />
              )}
            </Tabs.Panel>
            <Tabs.Panel value="code" mt={10}>
              {solution.file_structure !== null && (
                <FileStructureDisplay
                  structure={solution.file_structure}
                  assignmentId={solution.assignment.id}
                  solutionId={solution.id}
                />
              )}
            </Tabs.Panel>
          </>
        )}
      </Tabs>
      {executorModalOpen &&
        solution.job !== undefined &&
        solution.job !== null && (
          <ExecutorUIDisplay
            jobId={solution.job?.id}
            onClose={() => setExecutorModalOpen(false)}
          />
        )}
    </Container>
  );
};

export default SolutionDetailsPage;
