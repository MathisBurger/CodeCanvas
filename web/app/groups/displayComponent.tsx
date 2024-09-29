'use client';
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import {useRouter} from "next/navigation";
import {MinifiedGroup} from "@/service/types/tasky";
import {UserRoles} from "@/service/types/usernator";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {notifications} from "@mantine/notifications";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";

interface DisplayComponentProps {
    groups: MinifiedGroup[];
    refetch?: () => void;
    page: 'my-groups'|'groups'
}

const GroupsDisplayComponent = ({groups, page, refetch}: DisplayComponentProps) => {

    const router = useRouter();
    const cols: EntityListCol[] = [
        {
            field: 'id',
            label: 'ID'
        },
        {
            field: 'title',
            label: 'Title'
        },
        {
            field: 'member_count',
            label: 'Members Count'
        },
        {
            field: 'tutor',
            label: 'Tutor',
            getter: (row) => row.tutor.username
        }
    ]
    const api = useApiServiceClient();
    const {user} = useCurrentUser();

    const actions: EntityListRowAction[] = [
        {
            color: 'blue',
            name: 'View',
            onClick: (row) => router.push(`/groups/${row.id}`),
            auth: [UserRoles.Admin, UserRoles.Tutor, UserRoles.Student],
            authFunc: () => page === 'groups' ? isGranted(user, [UserRoles.Tutor, UserRoles.Admin]) : true
        },
        {
            color: 'blue',
            name: 'Request Join',
            onClick: (row) => api.createGroupJoinRequest(row.id).then(() => {
                notifications.show({
                    title: 'Join Request created',
                    message: 'Created join request on group ' + row.title
                });
                if (refetch) refetch();
            }),
            auth: [UserRoles.Student],
            authFunc: (row) => (user?.groups ?? []).map(g => g.id).indexOf(row.id) === -1 && page === 'groups'
        }
    ];

    return <EntityList cols={cols} rows={groups} rowActions={actions} />
}

export default GroupsDisplayComponent;
