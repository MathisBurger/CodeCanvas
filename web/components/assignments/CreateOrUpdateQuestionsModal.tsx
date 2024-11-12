"use client";
import { Button, Divider, Group, Modal, Stack } from "@mantine/core";
import { Assignment, QuestionCatalogueElement } from "@/service/types/tasky";
import QuestionInput from "@/components/assignments/questions/QuestionInput";
import { notifications } from "@mantine/notifications";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

interface CreateQuestionsModalProps {
  onClose: () => void;
  groupId: number;
  assignment: Assignment;
  refetch: () => void;
}

const CreateOrUpdateQuestionsModal = ({
  onClose,
  groupId,
  assignment,
  refetch,
}: CreateQuestionsModalProps) => {
  const api = useApiServiceClient();
  const [questions, setQuestions] = useState<QuestionCatalogueElement[]>([]);
  const { t } = useTranslation(["common", "assignment"]);

  useEffect(() => {
    if (assignment.question_catalogue?.catalogue) {
      setQuestions(Object.values(assignment.question_catalogue.catalogue));
    }
  }, [assignment]);

  const updateQuestion = useCallback(
    (index: number, value: QuestionCatalogueElement) => {
      const copy = [...questions];
      console.log(questions);
      copy[index] = value;
      console.log(value);
      console.log(copy);
      setQuestions(copy);
    },
    [questions],
  );

  const addNew = () => {
    setQuestions([
      ...questions,
      { answer: "", answer_type: null, question: "" },
    ]);
  };

  const removeAt = (index: number) => {
    setQuestions(questions.filter((_, i) => i !== index));
  };

  const isEmpty = (v: any) => v === null || v === undefined || v === "";

  const onSubmit = async () => {
    for (const question of questions) {
      if (
        isEmpty(question.question) ||
        isEmpty(question.answer) ||
        isEmpty(question.answer_type)
      ) {
        console.log(question);
        notifications.show({
          title: t("messages.error"),
          message: t("errors.all-required-fields"),
          color: "red",
        });
        return;
      }
    }
    try {
      await api.createOrUpdateQuestionCatalogue(
        groupId,
        assignment.id,
        questions,
      );
      refetch();
      onClose();
    } catch (e: any) {
      notifications.show({
        title: t("messages.error"),
        message: e?.message ?? t("assignment:errors.failed-question-cu"),
        color: "red",
      });
    }
  };

  return (
    <Modal
      opened={true}
      onClose={onClose}
      title={t("assignment:titles.question-catalogue")}
      size="xl"
    >
      <Stack>
        {questions.map((question, index) => (
          <QuestionInput
            value={question}
            setValue={(v) => updateQuestion(index, v)}
            key={`index_${index}`}
            remove={() => removeAt(index)}
          />
        ))}
        <Group justify="flex-end">
          <Button onClick={addNew} w={150}>
            {t("assignment:actions.new-question")}
          </Button>
        </Group>
      </Stack>
      <Divider mt={10} />
      <Group mt={10}>
        <Button type="submit" onClick={onSubmit}>
          {t("actions.save")}
        </Button>
        <Button onClick={onClose} color="gray">
          {t("actions.cancel")}
        </Button>
      </Group>
    </Modal>
  );
};

export default CreateOrUpdateQuestionsModal;
