import { Assignment, AssignmentLanguage } from "@/service/types/tasky";
import { Button, Title } from "@mantine/core";
import RichTextDisplay from "@/components/display/RichTextDisplay";
import FileStructure from "@/components/FileStructure";
import { useMemo, useState } from "react";
import { isGranted } from "@/service/auth";
import useCurrentUser from "@/hooks/useCurrentUser";
import { UserRoles } from "@/service/types/usernator";
import CreateTaskCodeModal from "@/components/assignments/CreateSolutionModal";
import AnswerQuestionsModal from "@/components/assignments/questions/AnswerQuestionsModal";
import { useTranslation } from "react-i18next";

interface AssignmentDetailsTaskProps {
  assignment: Assignment | null;
}

const AssignmentDetailsTaskTab = ({
  assignment,
}: AssignmentDetailsTaskProps) => {
  const { user } = useCurrentUser();
  const [createSolutionModalOpen, setCreateSolutionModalOpen] = useState(false);

  const { t } = useTranslation("assignment");

  const assignmentCompleted = useMemo<boolean>(() => {
    return (
      (assignment?.completed_by ?? [])
        .map((u) => u.id)
        .indexOf(user?.id ?? -1) > -1
    );
  }, [user, assignment]);

  return (
    <>
      <Title order={3}>{t("task")}</Title>
      <RichTextDisplay
        content={assignment?.description ?? ""}
        fullSize={true}
      />

      {assignment !== null && assignment.file_structure !== null && (
        <>
          <Title order={3} mb={10}>
            {t("required-files")}
          </Title>
          <FileStructure
            structure={assignment.file_structure}
            editable={false}
            displayMode="task"
          />
        </>
      )}
      {!assignmentCompleted && isGranted(user, [UserRoles.Student]) && (
        <Button
          color="lime"
          mt={20}
          onClick={() => setCreateSolutionModalOpen(true)}
        >
          {t("actions.submit-solution")}
        </Button>
      )}
      {createSolutionModalOpen &&
        assignment !== null &&
        assignment.language !== AssignmentLanguage.QuestionBased && (
          <CreateTaskCodeModal
            onClose={() => setCreateSolutionModalOpen(false)}
            assignment={assignment}
          />
        )}
      {createSolutionModalOpen &&
        assignment !== null &&
        assignment.language === AssignmentLanguage.QuestionBased &&
        assignment.question_catalogue !== null && (
          <AnswerQuestionsModal
            assignmentId={assignment.id}
            onClose={() => setCreateSolutionModalOpen(false)}
            catalogue={assignment.question_catalogue}
          />
        )}
    </>
  );
};

export default AssignmentDetailsTaskTab;
