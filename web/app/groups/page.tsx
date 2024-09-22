'use client';
import {Container, Title} from "@mantine/core";
import useApiService from "@/hooks/useApiService";
import {useEffect, useState} from "react";
import EntityList, {EntityListCol} from "@/components/EntityList";
import {GroupsResponse, MinifiedGroup} from "@/service/types/tasky";


const GroupsPage = () => {

    const api = useApiService();
    const [groups, setGroups] = useState<MinifiedGroup[]>([]);

    useEffect(() => {
        api.getGroups().then((response) => setGroups((response as GroupsResponse).groups));
    }, []);

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

    return (
        <Container fluid>
            <Title>Groups</Title>
            <EntityList cols={cols} rows={groups} />
        </Container>
    );
}

export default GroupsPage;
