import { Assignment, AssignmentLanguage } from "@/service/types/tasky";
import { Button, Group, Modal, Select, TextInput } from "@mantine/core";
import { DateTimePicker } from "@mantine/dates";
import { useForm } from "@mantine/form";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import RichTextInput from "@/components/form/RichTextInput";
import { notifications } from "@mantine/notifications";
import {useTranslation} from "react-i18next";

interface CreateAssignmentModalProps {
  groupId: number;
  onClose: () => void;
  refetch: () => void;
  action: "create" | "update";
  assignment?: Assignment;
}

const CreateOrUpdateAssignmentModal = ({
  groupId,
  onClose,
  refetch,
  assignment,
  action,
}: CreateAssignmentModalProps) => {
  const api = useApiServiceClient();
  const {t} = useTranslation(['common', 'assignment']);

  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      title: assignment?.title ?? "title",
      due_date: assignment?.due_date ? new Date(assignment.due_date) : null,
      description: assignment?.description ?? "",
      language: assignment?.language ?? AssignmentLanguage.QuestionBased,
    },
    validate: {
      title: (v) => (v.trim() === "" ? t('assignment:errors.empty-title') : null),
      due_date: (v) =>
        v ? (new Date(v).getTime() <= new Date().getTime()
          ? t('errors.future-due-date')
          : null) : null
    },
  });

  const onSubmit = form.onSubmit(async (values) => {
    try {
      if (action === "create") {
        const res = await api.createAssignment(
          groupId,
          values.title,
          values.due_date,
          values.description,
          values.language,
        );
        notifications.show({
          message: `${t('messages.successfully-created-assignment')} ${res.title}`,
          color: "green",
        });
      }
      if (action === "update") {
        const res = await api.updateAssignment(
          groupId,
          assignment?.id ?? -1,
          values.title,
          values.due_date,
          values.description,
        );
        notifications.show({
          message: `${t('messages.successfully-updated-assignment')} ${res.title}`,
          color: "green",
        });
      }
      refetch();
      onClose();
    } catch (e) {
      notifications.show({
        message: t('assignment:errors.create-or-update'),
        color: "red",
      });
    }
  });

  return (
    <Modal
      opened
      onClose={onClose}
      title={action === "create" ? t('titles.create-assignment') : t('titles.update-assignment')}
      size="xl"
    >
      <form onSubmit={onSubmit}>
        <TextInput
          label={t('assignment:fields.title')}
          withAsterisk
          key={form.key("title")}
          {...form.getInputProps("title")}
        />
        <DateTimePicker
          label={t('assignment:fields.due-date')}
          clearable
          mt={10}
          mb={10}
          key={form.key("due_date")}
          {...form.getInputProps("due_date")}
        />
        <RichTextInput
          content={
            assignment?.description ?? form.getInputProps("description").value
          }
          setContent={form.getInputProps("description").onChange}
        />
        <Select
          label={t('assignment:fields.language')}
          withAsterisk
          key={form.key("language")}
          data={Object.entries(AssignmentLanguage).map((e) => e[1])}
          {...form.getInputProps("language")}
          disabled={action === "update"}
        />
        <Group mt={10}>
          <Button type="submit">
            {action === "create" ? t('actions.create') : t('actions.update')}
          </Button>
          <Button onClick={onClose} color="gray">
            {t('actions.cancel')}
          </Button>
        </Group>
      </form>
    </Modal>
  );
};

export default CreateOrUpdateAssignmentModal;
