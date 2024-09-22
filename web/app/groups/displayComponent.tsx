'use client';
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import {redirect, useRouter} from "next/navigation";
import {MinifiedGroup} from "@/service/types/tasky";

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

    const actions: EntityListRowAction[] = [
        {
            color: 'blue',
            name: 'View',
            onClick: (row) => router.push(`/groups/${row.id}`)
        }
    ];

    return <EntityList cols={cols} rows={groups} rowActions={actions} />
}

export default GroupsDisplayComponent;
