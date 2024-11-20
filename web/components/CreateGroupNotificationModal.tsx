import {Button, Group, Modal, Stack, Textarea, TextInput} from "@mantine/core";
import {useTranslation} from "react-i18next";
import {useForm} from "@mantine/form";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {showNotification} from "@mantine/notifications";

interface CreateGroupNotificationModalProps {
    groupId: number;
    onClose: () => void;
}

const CreateGroupNotificationModal = ({groupId, onClose}: CreateGroupNotificationModalProps) => {


    const {t} = useTranslation("common");
    const api = useApiServiceClient();
    const form = useForm({
        initialValues: {
            title: '',
            content: '',
        },
        validate: {
            title: (value) => value.trim() === '' ? t('errors.title-empty') : null,
            content: (value) => value.trim() === '' ? t('errors.body-empty') : null
        }
    });

    const submitCallback = form.onSubmit(async (values) => {
        try {
            await api.createGroupNotification(groupId, values.title, values.content);
            showNotification({
                title: t('common:messages.success'),
                message: t('common:messages.successfully-created-notification')
            });
            onClose();
        } catch (e: any) {
            showNotification({
                title: t('common:messages.error'),
                message: e?.message ?? ''
            });
        }
    });

    return (
      <Modal opened onClose={onClose} title={t('actions.create-notification')}>
          <form onSubmit={submitCallback}>
              <Stack gap={3}>
                  <TextInput label={t('fields.title')} key={form.key('title')} {...form.getInputProps('title')} />
                  <Textarea label={t('fields.description')} key={form.key('content')} {...form.getInputProps('content')} />
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

export default CreateGroupNotificationModal;
