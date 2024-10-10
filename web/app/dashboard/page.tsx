"use client";
import useCurrentUser from "@/hooks/useCurrentUser";
import { Container, Title } from "@mantine/core";

const DashboardPage = () => {
  const { user } = useCurrentUser();

  return (
    <Container fluid>
      <Title>Welcome back, {user?.username}!</Title>
    </Container>
  );
};

export default DashboardPage;
