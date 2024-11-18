"use client";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useState} from "react";
import useClientQuery from "@/hooks/useClientQuery";
import {useTranslation} from "react-i18next";
import {Container, Pagination, Title} from "@mantine/core";
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import {UserRoles} from "@/service/types/usernator";
import {showNotification} from "@mantine/notifications";


const PendingWishes = () => {
    const api = useApiServiceClient();
    const [page, setPage] = useState<number>(1);
    const [wishes, refetch] = useClientQuery(() => api.getPendingWishes(page), [page]);
    const {t} = useTranslation(['common', 'assignment']);

    const cols: EntityListCol[] = [
        {
            field: "title",
            label: t("fields.title"),
        },
        {
            field: "description",
            label: t("fields.description"),
        },
        {
            field: 'group_id',
            label: t("fields.group_id")
        }
    ];

    const rowActions: EntityListRowAction[] = [
        {
            auth: [UserRoles.Admin, UserRoles.Tutor],
            name: t("actions.delete"),
            onClick: async (row) => {
                try {
                    await api.deleteAssignmentWish(row.group_id, row.id);
                    refetch();
                } catch (e: any) {
                    showNotification({
                        title: t("messages.error"),
                        message: e?.message ?? "",
                    });
                }
            },
            color: "red",
        },
    ];
    return (
        <Container fluid>
            <Title>{t('assignment:titles.pending-wishes')}</Title>
            <EntityList
                cols={cols}
                rowActions={rowActions}
                rows={wishes?.results ?? []}
            />
            <Pagination
                total={Math.ceil((wishes?.total ?? 0) / 50)}
                value={page}
                onChange={setPage}
            />
        </Container>
    )
}

export default PendingWishes;
