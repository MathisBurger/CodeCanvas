"use client";
import {useEffect, useState} from "react";
import FileStructure, { FileStructureTree } from "@/components/FileStructure";
import { Button, Group, Modal, Select, Title, TextInput } from "@mantine/core";
import { useSetState } from "@mantine/hooks";
import InternalDropzone from "@/components/InternalDropzone";
import { FileWithPath } from "@mantine/dropzone";
import { notifications } from "@mantine/notifications";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { useForm } from "@mantine/form";
import {Assignment, RunnerConfig} from "@/service/types/tasky";
import {useTranslation} from "react-i18next";

interface AssignmentCreateOrUpdateCodeTestModalProps {
  onClose: () => void;
  groupId: number;
  assignment: Assignment;
  refetch: () => void;
}

const cpuOptions = [".5", "1"];
const memoryOptions = ["50m", "100m", "200m", "300m", "500m"];
const timeoutOptions = ["20s", "60s", "120s", "180s", "240s", "300s"];

const AssignmentCreateOrUpdateCodeTestModal = ({
  onClose,
  groupId,
  assignment,
  refetch,
}: AssignmentCreateOrUpdateCodeTestModalProps) => {
  const [fileStructure, setFileStructure] = useSetState<FileStructureTree>({
    folders: [],
    files: [],
    current_folder_name: null,
  });
  const [files, setFiles] = useState<FileWithPath[]>([]);
  const {t} = useTranslation(['assignment', 'common']);

  useEffect(() => {
    if (assignment.file_structure) {
      setFileStructure(assignment.file_structure);
    }
  }, [assignment])


  const form = useForm({
    initialValues: {
      runner_cpu: assignment.runner_cpu ?? cpuOptions[0],
      runner_memory: assignment.runner_memory ?? memoryOptions[0],
      runner_timeout: assignment.runner_timeout ?? timeoutOptions[0],
      runner_cmd: assignment.runner_cmd ?? 'echo "Hello World!"',
    },
    validate: {
      runner_cpu: (v) =>
        cpuOptions.indexOf(v) === -1 ? t('errors.invalid-cpu') : null,
      runner_memory: (v) =>
        memoryOptions.indexOf(v) === -1 ? t('errors.invalid-memory') : null,
      runner_timeout: (v) =>
        timeoutOptions.indexOf(v) === -1 ? t('errors.invalid-timeout') : null,
      runner_cmd: (v) =>
        v.trim() === "" ? t('errors.empty-cmd') : null,
    },
  });

  const api = useApiServiceClient();

  const submit = form.onSubmit(async (values) => {
    try {
      await api.createOrUpdateCodeTests(groupId, assignment.id, fileStructure, files, {
        ...values,
      } as RunnerConfig);
      refetch();
      onClose();
    } catch (e: any) {
      notifications.show({
        message: e?.message ?? t('errors.code-test-creation-failed'),
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
        <Title order={3}>{t('runner-configuration')}</Title>
        <Select
          key={form.key("runner_cpu")}
          label={t('fields.cpu')}
          {...form.getInputProps("runner_cpu")}
          data={cpuOptions}
        />
        <Select
          key={form.key("runner_memory")}
          label={t('fields.memory')}
          {...form.getInputProps("runner_memory")}
          data={memoryOptions}
        />
        <Select
          key={form.key("runner_timeout")}
          label={t('fields.timeout')}
          {...form.getInputProps("runner_timeout")}
          data={timeoutOptions}
        />
        <TextInput
          key={form.key("runner_cmd")}
          label={t('fields.cmd')}
          {...form.getInputProps("runner_cmd")}
        />
        <Group mt={10}>
          <Button type="submit">{t('common:actions.save')}</Button>
          <Button onClick={onClose} color="gray">
            {t('common:actions.cancel')}
          </Button>
        </Group>
      </form>
    </Modal>
  );
};

export default AssignmentCreateOrUpdateCodeTestModal;
