'use client';
import { useForm } from '@mantine/form';
import {Button, Container, Divider, Group, Paper, PasswordInput, Stack, Text, TextInput} from "@mantine/core";
import ApiError from "@/service/types/error";
import {useRouter} from "next/navigation";
import {notifications} from "@mantine/notifications";
import useApiServiceClient from "@/hooks/useApiServiceClient";

interface LoginInput {
    name: string;
    password: string;
}


const LoginPage = () => {

    const api = useApiServiceClient();
    const router = useRouter();

    const form = useForm({
        initialValues: {
            name: '',
            password: '',
        }
    });

    const onSubmit = async (values: LoginInput) => {
        try {
            await api.loginUser(values.name, values.password);
            router.push("/dashboard");
        } catch (e) {
            if (e instanceof ApiError) {
                notifications.show({
                    title: 'Login failed',
                    message: e.message,
                    color: 'red'
                });
            }
        }
    }

    return (
        <Container>
            <Paper radius="md" p="xl" withBorder>
                <Text size="lg" fw={500}>
                    Login to CodeCanvas
                </Text>

                <Divider />

                <form onSubmit={form.onSubmit(onSubmit)}>
                    <Stack>
                        <TextInput
                            required
                            label="Username"
                            placeholder="Your username"
                            value={form.values.name}
                            onChange={(event) => form.setFieldValue('name', event.currentTarget.value)}
                            radius="md"
                        />

                        <PasswordInput
                            required
                            label="Password"
                            placeholder="Your password"
                            value={form.values.password}
                            onChange={(event) => form.setFieldValue('password', event.currentTarget.value)}
                            radius="md"
                        />
                    </Stack>

                    <Group justify="space-between" mt="xl">
                        <Button type="submit" radius="xl">
                            Login
                        </Button>
                    </Group>
                </form>
            </Paper>
        </Container>
    );
}

export default LoginPage;
