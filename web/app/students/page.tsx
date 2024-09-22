'use server';
import {GetStudentsResponse} from "@/service/types/usernator";
import useApiService from "@/hooks/useApiService";
import {Container, Title} from "@mantine/core";
import EntityList, {EntityListCol} from "@/components/EntityList";


const StudentsPage = async () => {

    const api = useApiService();
    const students = (await api.getStudents() as GetStudentsResponse).students;


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
