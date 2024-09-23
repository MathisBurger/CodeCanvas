'use client';
import {Badge, Container, Group, Title} from "@mantine/core";
import {Group as GroupType} from "@/service/types/tasky";
import { TabsComponent } from "./client";
import useClientQuery from "@/hooks/useClientQuery";
import useApiServiceClient from "@/hooks/useApiServiceClient";


const GroupDetailsPage = ({params}: {params: {id: string}}) => {
    const id = parseInt(`${params.id}`, 10);
    const api = useApiServiceClient();
    const group = useClientQuery<GroupType|string>(() => api.getGroup(id)) as GroupType;
    if (isNaN(id)) {
        return (
            <Container fluid>
                <Title>Invalid Group ID</Title>
            </Container>
        )
    }

    return (
        <Container fluid>
            <Group>
                <Title>{group.title}</Title>
                <Badge>{group.tutor.username}</Badge>
            </Group>
            <TabsComponent group={group} />
        </Container>
    )
}

export default GroupDetailsPage;
