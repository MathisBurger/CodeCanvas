"use client";
import useCurrentUser from "@/hooks/useCurrentUser";
import {Container, Title, Text, Card, Grid, Group, Flex} from "@mantine/core";
import {IconTrophyFilled} from "@tabler/icons-react";
import {useTranslation} from "react-i18next";

const DashboardPage = () => {
  const { user } = useCurrentUser();
  const {t} = useTranslation('dashboard');

  return (
    <Container fluid>
      <Title>{t('welcome-back')} {user?.username}!</Title>
        <Card
            shadow="sm"
            padding="xl"
            mt={20}
        >
            <Text mt="xs" c="dimmed" size="sm">
                {t('us-again-text')}
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
                            <Title order={5}>{t('ole-title')}</Title>
                            <Text>{t('ole-text')}</Text>
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
