import {
  QuestionCatalogue,
  ValidatedQuestionSolution,
} from "@/service/types/tasky";
import { Paper, Stack, Text } from "@mantine/core";

interface QuestionAnswersDisplayProps {
  answers?: ValidatedQuestionSolution[] | null;
  questions?: QuestionCatalogue | null;
}

const QuestionAnswersDisplay = ({
  answers,
  questions,
}: QuestionAnswersDisplayProps) => {
  if (answers) {
    return (
      <Stack gap={20}>
        {answers.map((solution) => (
          <Paper key={solution.question} withBorder radius="md" p="md">
            <Text>{solution.question}</Text>
            <Text c={solution.correct ? "lime" : "red"}>{solution.answer}</Text>
          </Paper>
        ))}
      </Stack>
    );
  }
  if (questions) {
    return (
      <Stack gap={20}>
        {Object.values(questions.catalogue).map((question) => (
          <Paper key={question.question} withBorder radius="md" p="md">
            <Text>{question.question}</Text>
            <Text c="lime">{question.answer}</Text>
          </Paper>
        ))}
      </Stack>
    );
  }
  return null;
};

export default QuestionAnswersDisplay;
