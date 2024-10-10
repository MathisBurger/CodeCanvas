import {
  SpotlightActionData,
  SpotlightActionGroupData,
} from "@mantine/spotlight";
import useRoutes from "@/hooks/useRoutes";
import { Route } from "@/static/routes";
import { useRouter } from "next/navigation";
import { IconColorFilter } from "@tabler/icons-react";
import { useMantineColorScheme } from "@mantine/core";

export const useStaticRoutesActions = (): (
  | SpotlightActionGroupData
  | SpotlightActionData
)[] => {
  const routes = useRoutes();
  const router = useRouter();
  return [
    {
      group: "Basic routes",
      actions: routes.map((route: Route) => ({
        id: `static-route-${route.path}`,
        label: route.name,
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
      group: "General actions",
      actions: [
        {
          id: "general-switch-color-scheme",
          label: "Toggle color scheme",
          description: "Toggles the color scheme (light/dark)",
          leftSection: <IconColorFilter />,
          onClick: toggleColorScheme,
        },
      ],
    },
  ];
};
