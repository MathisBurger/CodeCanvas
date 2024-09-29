'use client';
import { useRouter } from "next/navigation";
import {Button, Group} from "@mantine/core";


const NavigateBack = () => {

    const router = useRouter();

    return (
        <Group justify="start" mt={30}>
            <Button color="blue" onClick={() => router.back()}>Navigate Back</Button>
        </Group>
    );
}

export default NavigateBack;
