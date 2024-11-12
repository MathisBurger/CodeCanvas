"use client";
import classes from "./home.module.css";
import {
  Image,
  Container,
  Title,
  Button,
  Group,
  Text,
  List,
  ThemeIcon,
  rem,
} from "@mantine/core";
import { IconCheck } from "@tabler/icons-react";
import { useRouter } from "next/navigation";
import useCurrentUser from "@/hooks/useCurrentUser";
import { useTranslation } from "react-i18next";

export default function Home() {
  const router = useRouter();
  const currentUser = useCurrentUser();
  const { t } = useTranslation("landing-page");
  return (
    <Container size="md">
      <div className={classes.inner}>
        <div className={classes.content}>
          <Title className={classes.title}>
            {t("a")} <span className={classes.highlight}>{t("modern")}</span>{" "}
            {t("platform")} <br /> {t("title")}
          </Title>
          <Text c="dimmed" mt="md">
            {t("subtitle")}
          </Text>

          <List
            mt={30}
            spacing="sm"
            size="sm"
            icon={
              <ThemeIcon size={20} radius="xl">
                <IconCheck
                  style={{ width: rem(12), height: rem(12) }}
                  stroke={1.5}
                />
              </ThemeIcon>
            }
          >
            <List.Item>
              <b>{t("question-assignments")}</b> – {t("questions-text")}
            </List.Item>
            <List.Item>
              <b>{t("free-open-source")}</b> – {t("free-text")}
            </List.Item>
            <List.Item>
              <b>{t("code-testing")}</b> – {t("test-text")}
            </List.Item>
          </List>

          <Group mt={30}>
            <Button
              radius="xl"
              size="md"
              className={classes.control}
              onClick={() =>
                router.push(currentUser === null ? "/register" : "/dashboard")
              }
            >
              {currentUser === null ? "Get started" : "Dashboard"}
            </Button>
            <Button
              variant="default"
              radius="xl"
              size="md"
              className={classes.control}
              onClick={() =>
                router.push("https://github.com/MathisBurger/CodeCanvas")
              }
            >
              Source code
            </Button>
          </Group>
        </div>
        <Image src="image.svg" className={classes.image} />
      </div>
    </Container>
  );
}
