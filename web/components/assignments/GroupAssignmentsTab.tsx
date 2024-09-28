'use client';
import {AssignmentsResponse, Group as TaskyGroup} from "@/service/types/tasky";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {Button, Container, Group} from "@mantine/core";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";
import {UserRoles} from "@/service/types/usernator";
import {IconPlus} from "@tabler/icons-react";
import {useState} from "react";
import CreateAssignmentModal from "@/components/assignments/CreateAssignmentModal";
import useClientQuery from "@/hooks/useClientQuery";

interface GroupAssignmentsTabProps {
    group: TaskyGroup|null;
}

const GroupAssignmentsTab = ({group}: GroupAssignmentsTabProps) => {

    const api = useApiServiceClient();
    const [createModalOpen, setCreateModalOpen] = useState(false);
    const [assignments, refetch] = useClientQuery<AssignmentsResponse>(() => api.getAssignmentsForGroup(group?.id ?? -1), [group?.id]);


    const {user} = useCurrentUser();

    return (
        <Container fluid>
            <Group justify="end">
                {user && isGranted(user, [UserRoles.Tutor]) && user.groups.map(g => g.id).indexOf(group?.id ?? -1) > -1 && (
                    <Button onClick={() => setCreateModalOpen(true)}>
                        <IconPlus />&nbsp;Create Assignment
                    </Button>
                )}
            </Group>
            {createModalOpen && group && (
                <CreateAssignmentModal group={group} onClose={() => setCreateModalOpen(false)} refetch={refetch} />
            )}
            {(assignments?.assignments ?? []).map((a) => a.title)}
        </Container>
    )
}

export default GroupAssignmentsTab;
