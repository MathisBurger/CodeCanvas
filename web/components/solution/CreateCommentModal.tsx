import { Solution } from "@/service/types/tasky";
import {Button, Divider, Group, Modal, Stack, TextInput} from "@mantine/core";
import {useForm} from "@mantine/form";
import RichTextInput from "@/components/form/RichTextInput";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {notifications} from "@mantine/notifications";


interface CreateCommentModalProps {
    solution: Solution;
    refetch: () => void;
    onClose: () => void;
}

const CreateCommentModal = ({solution, refetch, onClose}: CreateCommentModalProps) => {

    const form = useForm({
        initialValues: {
            title: '',
            content: ''
        }
    });
    const api = useApiServiceClient();

    const onSubmit = form.onSubmit(async (values) => {
        try {
            await api.createCodeComment(solution.id, values.title, values.content);
            refetch();
            onClose();
        } catch (e: any) {
            notifications.show({
                title: 'Error',
                message: e?.message ?? "Failed to create comment",
            })
        }
    });

    return (
        <Modal opened onClose={onClose} title="Create code comment" size="xl">
            <form onSubmit={onSubmit}>
                <Stack gap={10}>
                    <TextInput label="Title" key={form.key('title')} {...form.getInputProps('title')} />
                    <RichTextInput
                        key={form.key('content')}
                        content={form.getInputProps('content').value}
                        setContent={form.getInputProps('content').onChange}
                    />
                </Stack>
                <Divider mt={10} />
                <Group mt={10}>
                    <Button type="submit">
                        Create questions
                    </Button>
                    <Button onClick={onClose} color="gray">
                        Cancel
                    </Button>
                </Group>
            </form>
        </Modal>
    );
}

export default CreateCommentModal;
