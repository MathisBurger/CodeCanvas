'use client';
import {Badge, Tabs} from "@mantine/core";
import React from "react";
import {Group, GroupJoinRequestResponse, TaskyUser} from "@/service/types/tasky";
import EntityList, {EntityListCol} from "@/components/EntityList";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";

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

export const JoinRequestsComponent: React.FC<{group: Group}> = ({group}) => {

    const api = useApiServiceClient();
    const requests = useClientQuery<GroupJoinRequestResponse|string>(() => api.getGroupJoinRequests(group.id));

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
    ]

    return (
        <EntityList cols={cols} rows={requests ? (requests as GroupJoinRequestResponse).requests : []} />
    );
}


export const TabsComponent: React.FC<{group: Group}> = ({group}) => (
    <Tabs defaultValue="assignments" style={{marginTop: '2em'}}>
        <Tabs.List>
            <Tabs.Tab value="assignments">
                Assignments
            </Tabs.Tab>
            <Tabs.Tab value="members">
                Members
            </Tabs.Tab>
            <Tabs.Tab value="joinRequests" rightSection={group.request_count > 0 ? <Badge color="red">{group.request_count}</Badge> : null}>
                Join Requests
            </Tabs.Tab>
        </Tabs.List>
        <div style={{marginTop: '2em'}}>
            <Tabs.Panel value="assignments">
                Assignments
            </Tabs.Panel>
            <Tabs.Panel value="members">
                <MembersComponent members={group.members} />
            </Tabs.Panel>
            <Tabs.Panel value="joinRequests">
                <JoinRequestsComponent group={group} />
            </Tabs.Panel>
        </div>
    </Tabs>
);
