"use client";
import {
    ActionIcon,
    Avatar,
    Button,
    Drawer,
    Group,
    Indicator,
    Kbd,
    Menu, Stack,
    Text,
    useComputedColorScheme
} from "@mantine/core";
import Link from "next/link";
import { User } from "@/service/types/usernator";
import { useCookies } from "react-cookie";
import { useRouter } from "next/navigation";
import useCurrentUser from "@/hooks/useCurrentUser";
import {useTranslation} from "react-i18next";
import {IconMessage} from "@tabler/icons-react";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {useState} from "react";
import NotificationCard from "@/components/NotificationCard";

interface SsrHeaderProps {
  user: User | null;
}

const SsrHeader: React.FC<SsrHeaderProps> = ({ user }) => {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [s, _, removeSession] = useCookies(["session"]);
  const colorScheme = useComputedColorScheme();
  const { setUser } = useCurrentUser();
  const router = useRouter();
  const {t} = useTranslation('common');
  const api = useApiServiceClient();
  const [notifications, refetch] = useClientQuery(() => api.getNotifications(), [user]);
  const [notificationDrawerShown, setNotificationDrawerShown] = useState<boolean>(false);

  const clearAllNotifications = async () => {
      await api.removeAllNotificationsForUser();
      refetch();
  }

  const logOut = () => {
    removeSession("session");
    router.push("/login");
    setUser(null);
    //window.location.reload();
  };

  if (user === null) {
    return (
      <Group visibleFrom="sm">
        <Link href="/login">
          <Button variant="default">{t('actions.login')}</Button>
        </Link>
        <Link href="/register">
          <Button>{t('actions.sign-up')}</Button>
        </Link>
      </Group>
    );
  }
  return (
    <>
        <Group>
            <Group>
                <Text>{t('titles.spotlight-actions')}:</Text>
                <div dir="ltr">
                    <Kbd>CMD</Kbd> + <Kbd>K</Kbd> or <Kbd>CTRL</Kbd> + <Kbd>K</Kbd>
                </div>
            </Group>
            <Indicator inline label={notifications?.length ?? ""} display={notifications?.length ? "block" : "none"} size={16}>
                <ActionIcon variant="transparent" color={colorScheme === "light" ? "dark" : "white"} onClick={() => setNotificationDrawerShown(true)}>
                    <IconMessage />
                </ActionIcon>
            </Indicator>
            <Menu>
                <Menu.Target>
                    <Avatar name={user.username} color="initials" />
                </Menu.Target>
                <Menu.Dropdown>
                    <Menu.Item onClick={() => router.push("/settings")}>
                        {t('titles.settings')}
                    </Menu.Item>
                    <Menu.Item color="red" onClick={logOut}>
                        {t('titles.log-out')}
                    </Menu.Item>
                </Menu.Dropdown>
            </Menu>
        </Group>
        {notificationDrawerShown && notifications && (
            <Drawer opened onClose={() => setNotificationDrawerShown(false)} title={t('common:titles.notifications')}>
                <Button color="red" variant="light" mb={10} onClick={clearAllNotifications}>{t('common:actions.clear-all')}</Button>
                <Stack gap={10}>
                    {notifications.map((notification) => (
                        <NotificationCard notification={notification} key={notification.id} refetch={refetch} />
                    ))}
                </Stack>
            </Drawer>
        )}
    </>
  );
};

export default SsrHeader;
