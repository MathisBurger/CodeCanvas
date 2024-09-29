import {GetStudentsResponse, User} from "@/service/types/usernator";
import ApiError from "@/service/types/error";
import {
    Assignment,
    AssignmentLanguage, AssignmentsResponse,
    Group,
    GroupJoinRequest,
    GroupJoinRequestResponse,
    GroupsResponse
} from "@/service/types/tasky";
import dayjs from "dayjs";

export interface GenericMessage {
    message: string;
}

class ApiService {

    private apiUrl: string;

    constructor() {
        this.apiUrl = process.env.API_URL ?? "http://localhost:3002";
    }

    public async self(): Promise<User|string> {
        return await this.get<User>("/usernator/self");
    }

    public async registerUser(username: string, password: string): Promise<User> {
        return await this.post<User>("/usernator/register", { username, password });
    }

    public async loginUser(username: string, password: string): Promise<GenericMessage> {
        return await this.post<GenericMessage>("/usernator/login", { username, password });
    }

    public async getStudents(): Promise<GetStudentsResponse> {
        return await this.get<GetStudentsResponse>("/usernator/all-students");
    }

    public async getGroups(): Promise<GroupsResponse> {
        return await this.get<GroupsResponse>("/tasky/groups");
    }

    public async getMyGroups(): Promise<GroupsResponse> {
        return await this.get<GroupsResponse>("/tasky/my_groups");
    }

    public async getGroup(id: number): Promise<Group> {
        return await this.get<Group>("/tasky/groups/" + id);
    }

    public async getGroupJoinRequests(id: number): Promise<GroupJoinRequestResponse> {
        return await this.get<GroupJoinRequestResponse>(`/tasky/groups/${id}/join_requests`);
    }

    public async createGroupJoinRequest(id: number): Promise<GroupJoinRequest> {
        return await this.post<GroupJoinRequest>(`/tasky/groups/${id}/create_join_request`, {});
    }

    public async approveGroupJoinRequest(groupId: number, id: number): Promise<Group> {
        return await this.post<Group>(`/tasky/groups/${groupId}/join_requests/${id}/approve`, {});
    }

    public async rejectGroupJoinRequest(groupId: number, id: number): Promise<Group> {
        return await this.post<Group>(`/tasky/groups/${groupId}/join_requests/${id}/reject`, {});
    }

    public async createAssignment(groupId: number, title: string, due_date: Date, description: string, language: AssignmentLanguage): Promise<Assignment> {
        return await this.post<Assignment>(`/tasky/groups/${groupId}/assignments`, {title, due_date, description, language});
    }

    public async getAssignmentsForGroup(id: number): Promise<AssignmentsResponse> {
        return await this.get<AssignmentsResponse>(`/tasky/groups/${id}/assignments`)
    }

    public async getAssignmentForGroup(groupId: number, assignmentId: number): Promise<Assignment> {
        return await this.get<Assignment>(`/tasky/groups/${groupId}/assignments/${assignmentId}`);
    }

    public async updateAssignment(groupId: number, assignmentId: number, title: string, due_date: Date, description: string): Promise<Assignment> {
        return await this.post<Assignment>(`/tasky/groups/${groupId}/assignments/${assignmentId}/update`, {title, due_date, description});
    }

    /**
     * Executes a general get request
     *
     * @param path The path
     * @throws ApiError The api error
     * @private
     */
    private async get<T>(path: string): Promise<T> {
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
    private async post<T>(path: string, body: object): Promise<T> {
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
    private async fetch<T>(path: string, method: string, body?: object): Promise<T> {
        try {
            const resp = await fetch(`${this.apiUrl}${path}`, {
                method,
                mode: "cors",
                body: body ? JSON.stringify(body) : undefined,
                credentials: 'include',
                headers: {
                    "Content-Type": "application/json",
                    "Accept": "*/*",
                }
            });
            const txt = await resp.text();
            const obj = this.getObject(txt);
            if (resp.status !== 200) {
                throw new ApiError(resp.status, obj.message);
            }
            return obj;

        } catch (e) {
            if(e instanceof Error) {
                throw new ApiError(-1, e.message);
            }
            throw new ApiError(-1, `${e}`);
        }
    }

    private getObject(text: string): any|GenericMessage {
        try {
            return JSON.parse(text);
        } catch (_) {
            return {message: text} as GenericMessage;
        }
    }
}

export default ApiService;
