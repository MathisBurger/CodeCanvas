import { Assignment, AssignmentLanguage } from "@/service/types/tasky";
import { Button, Group, Modal, Select, TextInput } from "@mantine/core";
import { DateTimePicker } from "@mantine/dates";
import { useForm } from "@mantine/form";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import RichTextInput from "@/components/form/RichTextInput";
import { notifications } from "@mantine/notifications";

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

  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      title: assignment?.title ?? "title",
      due_date: assignment?.due_date ? new Date(assignment.due_date) : null,
      description: assignment?.description ?? "",
      language: assignment?.language ?? AssignmentLanguage.QuestionBased,
    },
    validate: {
      title: (v) => (v.trim() === "" ? "Title should contain a value" : null),
      due_date: (v) =>
        v ? (new Date(v).getTime() <= new Date().getTime()
          ? "Date should be in the future"
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
          message: `Successfully created assignment ${res.title}`,
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
          message: `Successfully updated assignment ${res.title}`,
          color: "green",
        });
      }
      refetch();
      onClose();
    } catch (e) {
      notifications.show({
        message: `Failed to create or update assignment`,
        color: "red",
      });
    }
  });

  return (
    <Modal
      opened
      onClose={onClose}
      title={action === "create" ? "Create Assignment" : "Update Assignment"}
      size="xl"
    >
      <form onSubmit={onSubmit}>
        <TextInput
          label="Title"
          withAsterisk
          key={form.key("title")}
          {...form.getInputProps("title")}
        />
        <DateTimePicker
          label="Due date"
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
          label="Language"
          withAsterisk
          key={form.key("language")}
          data={Object.entries(AssignmentLanguage).map((e) => e[1])}
          {...form.getInputProps("language")}
          disabled={action === "update"}
        />
        <Group mt={10}>
          <Button type="submit">
            {action === "create" ? "Create" : "Update"}
          </Button>
          <Button onClick={onClose} color="gray">
            Cancel
          </Button>
        </Group>
      </form>
    </Modal>
  );
};

export default CreateOrUpdateAssignmentModal;
