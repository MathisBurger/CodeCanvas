'use client';
import React, {useState} from "react";
import {AppShell, ColorSchemeScript, createTheme, Group, MantineProvider} from '@mantine/core';
import '@mantine/core/styles.css';
import Header from "@/components/Header";
import {Notifications} from "@mantine/notifications";
import {User} from "@/service/types/usernator";
import {CurrentUserContext} from "@/hooks/useCurrentUser";
import {usePathname} from "next/navigation";
import Navbar from "@/components/Navbar";



const theme = createTheme({
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {

    const [user, setUser] = useState<User | null>(null);

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
            <MantineProvider theme={theme}>
                <Notifications />
                <AppShell header={{height: 100}} navbar={{width: 250}}>
                    <AppShell.Header><Header /></AppShell.Header>
                    <AppShell.Navbar><Navbar /></AppShell.Navbar>
                    <AppShell.Main>{children}</AppShell.Main>
                </AppShell>
            </MantineProvider>
        </CurrentUserContext.Provider>
    </body>
    </html>
  );
}
