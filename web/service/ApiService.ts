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
    public async registerUser(username: string, password: string): Promise<RegisterUserResponse|string> {
        return await this.post<RegisterUserResponse>("/usernator/register", { username, password });
    }

    /**
     * Logs in a user
     *
     * @param username The username of the user
     * @param password The password of the user
     * @throws ApiError The api error
     */
    public async loginUser(username: string, password: string): Promise<any> {
        return await this.post<any>("/usernator/login", { username, password });
    }

    /**
     * Executes a general post request
     *
     * @param path The path
     * @param body The json body
     * @throws ApiError The api error
     * @private
     */
    private async post<T>(path: string, body: any): Promise<T|string> {
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
            const txt = await resp.text();
            try {
                return JSON.parse(txt);
            } catch (_) {
                return txt;
            }
        } catch (e) {
            if(e instanceof Error) {
                throw new ApiError(-1, e.message);
            }
            throw new ApiError(-1, `${e}`);
        }
    }
}

export default ApiService;
