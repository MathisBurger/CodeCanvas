"use client";
import useRoutes from "@/hooks/useRoutes";
import { Button, Stack } from "@mantine/core";
import { usePathname, useRouter } from "next/navigation";
import { Route } from "@/static/routes";
import {useTranslation} from "react-i18next";

const Navbar = () => {
  const router = useRouter();
  const pathname = usePathname();
  const routes = useRoutes();
  const {t} = useTranslation('routes');

  return (
    <Stack gap="xs" m={5}>
      {routes.map((route: Route) => (
        <Button
          key={route.path}
          variant={pathname === route.path ? "filled" : "default"}
          onClick={() => router.push(route.path)}
          leftSection={route.icon}
        >
          {t(route.name)}
        </Button>
      ))}
    </Stack>
  );
};

export default Navbar;
