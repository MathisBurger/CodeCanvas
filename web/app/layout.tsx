'use client';
import React, {useState} from "react";
import {ColorSchemeScript, createTheme, MantineProvider} from '@mantine/core';
import '@mantine/core/styles.css';
import Header from "@/components/Header";
import {Notifications} from "@mantine/notifications";
import {User} from "@/service/types/usernator";
import {CurrentUserContext} from "@/hooks/useCurrentUser";
import {usePathname} from "next/navigation";



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
                <Header />
                {children}
            </MantineProvider>
        </CurrentUserContext.Provider>
    </body>
    </html>
  );
}
