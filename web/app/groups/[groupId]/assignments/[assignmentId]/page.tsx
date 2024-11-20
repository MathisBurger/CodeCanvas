"use client";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import { Badge, Button, Container, Group, Tabs, Title } from "@mantine/core";
import AssignmentDateDisplay from "@/components/assignments/AssignmentDateDisplay";
import NavigateBack from "@/components/NavigateBack";
import useCurrentUser from "@/hooks/useCurrentUser";
import { isGranted } from "@/service/auth";
import { UserRoles } from "@/service/types/usernator";
import { useEffect, useState } from "react";
import CreateOrUpdateAssignmentModal from "@/components/assignments/CreateOrUpdateAssignmentModal";
import CentralLoading from "@/components/CentralLoading";
import AssignmentCreateOrUpdateCodeTestModal from "@/components/assignments/AssignmentCreateOrUpdateCodeTestModal";
import { AssignmentLanguage } from "@/service/types/tasky";
import FileStructureDisplay from "@/components/FileStructureDisplay";
import AssignmentDetailsTaskTab from "@/components/assignments/AssignmentDetailsTaskTab";
import AssignmentSolutionsTab from "@/components/assignments/AssignmentSolutionsTab";
import AssignmentCompletedByTab from "@/components/assignments/AssignmentCompletedByTab";
import CreateOrUpdateQuestionsModal from "@/components/assignments/CreateOrUpdateQuestionsModal";
import QuestionAnswersDisplay from "@/components/solution/questions/QuestionAnswersDisplay";
import { useSpotlightStage2 } from "@/hooks/spotlight/stage2";
import { useTranslation } from "react-i18next";

const AssignmentDetailsPage = ({
  params,
}: {
  params: { groupId: string; assignmentId: string };
}) => {
  const groupId = parseInt(`${params.groupId}`, 10);
  const assignmentId = parseInt(`${params.assignmentId}`, 10);
  const api = useApiServiceClient();
  const { user } = useCurrentUser();
  const [updateModalOpen, setUpdateModalOpen] = useState(false);
  const [fileStructureModalOpen, setFileStructureModalOpen] = useState(false);
  const [questionsModalOpen, setQuestionsModalOpen] = useState(false);
  const [assignment, refetch] = useClientQuery(
    () => api.getAssignmentForGroup(groupId, assignmentId),
    [assignmentId, groupId],
  );
  const { t } = useTranslation(["common", "assignment"]);

  const { addAssignment } = useSpotlightStage2();

  useEffect(() => {
    if (assignment && groupId) {
      addAssignment(assignment, groupId);
    }
  }, [addAssignment, assignment, groupId]);

  if (isNaN(groupId) || isNaN(assignmentId)) {
    return (
      <Container fluid>
        <Title>{t("invalid-group-id")}</Title>
      </Container>
    );
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
        {assignment.completed && (
          <Badge color="green">{t("assignment:messages.completed")}</Badge>
        )}
        <AssignmentDateDisplay date={assignment?.due_date ?? null} />
        {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
          <Button onClick={() => setUpdateModalOpen(true)}>Edit</Button>
        )}
        {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) &&
          assignment.language !== AssignmentLanguage.QuestionBased && (
            <Button onClick={() => setFileStructureModalOpen(true)}>
              {t("assignment:code-tests")}
            </Button>
          )}
        {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) &&
          assignment.language === AssignmentLanguage.QuestionBased && (
            <Button onClick={() => setQuestionsModalOpen(true)}>
              {t("assignment:questions")}
            </Button>
          )}
      </Group>
      <Tabs defaultValue="task">
        <Tabs.List>
          <Tabs.Tab value="task">{t("assignment:task")}</Tabs.Tab>
          {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) &&
            assignment.file_structure !== null &&
            assignment.language !== AssignmentLanguage.QuestionBased && (
              <Tabs.Tab value="codeTests">
                {t("assignment:code-tests")}
              </Tabs.Tab>
            )}
          {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) &&
            assignment.question_catalogue !== null &&
            assignment.language === AssignmentLanguage.QuestionBased && (
              <Tabs.Tab value="questions">{t("assignment:questions")}</Tabs.Tab>
            )}
          {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
            <>
              <Tabs.Tab value="solutions">{t("assignment:solutions")}</Tabs.Tab>
              <Tabs.Tab value="completedBy">
                {t("assignment:completed-by")}
              </Tabs.Tab>
            </>
          )}
        </Tabs.List>
        <Tabs.Panel mt={20} value="task">
          <AssignmentDetailsTaskTab assignment={structuredClone(assignment)} />
        </Tabs.Panel>
        {assignment.file_structure !== null &&
          isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
            <Tabs.Panel value="codeTests" mt={20}>
              <FileStructureDisplay
                structure={structuredClone(assignment.file_structure)}
                groupId={groupId}
                assignmentId={assignmentId}
              />
            </Tabs.Panel>
          )}
        {assignment.question_catalogue !== null &&
          isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
            <Tabs.Panel value="questions" mt={20}>
              <QuestionAnswersDisplay
                questions={assignment.question_catalogue}
              />
            </Tabs.Panel>
          )}
        {isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) && (
          <>
            <Tabs.Panel value="solutions">
              <AssignmentSolutionsTab assignmentId={assignmentId} />
            </Tabs.Panel>
            <Tabs.Panel value="completedBy">
              <AssignmentCompletedByTab groupId={groupId} assignmentId={assignmentId} />
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
      {fileStructureModalOpen && assignment && (
        <AssignmentCreateOrUpdateCodeTestModal
          onClose={() => setFileStructureModalOpen(false)}
          groupId={groupId}
          assignment={structuredClone(assignment)}
          refetch={refetch}
        />
      )}
      {questionsModalOpen && assignment && (
        <CreateOrUpdateQuestionsModal
          groupId={groupId}
          assignment={assignment}
          refetch={refetch}
          onClose={() => setQuestionsModalOpen(false)}
        />
      )}
    </Container>
  );
};

export default AssignmentDetailsPage;
