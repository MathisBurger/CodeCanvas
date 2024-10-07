'use client';
import {Badge, Tabs} from "@mantine/core";
import React from "react";
import {Group, GroupJoinRequestResponse, TaskyUser} from "@/service/types/tasky";
import EntityList, {EntityListCol, EntityListRowAction} from "@/components/EntityList";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {UserRoles} from "@/service/types/usernator";
import GroupAssignmentsTab from "@/components/assignments/GroupAssignmentsTab";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";

const MembersComponent: React.FC<{members: TaskyUser[]}> = ({members}) => {

    const cols: EntityListCol[] = [
        {
            field: 'id',
            label: 'ID'
        },
        {
            field: 'username',
            label: 'Username'
        }
    ]

    return (
        <EntityList cols={cols} rows={members} />
    )
}

export const JoinRequestsComponent: React.FC<{group: Group|null, refetchParent: () => void}> = ({group, refetchParent}) => {

    const api = useApiServiceClient();
    const [requests, refetch] = useClientQuery<GroupJoinRequestResponse>(() => api.getGroupJoinRequests(group?.id ?? -1), [group?.id]);

    const cols: EntityListCol[] = [
        {
            field: 'id',
            label: 'ID'
        },
        {
            field: 'username',
            label: 'Username',
            getter: (row) => row.requestor.username
        }
    ];

    const actions: EntityListRowAction[] = [
        {
            name: 'Approve',
            color: 'green',
            onClick: (row) => api.approveGroupJoinRequest(row.group_id, row.id).then(() => {refetch(); refetchParent();}),
            auth: [UserRoles.Tutor, UserRoles.Admin]
        },
        {
            name: 'Reject',
            color: 'red',
            onClick: (row) => api.rejectGroupJoinRequest(row.group_id, row.id).then(() => {refetch(); refetchParent();}),
            auth: [UserRoles.Tutor, UserRoles.Admin]
        }
    ];

    return (
        <EntityList cols={cols} rows={requests ? (requests as GroupJoinRequestResponse).requests : []} rowActions={actions} />
    );
}


export const TabsComponent: React.FC<{group: Group|null, refetch: () => void}> = ({group, refetch}) => {

    const {user} = useCurrentUser();

    return (
        <Tabs defaultValue="assignments" style={{marginTop: '2em'}}>
            <Tabs.List>
                <Tabs.Tab value="assignments">
                    Assignments
                </Tabs.Tab>
                <Tabs.Tab value="members">
                    Members
                </Tabs.Tab>
                {isGranted(user, [UserRoles.Admin, UserRoles.Tutor]) && (
                    <Tabs.Tab value="joinRequests" rightSection={group && group.request_count > 0 ? <Badge color="red">{group.request_count}</Badge> : null}>
                        Join Requests
                    </Tabs.Tab>
                )}
            </Tabs.List>
            <div style={{marginTop: '2em'}}>
                <Tabs.Panel value="assignments">
                    <GroupAssignmentsTab group={group} />
                </Tabs.Panel>
                <Tabs.Panel value="members">
                    <MembersComponent members={group?.members ?? []} />
                </Tabs.Panel>
                {isGranted(user, [UserRoles.Admin, UserRoles.Tutor]) && (
                    <Tabs.Panel value="joinRequests">
                        {group !== null && group !== undefined && (
                            <JoinRequestsComponent group={group} refetchParent={refetch} />
                        )}
                    </Tabs.Panel>
                )}
            </div>
        </Tabs>
    )
}
