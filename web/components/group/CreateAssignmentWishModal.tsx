import useApiServiceClient from "@/hooks/useApiServiceClient";
import {
  Button,
  Group,
  Modal,
  Stack,
  Textarea,
  TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { showNotification } from "@mantine/notifications";
import { useTranslation } from "react-i18next";

interface CreateAssignmentWishModalProps {
  onClose: () => void;
  refetch: () => void;
  groupId: number;
}

const CreateAssignmentWishModal = ({
  onClose,
  refetch,
  groupId,
}: CreateAssignmentWishModalProps) => {
  const { t } = useTranslation(["common", "assignment"]);
  const api = useApiServiceClient();
  const form = useForm({
    initialValues: {
      title: "",
      description: "",
    },
    validate: {
      title: (val) => (val.trim() == "" ? t("errors.title-empty") : null),
      description: (val) =>
        val.trim() == "" ? t("errors.description-empty") : null,
    },
  });

  const submit = form.onSubmit(async (values) => {
    try {
      await api.createAssignmentWish(groupId, values.title, values.description);
      refetch();
      onClose();
    } catch (e: any) {
      showNotification({
        title: t("messages.error"),
        message: e?.message ?? t("errors.assignment-wish"),
      });
    }
  });

  return (
    <Modal opened onClose={onClose} title={t("titles.create-wish")}>
      <form onSubmit={submit}>
        <Stack gap={2}>
          <TextInput
            label={t("fields.title")}
            key={form.key("title")}
            {...form.getInputProps("title")}
          />
          <Textarea
            label={t("fields.description")}
            key={form.key("description")}
            autosize
            {...form.getInputProps("description")}
          />
          <Group mt={10}>
            <Button type="submit">{t("actions.create")}</Button>
            <Button onClick={onClose} color="gray">
              {t("actions.cancel")}
            </Button>
          </Group>
        </Stack>
      </form>
    </Modal>
  );
};

export default CreateAssignmentWishModal;
