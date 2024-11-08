'use client';
import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import {User} from "@/service/types/usernator";
import EntityList, {EntityListCol} from "@/components/EntityList";
import {Button, Container, Group, Pagination, Title} from "@mantine/core";
import {useState} from "react";
import CreateTutorModal from "@/components/CreateTutorModal";
import {useTranslation} from "react-i18next";


const TutorsPage = () => {

    const api = useApiServiceClient();
    const [page, setPage] = useState(1);
    const [tutors, refetch] = useClientQuery<{tutors: User[], total: number}>(() =>
        api.getTutors(page),
        [page]
    );
    const [createModalOpen, setCreateModalOpen] = useState(false);
    const {t} = useTranslation('common');

    const cols: EntityListCol[] = [
        {
            field: "id",
            label: t('cols.id'),
        },
        {
            field: "username",
            label: t('cols.username'),
        },
    ];

    return (
        <Container fluid>
            <Group>
                <Title>{t('tutors')}</Title>
                <Button onClick={() => setCreateModalOpen(true)}>{t('actions.create-tutor')}</Button>
            </Group>
            <EntityList cols={cols} rows={tutors?.tutors ?? []} />
            <Pagination total={Math.ceil((tutors?.total ?? 0) / 50)} value={page} onChange={setPage} />
            {createModalOpen && (
                <CreateTutorModal onClose={() => setCreateModalOpen(false)} refetch={refetch} />
            )}
        </Container>
    );
}

export default TutorsPage;
