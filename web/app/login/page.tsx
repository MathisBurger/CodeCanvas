"use client";
import { useForm } from "@mantine/form";
import {
  Button,
  Container,
  Divider,
  Group,
  Paper,
  PasswordInput,
  Stack,
  Text,
  TextInput,
} from "@mantine/core";
import ApiError from "@/service/types/error";
import { useRouter } from "next/navigation";
import { notifications } from "@mantine/notifications";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { useTranslation } from "react-i18next";

interface LoginInput {
  name: string;
  password: string;
}

const LoginPage = () => {
  const api = useApiServiceClient();
  const router = useRouter();
  const { t } = useTranslation("common");

  const form = useForm({
    initialValues: {
      name: "",
      password: "",
    },
  });

  const onSubmit = async (values: LoginInput) => {
    try {
      await api.loginUser(values.name, values.password);
      router.push("/dashboard");
    } catch (e) {
      if (e instanceof ApiError) {
        notifications.show({
          title: t("login-failed"),
          message: e.message,
          color: "red",
        });
      }
    }
  };

  return (
    <Container>
      <Paper radius="md" p="xl" withBorder>
        <Text size="lg" fw={500}>
          {t("login-to-cc")}
        </Text>

        <Divider />

        <form onSubmit={form.onSubmit(onSubmit)}>
          <Stack>
            <TextInput
              required
              label={t("fields.username")}
              placeholder={t("fields.username-placeholder")}
              value={form.values.name}
              onChange={(event) =>
                form.setFieldValue("name", event.currentTarget.value)
              }
              radius="md"
            />

            <PasswordInput
              required
              label={t("fields.password")}
              placeholder={t("fields.password-placeholder")}
              value={form.values.password}
              onChange={(event) =>
                form.setFieldValue("password", event.currentTarget.value)
              }
              radius="md"
            />
          </Stack>

          <Group justify="space-between" mt="xl">
            <Button type="submit" radius="xl">
              {t("actions.login")}
            </Button>
          </Group>
        </form>
      </Paper>
    </Container>
  );
};

export default LoginPage;
