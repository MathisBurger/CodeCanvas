'use client';
import useRoutes from "@/hooks/useRoutes";
import {Button, Stack} from "@mantine/core";
import {usePathname, useRouter} from "next/navigation";
import { Route } from "@/static/routes";


const Navbar = () => {

    const router = useRouter();
    const pathname = usePathname();
    const routes = useRoutes();

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
