'use client';
import { Assignment } from "@/service/types/tasky";
import {Badge, Card, Group, Title} from '@mantine/core';
import RichTextDisplay from "@/components/display/RichTextDisplay";
import AssignmentDateDisplay from "@/components/assignments/AssignmentDateDisplay";
import {useRouter} from "next/navigation";

interface AssignmentCardProps {
    assignment: Assignment;
    groupId: number;
}

const AssignmentCard = ({assignment, groupId}: AssignmentCardProps) => {

    const router = useRouter();

    const navigateTo = () => router.push(`/groups/${groupId}/assignments/${assignment.id}`);

    return (
        <Card shadow="sm" padding="lg" radius="md" withBorder onClick={navigateTo}>
            <Group>
                <Title order={4}>{assignment.title}</Title>
                <Badge color="indigo">{assignment.language}</Badge>
                <AssignmentDateDisplay date={assignment.due_date} />
            </Group>
            <RichTextDisplay content={assignment.description} fullSize={false} />
        </Card>
    );
}

export default AssignmentCard;
