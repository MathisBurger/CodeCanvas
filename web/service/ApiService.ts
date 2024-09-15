import {RegisterUserResponse} from "@/service/types/usernator";
import ApiError from "@/service/types/error";

class ApiService {

    private apiUrl: string;

    constructor() {
        this.apiUrl = process.env.API_URL ?? "http://localhost:3002";
    }

    /**
     * Registers a user
     *
     * @param username The username of the user
     * @param password The password of the user
     * @throws ApiError The api error
     */
    public async registerUser(username: string, password: string): Promise<RegisterUserResponse> {
        return await this.post<RegisterUserResponse>("/usernator/register", { username, password });
    }

    /**
     * Executes a general post request
     *
     * @param path The path
     * @param body The json body
     * @throws ApiError The api error
     * @private
     */
    private async post<T>(path: string, body: any): Promise<T> {
        try {
            const resp = await fetch(`${this.apiUrl}${path}`, {
                method: "POST",
                mode: "cors",
                body: JSON.stringify(body),
                headers: {
                    "Content-Type": "application/json",
                    "Accept": "*/*",
                }
            });
            return await resp.json() as T;
        } catch (e) {
            if(e instanceof Error) {
                throw new ApiError(-1, e.message);
            }
            throw new ApiError(-1, `${e}`);
        }
    }
}

export default ApiService;
