import {Button, Group, Modal, Stack, TextInput} from "@mantine/core";
import {useForm} from "@mantine/form";
import {useTranslation} from "react-i18next";
import RichTextInput from "@/components/form/RichTextInput";
import {DateTimePicker} from "@mantine/dates";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {showNotification} from "@mantine/notifications";


interface CreateSystemWideNotificationModalProps {
    onClose: () => void;
    refetch: () => void;
}

const CreateSystemWideNotificationModal = ({onClose, refetch}: CreateSystemWideNotificationModalProps) => {

    const {t} = useTranslation();
    const api = useApiServiceClient();

    const form = useForm({
        initialValues: {
            title: '',
            content: '',
            show_until: new Date(),
        },
        validate: {
            title: (value) => value.trim() === '' ? t('errors.title-empty') : null,
        }
    });

    const submitCallback = form.onSubmit(async (values) => {
        try {
            await api.createSystemWideNotification(values.title, values.content, values.show_until);
            showNotification({
                title: t('common:messages.success'),
                message: t('common:messages.successfully-created-notification')
            });
            refetch();
            onClose();
        } catch (e: any) {
            showNotification({
                title: t('common:messages.error'),
                message: e?.message ?? '',
            })
        }
    });

    return (
        <Modal opened onClose={onClose} title={t('common:actions.create-notification')} size="lg">
            <form onSubmit={submitCallback}>
                <Stack gap={3}>
                    <TextInput label={t('fields.title')} key={form.key('title')} {...form.getInputProps('title')} />
                    <DateTimePicker
                        label={t("assignment:fields.due-date")}
                        key={form.key("show_until")}
                        {...form.getInputProps("show_until")}
                    />
                    <RichTextInput content={form.getInputProps('content').value} setContent={form.getInputProps('content').onChange} />
                </Stack>
                <Group mt={10}>
                    <Button type="submit">{t("actions.create")}</Button>
                    <Button onClick={onClose} color="gray">
                        {t("actions.cancel")}
                    </Button>
                </Group>
            </form>
        </Modal>
    );
}

export default CreateSystemWideNotificationModal;
