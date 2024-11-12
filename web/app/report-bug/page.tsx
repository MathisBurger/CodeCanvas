"use client";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {
  Container,
  Stack,
  Textarea,
  TextInput,
  Title,
  Button,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { showNotification } from "@mantine/notifications";
import { useTranslation } from "react-i18next";

const ReportBugPage = () => {
  const api = useApiServiceClient();
  const { t } = useTranslation("common");
  const form = useForm({
    initialValues: {
      title: "",
      body: "",
    },
    validate: {
      title: (v) => (v === "" ? t("errors.title-empty") : null),
      body: (v) => (v === "" ? t("errors.body-empty") : null),
    },
  });

  const onSubmit = form.onSubmit(async (values) => {
    try {
      await api.reportBug(values.title, values.body);
      showNotification({
        title: t("messages.success"),
        message: t("messages.created-report"),
        type: "success",
      });
    } catch (e: any) {
      console.error(e);
      showNotification({
        title: "Error",
        message: e?.message ?? "Error creating report",
        type: "error",
      });
    }
  });

  return (
    <Container fluid>
      <form onSubmit={onSubmit}>
        <Stack gap={10}>
          <Title order={2}>{t("report-bug")}</Title>
          <TextInput
            label={t("fields.title")}
            key={form.key("title")}
            {...form.getInputProps("title")}
          />
          <Textarea
            label={t("fields.description")}
            key={form.key("body")}
            autosize
            {...form.getInputProps("body")}
          />
          <Button type="submit" style={{ width: "15%" }}>
            {t("actions.submit")}
          </Button>
        </Stack>
      </form>
    </Container>
  );
};

export default ReportBugPage;
