"use client";
import useCurrentUser from "@/hooks/useCurrentUser";
import { Container, Title, Text, Card } from "@mantine/core";

const DashboardPage = () => {
  const { user } = useCurrentUser();

  return (
    <Container fluid>
      <Title>Welcome back, {user?.username}!</Title>
        <Card
            shadow="sm"
            padding="xl"
            mt={20}
        >
            <Text mt="xs" c="dimmed" size="sm">
                Hey, its us again. Please be aware of that this software is totally free to use for you. We do not store
                any personal data except from your username and password. Nevertheless, we have to pay our fees, for domains
                and server hosting. So if you want, feel free to support us, because developing this application takes a lot of time.
            </Text>
        </Card>
    </Container>
  );
};

export default DashboardPage;
