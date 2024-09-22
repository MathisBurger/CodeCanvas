'use server';
import {Container, Title} from "@mantine/core";
import useApiService from "@/hooks/useApiService";
import {GroupsResponse} from "@/service/types/tasky";
import GroupsDisplayComponent from "@/app/groups/displayComponent";


const GroupsPage = async () => {

    const api = useApiService();
    const groups = (await api.getGroups() as GroupsResponse).groups;



    return (
        <Container fluid>
            <Title>Groups</Title>
            <GroupsDisplayComponent  groups={groups}/>
        </Container>
    );
}

export default GroupsPage;
