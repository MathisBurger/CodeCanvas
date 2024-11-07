"use client";
import React, {useEffect, useMemo, useState} from "react";
import { AppShell, ColorSchemeScript, MantineProvider } from "@mantine/core";
import "@mantine/core/styles.css";
import Header from "@/components/Header";
import { Notifications } from "@mantine/notifications";
import { usePathname } from "next/navigation";
import Navbar from "@/components/Navbar";
import { User } from "@/service/types/usernator";
import { CurrentUserContext } from "@/hooks/useCurrentUser";
import "@mantine/core/styles.css";
import "@mantine/tiptap/styles.css";
import "@mantine/dates/styles.css";
import "@mantine/notifications/styles.css";
import "@mantine/spotlight/styles.css";
import "@mantine/code-highlight/styles.css";
import "@mantine/dropzone/styles.css";
import { DatesProvider } from "@mantine/dates";
import SpotlightWrapper from "@/components/spotlight/SpotlightWrapper";
import Footer from "@/components/Footer";
import { publicRoutes } from "@/static/routes";
import Stage2SpotlightContextWrapper from "@/components/spotlight/Stage2SpotlightContextWrapper";
import {Stage2Type} from "@/hooks/spotlight/stage2";
import i18n from "../i18n"
import CentralLoading from "@/components/CentralLoading";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const pathname = usePathname();
  const showNavbar = useMemo(
    () => publicRoutes.indexOf(pathname) === -1,
    [pathname],
  );

  /**
   * Removes dangling spotlight2 entries
   */
  useEffect(() => {
    const data = localStorage.getItem("spotlight-stage2");
    if (data) {
      try {
        const json: Stage2Type = JSON.parse(data);
        const now = new Date().getTime();
        json.groups = json.groups.filter((g) => g.die?.getTime() < now);
        json.assignments = json.assignments.filter((g) => g.die?.getTime() < now);
        json.solutions = json.solutions.filter((g) => g.die?.getTime() < now);
        localStorage.setItem("spotlight-stage2", JSON.stringify(json));
      } catch (e) {
        console.error(e);
      }
    }
    i18n.init();
    setLoading(false);
  }, []);

  return (
    <html lang="en">
    <head>
      <meta charSet="UTF-8"/>
      <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
      <link rel="icon" href="/favicon.ico" type="image/x-icon"/>
      <title>CodeCanvas</title>

      <ColorSchemeScript/>
    </head>
    <body>
    <CurrentUserContext.Provider value={{ user, setUser }}>
          <MantineProvider theme={{}}>
            <DatesProvider settings={{ timezone: null }}>
              {loading ? (
                  <CentralLoading />
              ) : (
                  <Stage2SpotlightContextWrapper>
                    <Notifications />
                    <AppShell
                        header={{ height: 100 }}
                        navbar={showNavbar ? { width: 250, breakpoint: "" } : undefined}
                    >
                      <AppShell.Header>
                        <Header />
                      </AppShell.Header>
                      {showNavbar && (
                          <AppShell.Navbar>
                            <Navbar />
                          </AppShell.Navbar>
                      )}
                      <AppShell.Main mb={100}>{children}</AppShell.Main>
                      <AppShell.Footer><Footer /></AppShell.Footer>
                    </AppShell>
                    <SpotlightWrapper />
                  </Stage2SpotlightContextWrapper>
              )}
            </DatesProvider>
          </MantineProvider>
        </CurrentUserContext.Provider>
      </body>
    </html>
  );
}
