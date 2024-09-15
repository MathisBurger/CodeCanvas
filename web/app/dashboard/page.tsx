'use client';
import useCurrentUser from "@/hooks/useCurrentUser";
import {Container, Divider, Title} from "@mantine/core";


const DashboardPage = () => {

    const {user} = useCurrentUser();

    return (
        <Container fluid>
            <Title>Welcome back, {user?.username}!</Title>
            <Divider />
        </Container>
    );
}

export default DashboardPage;
