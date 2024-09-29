'use client';
import {Container, Title} from "@mantine/core";
import {GroupsResponse} from "@/service/types/tasky";
import GroupsDisplayComponent from "@/app/groups/displayComponent";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";


const GroupsPage = () => {

    const api = useApiServiceClient();
    const [groups, refetch] = useClientQuery<GroupsResponse>(() => api.getGroups())



    return (
        <Container fluid>
            <Title>Groups</Title>
            <GroupsDisplayComponent  groups={groups?.groups ?? []} page="groups" refetch={refetch} />
        </Container>
    );
}

export default GroupsPage;
