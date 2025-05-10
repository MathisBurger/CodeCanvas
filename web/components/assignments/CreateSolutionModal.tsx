import { Button, Group, List, Modal, Paper } from "@mantine/core";
import { Assignment } from "@/service/types/tasky";
import { FormEvent, useMemo, useState } from "react";
import { FileWithPath } from "@mantine/dropzone";
import { extractFilesFromFileStructure } from "@/utils/FileStructure";
import InternalDropzone from "@/components/InternalDropzone";
import { notifications, showNotification } from "@mantine/notifications";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { useRouter } from "next/navigation";
import { useTranslation } from "react-i18next";

interface CreateTaskCodeModalProps {
  onClose: () => void;
  assignment: Assignment;
}

const CreateSolutionModal = ({
  onClose,
  assignment,
}: CreateTaskCodeModalProps) => {
  const [files, setFiles] = useState<FileWithPath[]>([]);
  const api = useApiServiceClient();
  const router = useRouter();
  const { t } = useTranslation(["common"]);
  const { t: t2 } = useTranslation(["assignment"]);

  const requiredFiles = useMemo<string[]>(
    () =>
      assignment.file_structure !== null
        ? extractFilesFromFileStructure(assignment.file_structure)
        : [],
    [assignment.file_structure],
  );

  const missingFiles = useMemo<string[]>(() => {
    const uploaded = files.map((f) => f.name);
    return requiredFiles.filter((f) => !uploaded.includes(f));
  }, [files, requiredFiles]);

  const submit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (missingFiles.length > 0) {
      notifications.show({
        title: t("messages.error"),
        message: `${t2("errors.missing-files")} ${missingFiles.join(", ")}`,
        color: "red",
      });
      return;
    }
    try {
      const resp = await api.createSolution(assignment.id, files);
      router.push(`/solutions/${resp.id}`);
    } catch (e: any) {
      showNotification({
        title: t("common:messages.error"),
        message: e?.message ?? "",
      });
    }
  };

  return (
    <Modal
      opened
      onClose={onClose}
      title={t2("titles.create-solution")}
      size="lg"
    >
      <form onSubmit={submit}>
        {missingFiles.length > 0 && (
          <Paper withBorder mb={20} p={10}>
            <List>
              {missingFiles.map((f) => (
                <List.Item key={f}>{f}</List.Item>
              ))}
            </List>
          </Paper>
        )}
        <InternalDropzone files={files} setFiles={setFiles} />
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

export default CreateSolutionModal;
