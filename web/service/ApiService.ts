import {GetStudentsResponse, User} from "@/service/types/usernator";
import ApiError from "@/service/types/error";
import {Group, GroupJoinRequestResponse, GroupsResponse} from "@/service/types/tasky";

class ApiService {

    private apiUrl: string;
    private session: string|undefined = undefined;

    constructor(session: string|undefined = undefined) {
        this.apiUrl = process.env.API_URL ?? "http://localhost:3002";
        this.session = session;
    }

    /**
     * The logged-in user
     */
    public async self(): Promise<User|string> {
        return await this.get<User>("/usernator/self");
    }

    /**
     * Registers a user
     *
     * @param username The username of the user
     * @param password The password of the user
     * @throws ApiError The api error
     */
    public async registerUser(username: string, password: string): Promise<User|string> {
        return await this.post<User>("/usernator/register", { username, password });
    }

    /**
     * Logs in a user
     *
     * @param username The username of the user
     * @param password The password of the user
     * @throws ApiError The api error
     */
    public async loginUser(username: string, password: string): Promise<string> {
        return await this.post<never>("/usernator/login", { username, password });
    }

    /**
     * Gets all students
     */
    public async getStudents(): Promise<GetStudentsResponse|string> {
        return await this.get<GetStudentsResponse>("/usernator/all-students");
    }

    /**
     * Gets all groups
     */
    public async getGroups(): Promise<GroupsResponse|string> {
        return await this.get<GroupsResponse>("/tasky/groups");
    }

    /**
     * Gets a specific group
     *
     * @param id The ID of the group
     */
    public async getGroup(id: number): Promise<Group|string> {
        return await this.get<Group>("/tasky/groups/" + id);
    }

    public async getGroupJoinRequests(id: number): Promise<GroupJoinRequestResponse|string> {
        return await this.get<GroupJoinRequestResponse>(`/tasky/groups/${id}/join_requests`);
    }

    public async approveGroupJoinRequest(groupId: number, id: number): Promise<Group|string> {
        return await this.get<Group>(`/tasky/groups/${groupId}/join_requests/${id}/approve`);
    }

    public async rejectGroupJoinRequest(groupId: number, id: number): Promise<Group|string> {
        return await this.get<Group>(`/tasky/groups/${groupId}/join_requests/${id}/reject`);
    }

    /**
     * Executes a general get request
     *
     * @param path The path
     * @throws ApiError The api error
     * @private
     */
    private async get<T>(path: string): Promise<T|string> {
        return await this.fetch<T>(path, "GET", undefined);
    }

    /**
     * Executes a general post request
     *
     * @param path The path
     * @param body The json body
     * @throws ApiError The api error
     * @private
     */
    private async post<T>(path: string, body: object): Promise<T|string> {
        return await this.fetch<T>(path, "POST", body);
    }

    /**
     * Executes a generic HTTP fetch
     *
     * @param path The path
     * @param method The method
     * @param body The body if exists
     * @private
     */
    private async fetch<T>(path: string, method: string, body?: object): Promise<T|string> {
        try {
            const headers = {
                "Content-Type": "application/json",
                "Accept": "*/*",
            };
            if (this.session) {
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-expect-error
                headers["Cookie"] = `session=${this.session}`;
            }
            const resp = await fetch(`${this.apiUrl}${path}`, {
                method,
                mode: "cors",
                body: body ? JSON.stringify(body) : undefined,
                credentials: 'include',
                headers: headers
            });
            const txt = await resp.text();
            const obj = this.getObject(txt);
            if (resp.status !== 200) {
                throw new ApiError(resp.status, typeof obj == "object" ? obj.message : obj);
            }
            return obj;

        } catch (e) {
            if(e instanceof Error) {
                throw new ApiError(-1, e.message);
            }
            throw new ApiError(-1, `${e}`);
        }
    }

    private getObject(text: string): any|string {
        try {
            return JSON.parse(text);
        } catch (_) {
            return text;
        }
    }
}

export default ApiService;
