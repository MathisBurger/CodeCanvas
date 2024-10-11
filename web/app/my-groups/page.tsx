"use client";
import {Button, Container, Title} from "@mantine/core";
import { GroupsResponse } from "@/service/types/tasky";
import GroupsDisplayComponent from "@/app/groups/displayComponent";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {useState} from "react";
import CreateGroupModal from "@/components/group/CreateGroupModal";

const GroupsPage = () => {
  const api = useApiServiceClient();
  const [groups] = useClientQuery<GroupsResponse>(() => api.getMyGroups());
  const [createModalOpen, setCreateModalOpen] = useState(false);

  return (
    <Container fluid>
      <Title>My Groups</Title>
        <Button onClick={() => setCreateModalOpen(true)}>Create group</Button>
      <GroupsDisplayComponent groups={groups?.groups ?? []} page="my-groups" />
        {createModalOpen && (
            <CreateGroupModal onClose={() => setCreateModalOpen(false)} />
        )}
    </Container>
  );
};

export default GroupsPage;
