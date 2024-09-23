'use server';
import {Badge, Container, Group, Tabs, Title} from "@mantine/core";
import useApiService from "@/hooks/useApiService";
import {Group as GroupType} from "@/service/types/tasky";
import { TabsComponent } from "./client";


const GroupDetailsPage = async ({params}: {params: {id: string}}) => {
    const id = parseInt(`${params.id}`, 10);
    if (isNaN(id)) {
        return (
            <Container fluid>
                <Title>Invalid Group ID</Title>
            </Container>
        )
    }
    const api = useApiService();
    const group = (await api.getGroup(id)) as GroupType;

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
