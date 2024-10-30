"use client";
import useCurrentUser from "@/hooks/useCurrentUser";
import {Container, Title, Text, Card, Grid, Group, Flex} from "@mantine/core";
import {IconTrophyFilled} from "@tabler/icons-react";

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
        <Grid>
            <Grid.Col span={4}>
                <Card
                    shadow="sm"
                    padding="xl"
                    mt={20}
                >
                    <Group justify="space-between">
                        <IconTrophyFilled color="#bfba40" size={100} />
                        <Flex direction="column">
                            <Title order={5}>Oleggtro: contributor of year!</Title>
                            <Text>Throughout the year, Oleggtro has gone above and beyond, consistently sharing insights, knowledge, and support with our community. His dedication and contributions have made a significant impact, enriching our platform and setting a high standard for collaboration.
                                Thank you, Oleggtro, for your hard work, passion, and unwavering commitment. We are incredibly grateful to have you as part of our team! Here’s to many more achievements together.</Text>
                        </Flex>
                    </Group>
                </Card>
            </Grid.Col>
            <Grid.Col span={8}>
                <Card
                    shadow="sm"
                    padding="xl"
                    mt={20}
                >
                    <Title order={2}>Release v0.1.5</Title>
                    <Text>
                        We had some groundbreaking changes within our app for the current release:<br />
                        - Comments on solutions by the tutor <br/>
                        - Stage2 spotlight search <br/>
                        - Bug reporting feature <br/>
                        - Assignment wishes within the group <br/>
                        - Assignments without due date
                    </Text>
                </Card>
            </Grid.Col>
        </Grid>
    </Container>
  );
};

export default DashboardPage;
