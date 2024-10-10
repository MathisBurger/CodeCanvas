import { Center, Container, Loader } from "@mantine/core";

const CentralLoading = () => {
  return (
    <Container fluid>
      <Center style={{ minHeight: "80vh" }}>
        <Loader color="indigo" type="bars" size="xl" />
      </Center>
    </Container>
  );
};

export default CentralLoading;
