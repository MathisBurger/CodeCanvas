import {UserRoles} from "@/service/types/usernator";
import {IconDashboard, IconFile, IconSchool, IconUsersGroup,} from "@tabler/icons-react";

export interface Route {
  path: string;
  name: string;
  icon?: JSX.Element;
  description: string;
  authRoles?: UserRoles[];
}

export const routes: Route[] = [
  {
    path: "/dashboard",
    name: "Dashboard",
    description: "The local dashboard",
    icon: <IconDashboard />,
  },
  {
    path: "/students",
    name: "Students",
    description: "All students in the system",
    icon: <IconSchool />,
    authRoles: [UserRoles.Tutor, UserRoles.Admin],
  },
  {
    path: "/my-groups",
    name: "My Groups",
    description: "All groups you are a member of",
    icon: <IconUsersGroup />,
    authRoles: [UserRoles.Tutor, UserRoles.Student],
  },
  {
    path: "/groups",
    name: "Groups",
    description:
      "All groups you are not a member of that you can join or access",
    icon: <IconUsersGroup />,
    authRoles: [UserRoles.Student, UserRoles.Admin],
  },
  {
    path: "/solutions",
    name: "Solutions",
    description: "All your personal solutions",
    icon: <IconFile />,
    authRoles: [UserRoles.Student],
  },
];

export const publicRoutes = ["/login", "/register", "/", "/impress", "/privacy"];
