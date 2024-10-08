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
    runner_cmd: string;
}

export interface MongoTestFile {
    _id: {$oid: string};
    file_name: string;
    content_size: string;
    content: string;
    assignment_id: string;
}

export interface MongoTaskFile {
    _id: {$oid: string};
    file_name: string;
    content_size: string;
    content: string;
    solution_id: string;
}

export interface Job {
    id: string;
    execution: Execution[];
}

export interface Execution {
    state: string;
    result: string|null;
    error: string|null;
}

export interface Solution {
    id: number;
    submitter: TaskyUser;
    assignment: Assignment;
    approval_status?: string;
    file_structure?: FileStructureTree;
    job?: Job;
}

export interface SolutionsResponse {
    solutions: Solution[];
}

export interface SolutionFilesResponse {
    task_files: MongoTaskFile[];
    test_files: MongoTestFile[];
}
