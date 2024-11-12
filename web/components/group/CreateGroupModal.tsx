import { Button, Group, Modal, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { useRouter } from "next/navigation";
import { notifications } from "@mantine/notifications";
import { useTranslation } from "react-i18next";

interface CreateGroupModalProps {
  onClose: () => void;
}

const CreateGroupModal = ({ onClose }: CreateGroupModalProps) => {
  const { t } = useTranslation("common");

  const form = useForm({
    initialValues: {
      title: "",
    },
    validate: {
      title: (val) => (val.trim() == "" ? t("errors.title-empty") : null),
    },
  });
  const router = useRouter();
  const api = useApiServiceClient();

  const submit = form.onSubmit(async (values) => {
    try {
      const res = await api.createGroup(values.title);
      router.push(`/groups/${res.id}`);
    } catch (e: any) {
      notifications.show({
        title: t("messages.error"),
        message: e?.message ?? t("errors.create-group"),
      });
    }
  });

  return (
    <Modal opened onClose={onClose} title={t("titles.create-group")}>
      <form onSubmit={submit}>
        <TextInput
          label={t("fields.title")}
          key={form.key("title")}
          {...form.getInputProps("title")}
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

export default CreateGroupModal;
