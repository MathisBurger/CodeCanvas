"use client";
import {
  Assignment,
  AssignmentsResponse,
  Group as TaskyGroup,
} from "@/service/types/tasky";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {Button, Container, Flex, Group, Pagination} from "@mantine/core";
import useCurrentUser from "@/hooks/useCurrentUser";
import { isGranted } from "@/service/auth";
import { UserRoles } from "@/service/types/usernator";
import { IconPlus } from "@tabler/icons-react";
import { useState } from "react";
import CreateOrUpdateAssignmentModal from "@/components/assignments/CreateOrUpdateAssignmentModal";
import useClientQuery from "@/hooks/useClientQuery";
import AssignmentCard from "@/components/assignments/AssignmentCard";
import {useTranslation} from "react-i18next";

interface GroupAssignmentsTabProps {
  group: TaskyGroup | null;
}

const assignmentSort = (a: Assignment, b: Assignment) => {
  const timeA = new Date(a.due_date ?? "").getTime();
  const timeB = new Date(b.due_date ?? "").getTime();
  if (timeA < timeB) return 1;
  if (timeA > timeB) return -1;
  return 0;
};

const GroupAssignmentsTab = ({ group }: GroupAssignmentsTabProps) => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const [createModalOpen, setCreateModalOpen] = useState(false);
  const [assignments, refetch] = useClientQuery<AssignmentsResponse>(
    () => api.getAssignmentsForGroup(group?.id ?? -1, page),
    [group?.id, page],
  );
  const {t} = useTranslation('assignment');

  const { user } = useCurrentUser();

  return (
    <Container fluid pb={30}>
      <Group justify="end" mb={20}>
        {user &&
          isGranted(user, [UserRoles.Tutor]) &&
          user.groups.map((g) => g.id).indexOf(group?.id ?? -1) > -1 && (
            <Button onClick={() => setCreateModalOpen(true)}>
              <IconPlus />
              &nbsp;{t('actions.create-assignment')}
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
        <Pagination total={Math.ceil((assignments?.total ?? 0) / 50)} value={page} onChange={setPage} />
      </Flex>
    </Container>
  );
};

export default GroupAssignmentsTab;
