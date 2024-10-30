'use client';
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {Container, Stack, Textarea, TextInput, Title, Button} from "@mantine/core";
import {useForm} from "@mantine/form";
import {showNotification} from "@mantine/notifications";


const ReportBugPage = () => {

    const api = useApiServiceClient();
    const form = useForm({
        initialValues: {
            title: '',
            body: ''
        },
        validate: {
            title: (v) => v === '' ? 'The title should not be empty' : null,
            body: (v) => v === '' ? 'The body should not be empty' : null,
        }
    });

    const onSubmit = form.onSubmit(async (values) => {
        try {
            await api.reportBug(values.title, values.body);
            showNotification({
                title: 'Success',
                message: "Created report",
                type: 'success',
            });
        } catch (e: any) {
            console.error(e);
            showNotification({
                title: 'Error',
                message: e?.message ?? "Error creating report",
                type: 'error',
            });
        }
    });

    return (
        <Container fluid>
            <form onSubmit={onSubmit}>
                <Stack gap={10}>
                    <Title order={2}>Report Bug</Title>
                    <TextInput
                        label="Title"
                        key={form.key('title')}
                        {...form.getInputProps('title')}
                    />
                    <Textarea
                        label="Description"
                        key={form.key('body')}
                        autosize
                        {...form.getInputProps('body')}
                    />
                    <Button type="submit" style={{width: '15%'}}>
                        Submit
                    </Button>
                </Stack>
            </form>
        </Container>
    )
}

export default ReportBugPage;
