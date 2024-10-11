import useCurrentUser from "@/hooks/useCurrentUser";
import { isGranted } from "@/service/auth";
import { routes } from "@/static/routes";

const useRoutes = () => {
  const { user } = useCurrentUser();

  return routes.filter((r) =>
    user !== null && r.authRoles ? isGranted(user!, r.authRoles) : true,
  );
};

export default useRoutes;
