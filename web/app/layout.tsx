import React from "react";
import {ColorSchemeScript, createTheme, MantineProvider} from '@mantine/core';
import '@mantine/core/styles.css';
import Header from "@/components/Header";

const theme = createTheme({
    /** Put your mantine theme override here */
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
    <head>
        <meta charSet="UTF-8"/>
        <meta
            name="viewport"
            content="width=device-width, initial-scale=1.0"
        />
        <title>My awesome app</title>

        <ColorSchemeScript />
    </head>
    <body>
    <MantineProvider theme={theme}>
        <Header />
        {children}
    </MantineProvider>
    </body>
    </html>
  );
}
