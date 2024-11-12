import { Notification } from "@/service/types/tasky";
import { ActionIcon, Card, Group, Title } from "@mantine/core";
import { IconTrash } from "@tabler/icons-react";
import useApiServiceClient from "@/hooks/useApiServiceClient";

interface NotificationCardProps {
  notification: Notification;
  refetch: () => void;
}

const NotificationCard = ({ notification, refetch }: NotificationCardProps) => {
  const api = useApiServiceClient();

  const remove = async () => {
    await api.removeNotificationForUser(notification.id);
    refetch();
  };

  return (
    <Card>
      <Group>
        <Title order={3}>{notification.title}</Title>
        <ActionIcon
          variant="transparent"
          color="red"
          style={{ marginLeft: "auto" }}
          onClick={remove}
        >
          <IconTrash />
        </ActionIcon>
      </Group>
      {notification.content}
    </Card>
  );
};

export default NotificationCard;
