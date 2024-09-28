import {AssignmentLanguage, Group as TaskyGroup} from "@/service/types/tasky"
import {Button, Group, Modal, Select, Textarea, TextInput} from "@mantine/core";
import {DateTimePicker} from "@mantine/dates";
import {useForm} from "@mantine/form";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useRouter} from "next/navigation";
import RichTextInput from "@/components/form/RichTextInput";
import { notifications } from "@mantine/notifications";



interface CreateAssignmentModalProps {
    group: TaskyGroup;
    onClose: () => void;
    refetch: () => void;
}

const CreateAssignmentModal = ({group, onClose, refetch}: CreateAssignmentModalProps) => {

    const api = useApiServiceClient();
    const router = useRouter();

    const form = useForm({
        mode: 'uncontrolled',
        initialValues: {
            title: 'title',
            due_date: new Date(),
            description: '',
            language: AssignmentLanguage.QuestionBased,
        },
        validate: {
            title: (v) => v.trim() === '' ? 'Title should contain a value' : null,
            description: (v) => v.trim() === '' ? 'Description should contain a value' : null,
            due_date: (v) => new Date(v).getTime() <= new Date().getTime() ? 'Date should be in the future' : null,
        }
    });

    const onSubmit = form.onSubmit(async (values) => {
        try {
            const res = await api.createAssignment(group.id, values.title, values.due_date, values.description, values.language);
            notifications.show({
                message: `Successfully created assignment ${res.title}`,
                color: 'green',
            })
            refetch();
            onClose();
        } catch (e) {
            notifications.show({
                message: `Failed to create assignment`,
                color: 'red',
            })
        }
    });

    return (
        <Modal opened onClose={onClose} title="Create Assignment" size="xl">
            <form onSubmit={onSubmit}>
                <TextInput label="Title" withAsterisk key={form.key('title')} {...form.getInputProps('title')} />
                <DateTimePicker withAsterisk label="Due date" key={form.key('due_date')} {...form.getInputProps('due_date')} />
                <RichTextInput content={form.getInputProps('description').value} setContent={form.getInputProps('description').onChange} />
                <Select label="Language" withAsterisk key={form.key('language')} data={Object.entries(AssignmentLanguage).map(e => e[1])} {...form.getInputProps('language')} />
                <Group mt={10}>
                    <Button type="submit">Create</Button>
                    <Button onClick={onClose} color="gray">Cancel</Button>
                </Group>
            </form>
        </Modal>
    );
}

export default CreateAssignmentModal;
