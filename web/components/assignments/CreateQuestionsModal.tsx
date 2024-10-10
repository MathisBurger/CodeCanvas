"use client";
import { Button, Divider, Group, Modal, Stack } from "@mantine/core";
import { QuestionCatalogueElement } from "@/service/types/tasky";
import QuestionInput from "@/components/assignments/questions/QuestionInput";
import { notifications } from "@mantine/notifications";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { useCallback, useState } from "react";

interface CreateQuestionsModalProps {
  onClose: () => void;
  groupId: number;
  assignmentId: number;
  refetch: () => void;
}

const CreateQuestionsModal = ({
  onClose,
  groupId,
  assignmentId,
  refetch,
}: CreateQuestionsModalProps) => {
  const api = useApiServiceClient();
  const [questions, setQuestions] = useState<QuestionCatalogueElement[]>([]);

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
          title: "Error",
          message: "Please fill all question fields",
          color: "red",
        });
        return;
      }
    }
    try {
      await api.createQuestionCatalogue(groupId, assignmentId, questions);
      refetch();
      onClose();
    } catch (e: any) {
      notifications.show({
        title: "Error",
        message: e?.message ?? "Failed to create question",
        color: "red",
      });
    }
  };

  return (
    <Modal
      opened={true}
      onClose={onClose}
      title="Create new questions"
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
            New question
          </Button>
        </Group>
      </Stack>
      <Divider mt={10} />
      <Group mt={10}>
        <Button type="submit" onClick={onSubmit}>
          Create questions
        </Button>
        <Button onClick={onClose} color="gray">
          Cancel
        </Button>
      </Group>
    </Modal>
  );
};

export default CreateQuestionsModal;
