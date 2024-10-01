import {FileStructureTree} from "@/components/FileStructure";

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

export interface GroupJoinRequest {
    id: number;
    requestor: TaskyUser;
    group_id: number;
}

export interface GroupJoinRequestResponse {
    requests: GroupJoinRequest[];
}

export enum AssignmentLanguage {
    QuestionBased = 'QuestionBased',
    Java = "Java",
    Golang = "Golang",
}

export interface Assignment {
    id: number;
    title: string;
    due_date: string;
    description: string;
    language: AssignmentLanguage;
    file_structure: FileStructureTree|null;
    completed_by: TaskyUser[];
    runner_cpu: string;
    runner_memory: string;
    runner_timeout: string;
}

export interface AssignmentsResponse {
    assignments: Assignment[];
}

export interface RunnerConfig {
    runner_cpu: string;
    runner_memory: string;
    runner_timeout: string;
}
