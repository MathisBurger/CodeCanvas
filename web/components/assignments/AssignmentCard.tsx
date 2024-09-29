import { Assignment } from "@/service/types/tasky";
import {Badge, Card, Group, Text, Title} from '@mantine/core';
import RichTextDisplay from "@/components/display/RichTextDisplay";
import {useMemo} from "react";
import dayjs from "dayjs";

interface AssignmentCardProps {
    assignment: Assignment;
}

const AssignmentCard = ({assignment}: AssignmentCardProps) => {

    const dueDate = useMemo(() => new Date(assignment.due_date), [assignment.due_date]);
    const dueDateIsOver = useMemo(() => dueDate.getTime() < new Date().getTime(), [dueDate]);
    const formattedDueDate = useMemo(() => dayjs(dueDate).format("DD/MM/YYYY hh:mm"), [dueDate])

    return (
        <Card shadow="sm" padding="lg" radius="md" withBorder>
            <Group>
                <Title order={4}>{assignment.title}</Title>
                <Badge color="indigo">{assignment.language}</Badge>
                <Text c={dueDateIsOver ? "red" : "gray"} td={dueDateIsOver ? "line-through" : undefined}>{formattedDueDate}</Text>
            </Group>
            <RichTextDisplay content={assignment.description} fullSize={false} />
        </Card>
    );
}

export default AssignmentCard;
