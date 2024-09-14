import {Box, Button, Drawer, Group, Image} from "@mantine/core"


const Header = () => {

    return (
        <Box pb={120} pr={20}>
            <header>
                <Group justify="space-between" h="100%">
                    <Image src="/CodeCanvas.png" h={100} />
                    <Group visibleFrom="sm">
                        <Button variant="default">Log in</Button>
                        <Button>Sign up</Button>
                    </Group>
                </Group>
            </header>
        </Box>
    );
}

export default Header;
