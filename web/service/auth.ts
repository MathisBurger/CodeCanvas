import { User, UserRoles } from "@/service/types/usernator";

/**
 * Checks if a user is granted to perform an action
 *
 * @param user The user
 * @param roles The roles required for this action
 */
export const isGranted = (user: User | null, roles: UserRoles[]): boolean => {
  if (null === user) {
    return false;
  }
  const all = [...user.roles, ...roles];
  return new Set(all).size !== all.length;
};
