import {
  SpotlightActionData,
  SpotlightActionGroupData,
} from "@mantine/spotlight";
import useRoutes from "@/hooks/useRoutes";
import { Route } from "@/static/routes";
import { useRouter } from "next/navigation";
import { IconColorFilter } from "@tabler/icons-react";
import { useMantineColorScheme } from "@mantine/core";
import { useTranslation } from "react-i18next";

export const useStaticRoutesActions = (): (
  | SpotlightActionGroupData
  | SpotlightActionData
)[] => {
  const routes = useRoutes();
  const router = useRouter();
  const { t } = useTranslation(["common", "routes"]);
  return [
    {
      group: t("spotlight.basic-routes"),
      actions: routes.map((route: Route) => ({
        id: `static-route-${route.path}`,
        label: t(`routes:${route.name}`),
        description: route.description,
        leftSection: route.icon,
        onClick: () => router.push(route.path),
      })),
    },
  ];
};

export const useStaticGeneralActions = (): (
  | SpotlightActionGroupData
  | SpotlightActionData
)[] => {
  const { colorScheme, setColorScheme } = useMantineColorScheme({
    keepTransitions: true,
  });
  const { t } = useTranslation("common");

  const toggleColorScheme = () => {
    switch (colorScheme) {
      case "dark":
        setColorScheme("light");
        break;
      case "light":
        setColorScheme("dark");
        break;
      case "auto":
        // Because dark is just better
        setColorScheme("dark");
        break;
    }
  };

  return [
    {
      group: t("spotlight.general-actions"),
      actions: [
        {
          id: "general-switch-color-scheme",
          label: t("spotlight.color-scheme-title"),
          description: t("spotlight.color-scheme-description"),
          leftSection: <IconColorFilter />,
          onClick: toggleColorScheme,
        },
      ],
    },
  ];
};
