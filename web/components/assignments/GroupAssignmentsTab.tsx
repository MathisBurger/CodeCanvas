import {Group as TaskyGroup} from "@/service/types/tasky";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {Button, Container, Group} from "@mantine/core";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";
import {UserRoles} from "@/service/types/usernator";
import {IconPlus} from "@tabler/icons-react";

interface GroupAssignmentsTabProps {
    group: TaskyGroup|null;
}

const GroupAssignmentsTab = ({group}: GroupAssignmentsTabProps) => {

    const api = useApiServiceClient();
    const {user} = useCurrentUser();

    return (
        <Container fluid>
            <Group justify="end">
                {user && isGranted(user, [UserRoles.Tutor]) && user.groups.map(g => g.id).indexOf(group?.id ?? -1) > -1 && (
                    <Button>
                        <IconPlus />&nbsp;Create Assignment
                    </Button>
                )}
            </Group>
        </Container>
    )
}

export default GroupAssignmentsTab;
