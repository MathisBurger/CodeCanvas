'use client';
import {Avatar, Button, Group, Menu} from "@mantine/core";
import Link from "next/link";
import {User} from "@/service/types/usernator";
import {useCookies} from "react-cookie";
import useCurrentUser from "@/hooks/useCurrentUser";
import {useEffect} from "react";

interface SsrHeaderProps {
    newUser: User|null;
}

const SsrHeader: React.FC<SsrHeaderProps> = ({newUser}) => {
    const [s, _, removeSession] = useCookies(['session']);
    const {user, setUser} = useCurrentUser();

    const logOut = () => {
        removeSession('session');
        //router.push("/");
        //window.location.reload();
    }
/*
    useEffect(() => {
        setUser(newUser);
    }, [newUser]);
*/
    if (user === null) {
        return (
            <Group visibleFrom="sm">
                <Link href="/login">
                    <Button variant="default">Log in</Button>
                </Link>
                <Link href="/register">
                    <Button>Sign up</Button>
                </Link>
            </Group>
        );
    }
    return (
        <Menu>
            <Menu.Target>
                <Avatar name={user.username} color="initials" />
            </Menu.Target>
            <Menu.Dropdown>
                <Menu.Item color="red" onClick={logOut}>
                    Log out
                </Menu.Item>
            </Menu.Dropdown>
        </Menu>
    )
}

export default SsrHeader;
