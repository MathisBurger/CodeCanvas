'use client';
import {Box, Button, Group, Image} from "@mantine/core"
import Link from "next/link";
import useApiService from "@/hooks/useApiService";
import {useEffect, useState} from "react";


const Header = () => {

    const api = useApiService();
    const [loggedIn, setLoggedIn] = useState(false);

    useEffect(() => {
        api.self()
            .then((res) => {
                console.log(res);
                setLoggedIn(true);
            })
            .catch(() => setLoggedIn(false))
    }, [api])

    return (
        <Box pb={120} pr={20}>
            <header>
                <Group justify="space-between" h="100%">
                    <Image src="/CodeCanvas.png" h={100} alt="CompanyLogo" />
                    {!loggedIn && (
                        <Group visibleFrom="sm">
                            <Link href="/login">
                                <Button variant="default">Log in</Button>
                            </Link>
                            <Link href="/register">
                                <Button>Sign up</Button>
                            </Link>
                        </Group>
                    )}
                </Group>
            </header>
        </Box>
    );
}

export default Header;
