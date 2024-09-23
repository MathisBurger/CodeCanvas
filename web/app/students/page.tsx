'use server';
import {GetStudentsResponse} from "@/service/types/usernator";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {Container, Title} from "@mantine/core";
import EntityList, {EntityListCol} from "@/components/EntityList";
import useClientQuery from "@/hooks/useClientQuery";


const StudentsPage = () => {

    const api = useApiServiceClient();
    const students = useClientQuery<GetStudentsResponse|string>(() => api.getStudents()) as GetStudentsResponse;


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
            <EntityList cols={cols} rows={students.students} />
        </Container>
    )
}

export default StudentsPage;
