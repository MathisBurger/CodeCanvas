'use server';
import {Container, Title} from "@mantine/core";
import useApiService from "@/hooks/useApiService";
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import {GroupsResponse} from "@/service/types/tasky";
import {useRouter} from "next/navigation";


const GroupsPage = async () => {

    const api = useApiService();
    const groups = (await api.getGroups() as GroupsResponse).groups;
    const router = useRouter();


    const cols: EntityListCol[] = [
        {
            field: 'id',
            label: 'ID'
        },
        {
            field: 'title',
            label: 'Title'
        },
        {
            field: 'member_count',
            label: 'Members Count'
        },
        {
            field: 'tutor',
            label: 'Tutor',
            getter: (row) => row.tutor.username
        }
    ]

    const actions: EntityListRowAction[] = [
        {
            color: 'blue',
            name: 'View',
            onClick: (row) => router.push(`/groups/${row.id}`)
        }
    ];

    return (
        <Container fluid>
            <Title>Groups</Title>
            <EntityList cols={cols} rows={groups} rowActions={actions} />
        </Container>
    );
}

export default GroupsPage;
