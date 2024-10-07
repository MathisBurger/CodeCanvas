'use client';
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import { SolutionsResponse } from "@/service/types/tasky";
import {Container, Title} from "@mantine/core";
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import {UserRoles} from "@/service/types/usernator";
import {router} from "next/client";


const PersonalSolutionsPage = () => {

    const api = useApiServiceClient();
    const [solutions] = useClientQuery<SolutionsResponse>(() => api.getPersonalSolutions());

    const cols: EntityListCol[] = [
        {
            field: 'id',
            label: 'ID'
        },
        {
            field: 'assignment',
            label: 'Assignment',
            getter: (row) => row.assignment.title
        }
    ];

    const rowActions: EntityListRowAction[] = [
        {
            name: 'View',
            onClick: (row) => router.push(`/solutions/${row.id}`),
            color: undefined,
            auth: [UserRoles.Student],
        }
    ]

    return (
        <Container fluid>
            <Title order={1} mb={20}>Personal solutions</Title>
            <EntityList cols={cols} rowActions={rowActions} rows={solutions?.solutions ?? []} />
        </Container>
    );
}

export default PersonalSolutionsPage;
