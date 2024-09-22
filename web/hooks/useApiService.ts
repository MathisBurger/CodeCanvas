'use server';
import ApiService from "@/service/ApiService";
import {cookies} from "next/headers";


const useApiService = () => {
    const cookieStore = cookies();
    const cookie = cookieStore.get("session");
    if (cookie === undefined) {
        return new ApiService("");
    }
    return new ApiService(cookie?.value);
}

export default useApiService;
