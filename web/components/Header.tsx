import {Box, Button, Group, Image} from "@mantine/core"
import Link from "next/link";


const Header = () => {

    return (
        <Box pb={120} pr={20}>
            <header>
                <Group justify="space-between" h="100%">
                    <Image src="/CodeCanvas.png" h={100} />
                    <Group visibleFrom="sm">
                        <Button variant="default">Log in</Button>
                        <Link href="/register">
                            <Button>Sign up</Button>
                        </Link>
                    </Group>
                </Group>
            </header>
        </Box>
    );
}

export default Header;
