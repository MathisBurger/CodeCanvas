'use client';
import {Button, Stack} from "@mantine/core";
import {usePathname, useRouter} from "next/navigation";
import {IconDashboard} from "@tabler/icons-react";

interface Route {
    path: string;
    name: string;
    icon?: JSX.Element;
}

const routes: Route[] = [
    {
        path: '/dashboard',
        name: 'Dashboard',
        icon: <IconDashboard />
    }
]


const Navbar = () => {

    const router = useRouter();
    const pathname = usePathname();

    return (
        <Stack gap="xs" m={5}>
            {routes.map((route: Route) => (
                <Button
                    key={route.path}
                    variant={pathname === route.path ? "filled" : "default"}
                    onClick={() => router.push(route.path)}
                    leftSection={route.icon}
                >
                    {route.name}
                </Button>
            ))}
        </Stack>
    );
}

export default Navbar;
