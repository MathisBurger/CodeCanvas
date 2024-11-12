import { Dropzone, FileWithPath } from "@mantine/dropzone";
import { Group, Paper, rem, SimpleGrid, Text } from "@mantine/core";
import { IconFile, IconUpload, IconX } from "@tabler/icons-react";
import { notifications } from "@mantine/notifications";
import { useTranslation } from "react-i18next";

const ALLOWED_TEXT_EXTENSIONS = [
  ".java",
  ".go",
  ".kt",
  ".xml",
  ".md",
  ".gradle",
  ".properties",
  "json",
  ".pem",
  ".yml",
  ".sql",
  ".mod",
];

interface InternalDropzoneProps {
  files: FileWithPath[];
  setFiles: (files: FileWithPath[]) => void;
}

const InternalDropzone = ({ files, setFiles }: InternalDropzoneProps) => {
  const { t } = useTranslation("common");

  return (
    <>
      <Dropzone
        onDrop={(f) => setFiles([...files, ...f])}
        onReject={(f) =>
          notifications.show({
            title: t("messages.rejected-files"),
            color: "red",
            message: `${t("messages.rejected-files")}: ${f.map((file) => file.file.name).join(", ")}`,
          })
        }
        maxSize={10 * 1024 ** 2}
        accept={{ "text/*": ALLOWED_TEXT_EXTENSIONS }}
      >
        <Group
          justify="center"
          gap="xl"
          mih={220}
          style={{ pointerEvents: "none" }}
        >
          <Dropzone.Accept>
            <IconUpload
              style={{
                width: rem(52),
                height: rem(52),
                color: "var(--mantine-color-blue-6)",
              }}
              stroke={1.5}
            />
          </Dropzone.Accept>
          <Dropzone.Reject>
            <IconX
              style={{
                width: rem(52),
                height: rem(52),
                color: "var(--mantine-color-red-6)",
              }}
              stroke={1.5}
            />
          </Dropzone.Reject>
          <Dropzone.Idle>
            <IconFile
              style={{
                width: rem(52),
                height: rem(52),
                color: "var(--mantine-color-dimmed)",
              }}
              stroke={1.5}
            />
          </Dropzone.Idle>

          <div>
            <Text size="xl" inline>
              {t("dropzone.instruction")}
            </Text>
            <Text size="sm" c="dimmed" inline mt={7}>
              {t("dropzone.limit")}
            </Text>
          </div>
        </Group>
      </Dropzone>
      <SimpleGrid cols={{ base: 1, sm: 4 }} mt={10}>
        {files.map((file) => (
          <Paper key={file.name} radius="md" p="sm" withBorder>
            <Text style={{ overflow: "hidden" }}>{file.name}</Text>
          </Paper>
        ))}
      </SimpleGrid>
    </>
  );
};

export default InternalDropzone;
