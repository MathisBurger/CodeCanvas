
export enum UserRoles {
    Student='ROLE_STUDENT',
    Admin='ROLE_ADMIN',
    Tutor='ROLE_TUTOR',
}

export interface User {
    id: number;
    createdAt: string;
    updatedAt: string;
    deletedAt?: string;
    username: string;
    email: string;
    roles: UserRoles[];
}

export interface GetStudentsResponse {
    students: User[];
}
