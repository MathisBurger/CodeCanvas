import {Button, Group, Modal, PasswordInput, TextInput} from "@mantine/core";
import {useForm} from "@mantine/form";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {showNotification} from "@mantine/notifications";

interface CreateTutorModalProps {
    onClose: () => void;
    refetch: () => void;
}

const CreateTutorModal = ({onClose, refetch}: CreateTutorModalProps) => {


    const api = useApiServiceClient();
    const form = useForm({
        initialValues: {
            username: '',
            password: ''
        },
        validate: {
            username: (val) => val.trim() == '' ? 'Username should not be empty' : null,
            password: (v) => v === '' ? 'The password should not be empty' : null,
        }
    });

    const submit = form.onSubmit(async (values) => {
        try {
            await api.createTutor(values.username, values.password);
            refetch();
            onClose();
        } catch (e: any) {
            console.error(e);
            showNotification({
                title: 'Error',
                message: e?.message ?? "Failed to create tutor",
            });
        }
    })

    return (
        <Modal opened onClose={onClose} title="Create Tutor">
            <form onSubmit={submit}>
                <TextInput label="Username" key={form.key('username')} {...form.getInputProps('username')} />
                <PasswordInput label="Password" key={form.key('password')} {...form.getInputProps('password')} />
                <Group mt={10}>
                    <Button type="submit">Create tutor</Button>
                    <Button onClick={onClose} color="gray">
                        Cancel
                    </Button>
                </Group>
            </form>
        </Modal>
    );
}

export default CreateTutorModal;
