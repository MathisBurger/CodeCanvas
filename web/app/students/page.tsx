"use client";
import { GetStudentsResponse } from "@/service/types/usernator";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {Container, Pagination, Title} from "@mantine/core";
import EntityList, { EntityListCol } from "@/components/EntityList";
import useClientQuery from "@/hooks/useClientQuery";
import {useState} from "react";

const StudentsPage = () => {
  const api = useApiServiceClient();
  const [page, setPage] = useState(1);
  const [students] = useClientQuery<GetStudentsResponse>(() =>
    api.getStudents(page),
      [page]
  );

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
      <Title>Students</Title>
      <EntityList cols={cols} rows={students?.students ?? []} />
      <Pagination total={Math.ceil((students?.total ?? 0) / 50)} value={page} onChange={setPage} />
    </Container>
  );
};

export default StudentsPage;
