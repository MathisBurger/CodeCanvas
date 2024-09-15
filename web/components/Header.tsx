'use client';
import {Avatar, Box, Button, Group, Image} from "@mantine/core"
import Link from "next/link";
import useApiService from "@/hooks/useApiService";
import {useEffect} from "react";
import useCurrentUser from "@/hooks/useCurrentUser";
import {User} from "@/service/types/usernator";
import {usePathname, useRouter} from "next/navigation";


const Header = () => {

    const api = useApiService();
    const {user, setUser} = useCurrentUser();
    const router = useRouter();
    const pathname = usePathname();

    useEffect(() => {
        api.self()
            .then((res) => {
                setUser(res as User);
            })
            .catch(() => {
                setUser(null);
                if (pathname !== "/login" && pathname !== "/register" && pathname !== "/") {
                    router.push("/login");
                }
            })
    }, [pathname])

    return (
        <Box pr={20}>
            <header>
                <Group justify="space-between" h="100%">
                    <Image src="/CodeCanvas.png" h={100} alt="CompanyLogo" />
                    {user === null ? (
                        <Group visibleFrom="sm">
                            <Link href="/login">
                                <Button variant="default">Log in</Button>
                            </Link>
                            <Link href="/register">
                                <Button>Sign up</Button>
                            </Link>
                        </Group>
                    ) : (
                        <Avatar name={user.username} color="initials" />
                    )}
                </Group>
            </header>
        </Box>
    );
}

export default Header;
