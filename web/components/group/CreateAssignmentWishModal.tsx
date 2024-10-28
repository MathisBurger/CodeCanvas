import useApiServiceClient from "@/hooks/useApiServiceClient";
import {Button, Group, Modal, Stack, Textarea, TextInput} from "@mantine/core";
import {useForm} from "@mantine/form";
import {showNotification} from "@mantine/notifications";

interface CreateAssignmentWishModalProps {
    onClose: () => void;
    refetch: () => void;
    groupId: number;
}

const CreateAssignmentWishModal = ({onClose, refetch, groupId}: CreateAssignmentWishModalProps) => {

    const api = useApiServiceClient();
    const form = useForm({
        initialValues: {
            title: '',
            description: '',
        },
        validate: {
            title: (val) => val.trim() == '' ? 'Title should not be empty' : null,
            description: (val) => val.trim() == '' ? 'The description should not be empty' : null,
        }
    });

    const submit = form.onSubmit(async (values) => {
        try {
            await api.createAssignmentWish(groupId, values.title, values.description);
            refetch();
            onClose();
        } catch (e: any) {
            showNotification({
                title: 'Error',
                message: e?.message ?? "Error creating assignment wish",
            })
        }
    });

    return (
        <Modal opened onClose={onClose} title="Create Assignment Wish">
            <form onSubmit={submit}>
                <Stack gap={2}>
                    <TextInput label="Title" key={form.key('title')} {...form.getInputProps('title')} />
                    <Textarea label="Description" key={form.key('description')} autosize {...form.getInputProps('description')} />
                    <Group mt={10}>
                        <Button type="submit">Create wish</Button>
                        <Button onClick={onClose} color="gray">
                            Cancel
                        </Button>
                    </Group>
                </Stack>
            </form>
        </Modal>
    );
}

export default CreateAssignmentWishModal;
