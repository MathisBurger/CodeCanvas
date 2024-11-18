"use client";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {useState} from "react";
import {Container, Pagination, Title} from "@mantine/core";
import NavigateBack from "@/components/NavigateBack";
import {useTranslation} from "react-i18next";
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import SolutionBadge from "@/components/solution/SolutionBadge";
import {UserRoles} from "@/service/types/usernator";
import {useRouter} from "next/navigation";


const UserSolutionsPage = ({ params }: { params: { id: string } }) => {
    const id = parseInt(`${params.id}`, 10);
    const api = useApiServiceClient();
    const [page, setPage] = useState<number>(1);
    const [solutions] = useClientQuery(() => api.getUserSolutions(id, page), [id, page]);
    const router = useRouter();
    const {t} = useTranslation(["common", "solution"]);

    const cols: EntityListCol[] = [
        {
            field: "id",
            label: t("cols.id"),
        },
        {
            field: "assignment",
            label: t("solution:cols.assignment"),
            getter: (row) => row.assignment.title,
        },
        {
            field: "approval_status",
            label: t("solution:cols.approval-status"),
            render: (value) => <SolutionBadge status={value as string} />,
        },
    ];

    const rowActions: EntityListRowAction[] = [
        {
            name: t("actions.view"),
            onClick: (row) => router.push(`/solutions/${row.id}`),
            color: undefined,
            auth: [UserRoles.Student, UserRoles.Admin],
        },
    ];

    return (
        <Container fluid>
            <NavigateBack />
            <Title>{t('solution:personal-solutions')}</Title>
            <EntityList
                cols={cols}
                rowActions={rowActions}
                rows={solutions?.solutions ?? []}
            />
            <Pagination
                total={Math.ceil((solutions?.total ?? 0) / 50)}
                value={page}
                onChange={setPage}
            />
        </Container>
    );
}

export default UserSolutionsPage;