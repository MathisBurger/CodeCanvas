import {ValidatedQuestionSolution} from "@/service/types/tasky";
import {Paper, Stack, Text} from "@mantine/core";

interface QuestionAnswersDisplayProps {
    answers?: ValidatedQuestionSolution[]|null;
}

const QuestionAnswersDisplay = ({answers}: QuestionAnswersDisplayProps) => {

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
        )
    }
    return null;
}

export default QuestionAnswersDisplay;
