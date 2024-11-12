import useCurrentUser from "@/hooks/useCurrentUser";
import { Group, Paper, Text } from "@mantine/core";
import { useRouter } from "next/navigation";
import { useTranslation } from "react-i18next";

const Footer = () => {
  const router = useRouter();
  const { user } = useCurrentUser();
  const { t } = useTranslation("routes");

  return (
    <Paper m="sm">
      <Group justify="flex-end">
        <Text
          c="dimmed"
          style={{ cursor: "pointer" }}
          onClick={() => router.push("/impress")}
        >
          {t("impress")}
        </Text>
        <Text
          c="dimmed"
          style={{ cursor: "pointer" }}
          onClick={() => router.push("/privacy")}
        >
          {t("privacy")}
        </Text>
        {user && (
          <Text
            c="dimmed"
            style={{ cursor: "pointer" }}
            onClick={() => router.push("/report-bug")}
          >
            {t("report-bug")}
          </Text>
        )}
      </Group>
    </Paper>
  );
};

export default Footer;
