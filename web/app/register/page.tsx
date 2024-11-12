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

interface RegisterInput {
  name: string;
  password: string;
}

const RegisterPage = () => {
  const api = useApiServiceClient();
  const router = useRouter();
  const { t } = useTranslation("common");

  const form = useForm({
    initialValues: {
      name: "",
      password: "",
    },

    validate: {
      password: (val) =>
        val.length <= 6 ? t("messages.password-requirements") : null,
    },
  });

  const onSubmit = async (values: RegisterInput) => {
    try {
      await api.registerUser(values.name, values.password);
      router.push("/login");
    } catch (e) {
      if (e instanceof ApiError) {
        notifications.show({
          title: t("messages.registration-failed"),
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
          {t("register-cc")}
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
              error={
                form.errors.password && t("messages.password-requirements")
              }
              radius="md"
            />
          </Stack>

          <Group justify="space-between" mt="xl">
            <Button type="submit" radius="xl">
              {t("actions.sign-up")}
            </Button>
          </Group>
        </form>
      </Paper>
    </Container>
  );
};

export default RegisterPage;
