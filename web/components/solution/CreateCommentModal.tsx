import { Solution } from "@/service/types/tasky";
import {Button, Divider, Group, Modal, Stack, TextInput} from "@mantine/core";
import {useForm} from "@mantine/form";
import RichTextInput from "@/components/form/RichTextInput";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {notifications} from "@mantine/notifications";
import {useTranslation} from "react-i18next";


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
    const {t} = useTranslation(['common', 'solution']);

    const onSubmit = form.onSubmit(async (values) => {
        try {
            await api.createCodeComment(solution.id, values.title, values.content);
            refetch();
            onClose();
        } catch (e: any) {
            notifications.show({
                title: t('messages.error'),
                message: e?.message ?? "",
            })
        }
    });

    return (
        <Modal opened onClose={onClose} title={t('titles.create-comment')} size="xl">
            <form onSubmit={onSubmit}>
                <Stack gap={10}>
                    <TextInput label={t('fields.title')} key={form.key('title')} {...form.getInputProps('title')} />
                    <RichTextInput
                        key={form.key('content')}
                        content={form.getInputProps('content').value}
                        setContent={form.getInputProps('content').onChange}
                    />
                </Stack>
                <Divider mt={10} />
                <Group mt={10}>
                    <Button type="submit">
                        {t('actions.create')}
                    </Button>
                    <Button onClick={onClose} color="gray">
                        {t('actions.cancel')}
                    </Button>
                </Group>
            </form>
        </Modal>
    );
}

export default CreateCommentModal;
