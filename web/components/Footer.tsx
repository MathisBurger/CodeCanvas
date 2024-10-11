import {Group, Paper, Text} from "@mantine/core";
import {useRouter} from "next/navigation";

const Footer = () => {

    const router = useRouter();

    return (
        <Paper m="sm">
            <Group justify="flex-end">
                <Text c="dimmed" style={{cursor: 'pointer'}} onClick={() => router.push('/impress')}>Impress</Text>
                <Text c="dimmed" style={{cursor: 'pointer'}} onClick={() => router.push('/privacy')}>Privacy</Text>
            </Group>
        </Paper>
    );
}

export default Footer;
