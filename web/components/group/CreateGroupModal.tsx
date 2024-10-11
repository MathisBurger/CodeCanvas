import {Button, Group, Modal, TextInput} from "@mantine/core";
import {useForm} from "@mantine/form";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useRouter} from "next/navigation";
import {notifications} from "@mantine/notifications";

interface CreateGroupModalProps {
    onClose: () => void;
}

const CreateGroupModal = ({onClose}: CreateGroupModalProps) => {

    const form = useForm({
        initialValues: {
            title : ''
        },
        validate: {
            title: (val) => val.trim() == '' ? 'Title should not be empty' : null
        }
    });
    const router = useRouter();
    const api = useApiServiceClient();

    const submit = form.onSubmit(async (values) => {
        try {
            const res = await api.createGroup(values.title);
            router.push(`/groups/${res.id}`);
        } catch (e: any) {
            notifications.show({
                title: 'Error',
                message: e?.message ?? "Error creating group",
            });
        }
    })

    return (
        <Modal opened onClose={onClose} title="Create group">
            <form onSubmit={submit}>
                <TextInput label="Title" key={form.key('title')} {...form.getInputProps('title')} />
                <Group mt={10}>
                    <Button type="submit">Create group</Button>
                    <Button onClick={onClose} color="gray">
                        Cancel
                    </Button>
                </Group>
            </form>
        </Modal>
    );
}

export default CreateGroupModal;
