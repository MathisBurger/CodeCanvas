import { Button, Group, Modal, PasswordInput, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { showNotification } from "@mantine/notifications";
import { useTranslation } from "react-i18next";

interface CreateTutorModalProps {
  onClose: () => void;
  refetch: () => void;
}

const CreateTutorModal = ({ onClose, refetch }: CreateTutorModalProps) => {
  const { t } = useTranslation(["common", "assignment"]);

  const api = useApiServiceClient();
  const form = useForm({
    initialValues: {
      username: "",
      password: "",
    },
    validate: {
      username: (val) => (val.trim() == "" ? t("errors.username-empty") : null),
      password: (v) => (v === "" ? t("errors.password-empty") : null),
    },
  });

  const submit = form.onSubmit(async (values) => {
    try {
      await api.createTutor(values.username, values.password);
      refetch();
      onClose();
    } catch (e: any) {
      console.error(e);
      showNotification({
        title: "Error",
        message: e?.message ?? t("errors.create-tutor"),
      });
    }
  });

  return (
    <Modal opened onClose={onClose} title={t("titles.create-tutor")}>
      <form onSubmit={submit}>
        <TextInput
          label={t("fields.username")}
          key={form.key("username")}
          {...form.getInputProps("username")}
        />
        <PasswordInput
          label={t("fields.password")}
          key={form.key("password")}
          {...form.getInputProps("password")}
        />
        <Group mt={10}>
          <Button type="submit">{t("actions.create")}</Button>
          <Button onClick={onClose} color="gray">
            {t("actions.cancel")}
          </Button>
        </Group>
      </form>
    </Modal>
  );
};

export default CreateTutorModal;
