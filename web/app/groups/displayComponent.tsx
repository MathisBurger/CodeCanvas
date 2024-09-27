'use client';
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import {useRouter} from "next/navigation";
import {MinifiedGroup} from "@/service/types/tasky";
import {UserRoles} from "@/service/types/usernator";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { notifications } from "@mantine/notifications";

interface DisplayComponentProps {
    groups: MinifiedGroup[];
}

const GroupsDisplayComponent = ({groups}: DisplayComponentProps) => {

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

    const actions: EntityListRowAction[] = [
        {
            color: 'blue',
            name: 'View',
            onClick: (row) => router.push(`/groups/${row.id}`),
            auth: [UserRoles.Admin, UserRoles.Tutor],
        },
        {
            color: 'blue',
            name: 'Request Join',
            onClick: (row) => api.createGroupJoinRequest(row.id).then(() => {
                notifications.show({
                    title: 'Join Request created',
                    message: 'Created join request on group ' + row.title
                })
            }),
            auth: [UserRoles.Student],
        }
    ];

    return <EntityList cols={cols} rows={groups} rowActions={actions} />
}

export default GroupsDisplayComponent;
