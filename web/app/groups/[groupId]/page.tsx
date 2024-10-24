"use client";
import { Badge, Container, Group, Title } from "@mantine/core";
import { Group as GroupType } from "@/service/types/tasky";
import { TabsComponent } from "./client";
import useClientQuery from "@/hooks/useClientQuery";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import CentralLoading from "@/components/CentralLoading";
import {useSpotlightStage2} from "@/hooks/spotlight/stage2";
import {useEffect} from "react";

const GroupDetailsPage = ({ params }: { params: { groupId: string } }) => {
  const id = parseInt(`${params.groupId}`, 10);
  const api = useApiServiceClient();
  const [group, refetch] = useClientQuery<GroupType>(() => api.getGroup(id));
  const {addGroup} = useSpotlightStage2();

  useEffect(() => {
    if (group) {
      addGroup(group);
    }
  }, [addGroup, group]);

  if (isNaN(id)) {
    return (
      <Container fluid>
        <Title>Invalid Group ID</Title>
      </Container>
    );
  }

  return (
    <Container fluid>
      <Group>
        <Title>{group?.title ?? "Loading"}</Title>
        <Badge>{group?.tutor?.username ?? "Loading"}</Badge>
      </Group>
        {group === null ? (
            <CentralLoading />
        ) : (
            <TabsComponent group={group} refetch={refetch} />
        )}
    </Container>
  );
};

export default GroupDetailsPage;
