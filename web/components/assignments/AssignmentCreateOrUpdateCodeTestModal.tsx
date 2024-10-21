"use client";
import { useState } from "react";
import FileStructure, { FileStructureTree } from "@/components/FileStructure";
import { Button, Group, Modal, Select, Title, TextInput } from "@mantine/core";
import { useSetState } from "@mantine/hooks";
import InternalDropzone from "@/components/InternalDropzone";
import { FileWithPath } from "@mantine/dropzone";
import { notifications } from "@mantine/notifications";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { useForm } from "@mantine/form";
import { RunnerConfig } from "@/service/types/tasky";

interface AssignmentCreateOrUpdateCodeTestModalProps {
  onClose: () => void;
  groupId: number;
  assignmentId: number;
  refetch: () => void;
}

const cpuOptions = [".5", "1"];
const memoryOptions = ["50m", "100m", "200m", "300m", "500m"];
const timeoutOptions = ["20s", "60s", "120s", "180s", "240s", "300s"];

const AssignmentCreateOrUpdateCodeTestModal = ({
  onClose,
  groupId,
  assignmentId,
  refetch,
}: AssignmentCreateOrUpdateCodeTestModalProps) => {
  const [fileStructure, setFileStructure] = useSetState<FileStructureTree>({
    folders: [],
    files: [],
    current_folder_name: null,
  });
  const [files, setFiles] = useState<FileWithPath[]>([]);

  const form = useForm({
    initialValues: {
      runner_cpu: cpuOptions[0],
      runner_memory: memoryOptions[0],
      runner_timeout: timeoutOptions[0],
      runner_cmd: 'echo "Hello World!"',
    },
    validate: {
      runner_cpu: (v) =>
        cpuOptions.indexOf(v) === -1 ? "Invalid CPU option" : null,
      runner_memory: (v) =>
        memoryOptions.indexOf(v) === -1 ? "Invalid memory option" : null,
      runner_timeout: (v) =>
        timeoutOptions.indexOf(v) === -1 ? "Invalid timeout option" : null,
      runner_cmd: (v) =>
        v.trim() === "" ? "Please enter a execution cmd" : null,
    },
  });

  const api = useApiServiceClient();

  const submit = form.onSubmit(async (values) => {
    try {
      await api.createCodeTests(groupId, assignmentId, fileStructure, files, {
        ...values,
      } as RunnerConfig);
      refetch();
      onClose();
    } catch (e: any) {
      notifications.show({
        message: e?.message ?? "Failed to create code tests",
        color: "red",
      });
    }
  });

  return (
    <Modal opened={true} onClose={onClose} size="xl">
      <FileStructure
        structure={fileStructure}
        setStructure={setFileStructure}
        editable={true}
      />
      <InternalDropzone files={files} setFiles={setFiles} />
      <form onSubmit={submit}>
        <Title order={3}>Runner configuration</Title>
        <Select
          key={form.key("runner_cpu")}
          label="CPU"
          {...form.getInputProps("runner_cpu")}
          data={cpuOptions}
        />
        <Select
          key={form.key("runner_memory")}
          label="Memory"
          {...form.getInputProps("runner_memory")}
          data={memoryOptions}
        />
        <Select
          key={form.key("runner_timeout")}
          label="Timeout"
          {...form.getInputProps("runner_timeout")}
          data={timeoutOptions}
        />
        <TextInput
          key={form.key("runner_cmd")}
          label="CMD"
          {...form.getInputProps("runner_cmd")}
        />
        <Group mt={10}>
          <Button type="submit">Create tests</Button>
          <Button onClick={onClose} color="gray">
            Cancel
          </Button>
        </Group>
      </form>
    </Modal>
  );
};

export default AssignmentCreateOrUpdateCodeTestModal;
