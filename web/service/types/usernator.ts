

export interface RegisterUserResponse {
    id: number;
    createdAt: string;
    updatedAt: string;
    deletedAt?: string;
    username: string;
    email: string;
    roles: string[];
}
