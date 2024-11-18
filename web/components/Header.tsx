"use client";
import { Box, Group, Image } from "@mantine/core";
import SsrHeader from "@/components/SsrHeader";
import useCurrentUser from "@/hooks/useCurrentUser";
import { usePathname, useRouter } from "next/navigation";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { User } from "@/service/types/usernator";
import { useEffect } from "react";
import { publicRoutes } from "@/static/routes";
import Link from "next/link";

const Header = () => {
  const api = useApiServiceClient();
  const { user, setUser } = useCurrentUser();
  const pathname = usePathname();
  const router = useRouter();

  useEffect(() => {
    api
      .self()
      .then((res) => {
        setUser(res as User);
      })
      .catch(() => {
        setUser(null);
        if (publicRoutes.indexOf(pathname) === -1) {
          router.push("/login");
        }
      });
  }, [pathname]);

  return (
    <Box pr={20}>
      <header>
        <Group justify="space-between" h="100%">
          <Link href="/">
            <Image
                lightHidden
                src="/CodeCanvas-dark.png"
                h={100}
                alt="CompanyLogo"
            />
            <Image darkHidden src="/CodeCanvas.png" h={100} alt="CompanyLogo" />
          </Link>
          <SsrHeader user={user} />
        </Group>
      </header>
    </Box>
  );
};

export default Header;
