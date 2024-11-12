"use client";
import { Assignment } from "@/service/types/tasky";
import { Badge, Card, Group, Title } from "@mantine/core";
import RichTextDisplay from "@/components/display/RichTextDisplay";
import AssignmentDateDisplay from "@/components/assignments/AssignmentDateDisplay";
import { useRouter } from "next/navigation";
import { useTranslation } from "react-i18next";
import styles from "./AssignmentCard.module.scss";

interface AssignmentCardProps {
  assignment: Assignment;
  groupId: number;
}

const AssignmentCard = ({ assignment, groupId }: AssignmentCardProps) => {
  const router = useRouter();
  const { t } = useTranslation("assignment");

  const navigateTo = () =>
    router.push(`/groups/${groupId}/assignments/${assignment.id}`);

  return (
    <Card
      padding="lg"
      radius="md"
      withBorder
      onClick={navigateTo}
      className={styles.elevateHover}
    >
      <Group>
        <Title order={4}>{assignment.title}</Title>
        <Badge color="indigo">{assignment.language}</Badge>
        {assignment.completed && (
          <Badge color="green">{t("messages.completed")}</Badge>
        )}
        <AssignmentDateDisplay date={assignment.due_date} />
      </Group>
      <RichTextDisplay content={assignment.description} fullSize={false} />
    </Card>
  );
};

export default AssignmentCard;
