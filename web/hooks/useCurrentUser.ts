'use client';
import { User } from "@/service/types/usernator";
import {createContext, useContext} from "react";

interface UserContext {
    user: User|null;
    setUser: (user: User|null) => void;
}

export const CurrentUserContext = createContext<UserContext>({
    user: null,
    setUser: () => {},
});

const useCurrentUser = () => useContext(CurrentUserContext);

export default useCurrentUser;
