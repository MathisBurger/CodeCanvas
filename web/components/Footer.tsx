import useCurrentUser from "@/hooks/useCurrentUser";
import {Group, Paper, Text} from "@mantine/core";
import {useRouter} from "next/navigation";

const Footer = () => {

    const router = useRouter();
    const {user} = useCurrentUser();

    return (
        <Paper m="sm">
            <Group justify="flex-end">
                <Text c="dimmed" style={{cursor: 'pointer'}} onClick={() => router.push('/impress')}>Impress</Text>
                <Text c="dimmed" style={{cursor: 'pointer'}} onClick={() => router.push('/privacy')}>Privacy</Text>
                {user && (
                    <Text c="dimmed" style={{cursor: 'pointer'}} onClick={() => router.push('/report-bug')}>Report Bug</Text>
                )}
            </Group>
        </Paper>
    );
}

export default Footer;
