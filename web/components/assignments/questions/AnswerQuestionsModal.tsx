import {AnswerType, QuestionCatalogue, QuestionSolution} from "@/service/types/tasky";
import {Button, Group, Modal, Stack} from "@mantine/core";
import AnswerInput from "@/components/assignments/questions/AnswerInput";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useRouter} from "next/navigation";
import {notifications} from "@mantine/notifications";
import { useSetState } from "@mantine/hooks";


type StateType = {map: Map<string, QuestionSolution>};

interface AnswerQuestionsModalProps {
    onClose: () => void;
    catalogue: QuestionCatalogue;
    assignmentId: number;
}

/**
 * Gets the default answer for a question
 *
 * @param type The answer type
 */
const getDefaultAnswer = (type: AnswerType): any => {
    switch (type) {
        case AnswerType.Number:
            return 0;
        case AnswerType.Boolean:
            return false;
        default:
            return '';
    }
}

/**
 * Initializes all answers
 *
 * @param catalogue The catalogue
 */
const initializeAnswers = (catalogue: QuestionCatalogue): Map<string, QuestionSolution> => {
    const map = new Map<string, QuestionSolution>();
    for (const [hash, question] of Object.entries(catalogue.catalogue)) {
        map.set(hash, {answer: getDefaultAnswer(question.answer_type ?? AnswerType.String)});
    }
    return map;
}


const AnswerQuestionsModal = ({onClose, catalogue, assignmentId}: AnswerQuestionsModalProps) => {

    const [answers, setAnswers] = useSetState<StateType>({map: initializeAnswers(catalogue)});
    const api = useApiServiceClient();
    const router = useRouter();

    const updateSolution = (hash: string, answer: any) => {
        setAnswers({map: answers.map.set(hash, {answer})});
    }

    const onSubmit = async () => {
        try {
            const res = await api.createSolution(assignmentId, [], answers.map);
            router.push(`/solutions/${res.id}`);
        } catch (e: any) {
            notifications.show({
                title: 'Error',
                message: e?.message ?? 'Error creating solution',
            });
        }
    }

    return (
        <Modal opened onClose={onClose} title="Create solution">
            <Stack>
                {Array.from(answers.map.entries()).map(([hash, answer]) => (
                    <AnswerInput
                        value={answer?.answer ?? ''}
                        setValue={(v) => updateSolution(hash, v)}
                        answerType={catalogue?.catalogue[hash]?.answer_type ?? AnswerType.String}
                        question={catalogue.catalogue[hash]?.question ?? ''}
                        key={catalogue.catalogue[hash]?.question ?? ''}
                    />
                ))}
            </Stack>
            <Group mt={10}>
                <Button type="submit" onClick={onSubmit}>Submit solution</Button>
                <Button onClick={onClose} color="gray">Cancel</Button>
            </Group>
        </Modal>
    );
}

export default AnswerQuestionsModal;
