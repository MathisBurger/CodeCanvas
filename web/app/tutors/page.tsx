'use client';
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {User} from "@/service/types/usernator";
import EntityList, {EntityListCol} from "@/components/EntityList";
import {Button, Container, Group, Title} from "@mantine/core";
import {useState} from "react";
import CreateTutorModal from "@/components/CreateTutorModal";


const TutorsPage = () => {

    const api = useApiServiceClient();
    const [tutors, refetch] = useClientQuery<{tutors: User[]}>(() =>
        api.getTutors(),
    );
    const [createModalOpen, setCreateModalOpen] = useState(false);

    const cols: EntityListCol[] = [
        {
            field: "id",
            label: "ID",
        },
        {
            field: "username",
            label: "Username",
        },
    ];

    return (
        <Container fluid>
            <Group>
                <Title>Tutors</Title>
                <Button onClick={() => setCreateModalOpen(true)}>Create tutor</Button>
            </Group>
            <EntityList cols={cols} rows={tutors?.tutors ?? []} />
            {createModalOpen && (
                <CreateTutorModal onClose={() => setCreateModalOpen(false)} refetch={refetch} />
            )}
        </Container>
    );
}

export default TutorsPage;
