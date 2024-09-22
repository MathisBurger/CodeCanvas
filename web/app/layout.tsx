'use client';
import React, {useMemo, useState} from "react";
import {AppShell, ColorSchemeScript, MantineProvider} from '@mantine/core';
import '@mantine/core/styles.css';
import Header from "@/components/Header";
import {Notifications} from "@mantine/notifications";
import {usePathname} from "next/navigation";
import Navbar from "@/components/Navbar";
import { User } from "@/service/types/usernator";
import { CurrentUserContext } from "@/hooks/useCurrentUser";
import '@mantine/core/styles.css';

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {

    const [user, setUser] = useState<User | null>(null);
    const pathname = usePathname();
    const showNavbar = useMemo(() => pathname !== "/login" && pathname !== "/register" && pathname !== "/", [pathname]);

  return (
    <html lang="en">
    <head>
        <meta charSet="UTF-8"/>
        <meta
            name="viewport"
            content="width=device-width, initial-scale=1.0"
        />
        <title>CodeCanvas</title>

        <ColorSchemeScript />
    </head>
    <body>
    <CurrentUserContext.Provider value={{user, setUser}}>
            <MantineProvider theme={{}}>
                <Notifications />
                {/* @ts-ignore */}
                <AppShell header={{height: 100}} navbar={showNavbar ? {width: 250} : undefined}>
                    <AppShell.Header><Header /></AppShell.Header>
                    {showNavbar && (<AppShell.Navbar><Navbar /></AppShell.Navbar>)}
                    <AppShell.Main>{children}</AppShell.Main>
                </AppShell>
            </MantineProvider>
    </CurrentUserContext.Provider>
    </body>
    </html>
  );
}
