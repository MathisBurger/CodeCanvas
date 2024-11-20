"use client";
import useCurrentUser from "@/hooks/useCurrentUser";
import {Container, Title, Text, Card, Grid, Group, Flex, Box, Blockquote} from "@mantine/core";
import {IconInfoCircle, IconTrophyFilled} from "@tabler/icons-react";
import { useTranslation } from "react-i18next";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {Carousel} from "@mantine/carousel";
import RichTextDisplay from "@/components/display/RichTextDisplay";

const DashboardPage = () => {
  const { user } = useCurrentUser();
  const { t } = useTranslation("dashboard");
  const api = useApiServiceClient();
  const [notifications] = useClientQuery(() => api.getSystemWideNotification());

  return (
    <Container fluid>
      <Title>
        {t("welcome-back")} {user?.username}!
      </Title>
      {notifications && notifications?.length > 0 && (
          <Carousel withIndicators height={200}>
            {notifications.map((notification) => (
                <Carousel.Slide key={notification.id}>
                  <Card h={200}>
                    <Box mx="xl">
                      <Title order={2}>{notification.title}</Title>
                      <RichTextDisplay content={notification.content} fullSize={false} />
                    </Box>
                  </Card>
                </Carousel.Slide>
            ))}
          </Carousel>
      )}
      <Blockquote color="indigo" icon={<IconInfoCircle />} mt="xl" cite="~ Development team">
        {t('development-status')}
      </Blockquote>
      <Grid>
        <Grid.Col span={4}>
          <Card shadow="sm" padding="xl" mt={20}>
            <Group justify="space-between">
              <IconTrophyFilled color="#bfba40" size={100} />
              <Flex direction="column">
                <Title order={5}>{t("ole-title")}</Title>
                <Text>{t("ole-text")}</Text>
              </Flex>
            </Group>
          </Card>
        </Grid.Col>
        <Grid.Col span={8}>
          <Card shadow="sm" padding="xl" mt={20}>
            <Title order={2}>Release v0.2.2-stable</Title>
            <Text>
              We had some groundbreaking changes within our app for the current
              release:
              <br />
              - Improved scalability <br/>
              - Group and system wide notifications <br/>
              - Limited runner options for unverified groups <br/>
            </Text>
          </Card>
        </Grid.Col>
      </Grid>
    </Container>
  );
};

export default DashboardPage;
