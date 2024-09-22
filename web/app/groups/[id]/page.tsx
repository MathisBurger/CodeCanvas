'use server';
import {Container, Title} from "@mantine/core";
import useApiService from "@/hooks/useApiService";
import {Group} from "@/service/types/tasky";


const GroupDetails = async () => {
    const api = useApiService();
    //const group = (await api.getGroup(6)) as Group;


    return (
        <Container fluid>
            <Title>Group lol</Title>
        </Container>
    )
}

export default GroupDetails;
