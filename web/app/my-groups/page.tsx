'use client';
import {Container, Title} from "@mantine/core";
import {GroupsResponse} from "@/service/types/tasky";
import GroupsDisplayComponent from "@/app/groups/displayComponent";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";


const GroupsPage = () => {

    const api = useApiServiceClient();
    const [groups] = useClientQuery<GroupsResponse>(() => api.getMyGroups())



    return (
        <Container fluid>
            <Title>My Groups</Title>
            <GroupsDisplayComponent  groups={groups?.groups ?? []}/>
        </Container>
    );
}

export default GroupsPage;
