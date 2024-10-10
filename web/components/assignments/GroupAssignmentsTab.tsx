"use client";
import {
  Assignment,
  AssignmentsResponse,
  Group as TaskyGroup,
} from "@/service/types/tasky";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { Button, Container, Flex, Group } from "@mantine/core";
import useCurrentUser from "@/hooks/useCurrentUser";
import { isGranted } from "@/service/auth";
import { UserRoles } from "@/service/types/usernator";
import { IconPlus } from "@tabler/icons-react";
import { useState } from "react";
import CreateOrUpdateAssignmentModal from "@/components/assignments/CreateOrUpdateAssignmentModal";
import useClientQuery from "@/hooks/useClientQuery";
import AssignmentCard from "@/components/assignments/AssignmentCard";

interface GroupAssignmentsTabProps {
  group: TaskyGroup | null;
}

const assignmentSort = (a: Assignment, b: Assignment) => {
  const timeA = new Date(a.due_date).getTime();
  const timeB = new Date(b.due_date).getTime();
  if (timeA < timeB) return 1;
  if (timeA > timeB) return -1;
  return 0;
};

const GroupAssignmentsTab = ({ group }: GroupAssignmentsTabProps) => {
  const api = useApiServiceClient();
  const [createModalOpen, setCreateModalOpen] = useState(false);
  const [assignments, refetch] = useClientQuery<AssignmentsResponse>(
    () => api.getAssignmentsForGroup(group?.id ?? -1),
    [group?.id],
  );

  const { user } = useCurrentUser();

  return (
    <Container fluid pb={30}>
      <Group justify="end" mb={20}>
        {user &&
          isGranted(user, [UserRoles.Tutor]) &&
          user.groups.map((g) => g.id).indexOf(group?.id ?? -1) > -1 && (
            <Button onClick={() => setCreateModalOpen(true)}>
              <IconPlus />
              &nbsp;Create Assignment
            </Button>
          )}
      </Group>
      {createModalOpen && group && (
        <CreateOrUpdateAssignmentModal
          groupId={group.id ?? -1}
          onClose={() => setCreateModalOpen(false)}
          refetch={refetch}
          action="create"
        />
      )}
      <Flex direction="column" gap="xl">
        {(assignments?.assignments ?? []).sort(assignmentSort).map((a) => (
          <AssignmentCard assignment={a} groupId={group?.id ?? -1} key={a.id} />
        ))}
      </Flex>
    </Container>
  );
};

export default GroupAssignmentsTab;
