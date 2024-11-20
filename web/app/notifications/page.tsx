"use client";
import {Button, Container, Stack, Title} from "@mantine/core";
import {useTranslation} from "react-i18next";
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import RichTextDisplay from "@/components/display/RichTextDisplay";
import useClientQuery from "@/hooks/useClientQuery";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useState} from "react";
import CreateSystemWideNotificationModal from "@/components/CreateSystemWideNotificationModal";


const NotificationsPage = () => {

    const {t} = useTranslation('common');
    const api = useApiServiceClient();
    const [notifications, refetch] = useClientQuery(() => api.getSystemWideNotification());
    const [createModalOpen, setCreateModalOpen] = useState<boolean>(false);

    const cols: EntityListCol[] = [
        {
            field: 'title',
            label: t('common:fields.title')
        },
        {
            field: 'content',
            label: t('common:fields.description'),
            render: (value) => <RichTextDisplay content={value as string} fullSize={false} />
        }
    ];

    const rowActions: EntityListRowAction[] = [
        {
            name: t('actions.delete'),
            onClick: async (row) => {
                await api.deleteSystemWideNotifications(row.id);
                refetch();
            },
            color: 'red'
        }
    ]

    return (
        <Container fluid>
            <Stack gap={5}>
                <Title>{t('common:titles.system-wide-notifications')}</Title>
                <Button color="indigo" onClick={() => setCreateModalOpen(true)}>{t('common:actions.create-notification')}</Button>
                <EntityList cols={cols} rows={notifications ?? []} rowActions={rowActions} />
            </Stack>
            {createModalOpen && (
                <CreateSystemWideNotificationModal onClose={() => setCreateModalOpen(false)} refetch={refetch} />
            )}
        </Container>
    );
}

export default NotificationsPage;
