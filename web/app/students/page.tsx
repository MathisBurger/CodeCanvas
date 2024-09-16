'use client';
import {useEffect, useState} from "react";
import {GetStudentsResponse, User} from "@/service/types/usernator";
import useApiService from "@/hooks/useApiService";
import {Container, Title} from "@mantine/core";
import EntityList, {EntityListCol} from "@/components/EntityList";


const StudentsPage = () => {

    const api = useApiService();
    const [students, setStudents] = useState<User[]>([]);

    useEffect(() => {
        api.getStudents().then((response) => setStudents((response as GetStudentsResponse).students));
    }, []);

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
        <Container fluid>
            <Title>Students</Title>
            <EntityList cols={cols} rows={students} />
        </Container>
    )
}

export default StudentsPage;
