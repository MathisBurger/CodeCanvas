
export interface GroupsResponse {
    groups: MinifiedGroup[];
}

export interface MinifiedGroup {
    id: number;
    title: string;
    member_count: number;
    tutor: TaskyUser;
}

export interface TaskyUser {
    id: number;
    username: string;
    email: string;
}
