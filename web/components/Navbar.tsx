'use client';
import {Button, Stack} from "@mantine/core";
import {usePathname, useRouter} from "next/navigation";
import {IconDashboard, IconSchool} from "@tabler/icons-react";
import {UserRoles} from "@/service/types/usernator";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";

interface Route {
    path: string;
    name: string;
    icon?: JSX.Element;
    authRoles?: UserRoles[];
}

const routes: Route[] = [
    {
        path: '/dashboard',
        name: 'Dashboard',
        icon: <IconDashboard />
    },
    {
        path: '/students',
        name: 'Students',
        icon: <IconSchool />,
        authRoles: [UserRoles.Tutor, UserRoles.Admin]
    }
]


const Navbar = () => {

    const router = useRouter();
    const pathname = usePathname();
    const {user} = useCurrentUser();

    return (
        <Stack gap="xs" m={5}>
            {routes.filter((r) => user !== null && r.authRoles ? isGranted(user!, r.authRoles) : true).map((route: Route) => (
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
