"use client";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useState} from "react";
import useClientQuery from "@/hooks/useClientQuery";
import {useTranslation} from "react-i18next";
import {Container, Pagination, Stack, Title} from "@mantine/core";
import AssignmentCard from "@/components/assignments/AssignmentCard";


const PendingAssignmentsPage = () => {

    const api = useApiServiceClient();
    const [page, setPage] = useState<number>(1);
    const [assignments] = useClientQuery(() => api.getPendingAssignments(page), [page]);
    const { t } = useTranslation("assignment");

    return (
        <Container fluid>
            <Title mb={4}>{t('assignment:titles.pending-assignments')}</Title>
            <Stack gap={20}>
                {(assignments?.assignments ?? []).map((assignment) => (
                    <AssignmentCard assignment={assignment} groupId={assignment.group_id} key={assignment.id} />
                ))}
            </Stack>
            <Pagination
                total={Math.ceil((assignments?.total ?? 0) / 50)}
                value={page}
                onChange={setPage}
            />
        </Container>
    );
}

export default PendingAssignmentsPage;
