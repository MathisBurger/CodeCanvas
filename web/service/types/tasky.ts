
export interface GroupsResponse {
    groups: MinifiedGroup[];
}

export interface MinifiedGroup {
    id: number;
    title: string;
    member_count: number;
    tutor: TaskyUser;
}

export interface Group {
    id: number;
    title: string;
    members: TaskyUser[];
    tutor: TaskyUser;
    request_count: number;
}

export interface TaskyUser {
    id: number;
    username: string;
    email: string;
}
