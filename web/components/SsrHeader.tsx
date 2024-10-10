"use client";
import { Avatar, Button, Group, Kbd, Menu, Text } from "@mantine/core";
import Link from "next/link";
import { User } from "@/service/types/usernator";
import { useCookies } from "react-cookie";
import { useRouter } from "next/navigation";
import useCurrentUser from "@/hooks/useCurrentUser";

interface SsrHeaderProps {
  user: User | null;
}

const SsrHeader: React.FC<SsrHeaderProps> = ({ user }) => {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [s, _, removeSession] = useCookies(["session"]);
  const { setUser } = useCurrentUser();
  const router = useRouter();

  const logOut = () => {
    removeSession("session");
    router.push("/login");
    setUser(null);
    //window.location.reload();
  };

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
    <Group>
      <Group>
        <Text>Spotlight actions:</Text>
        <div dir="ltr">
          <Kbd>CMD</Kbd> + <Kbd>K</Kbd> or <Kbd>CTRL</Kbd> + <Kbd>K</Kbd>
        </div>
      </Group>
      <Menu>
        <Menu.Target>
          <Avatar name={user.username} color="initials" />
        </Menu.Target>
        <Menu.Dropdown>
          <Menu.Item onClick={() => router.push("/settings")}>
            Settings
          </Menu.Item>
          <Menu.Item color="red" onClick={logOut}>
            Log out
          </Menu.Item>
        </Menu.Dropdown>
      </Menu>
    </Group>
  );
};

export default SsrHeader;
