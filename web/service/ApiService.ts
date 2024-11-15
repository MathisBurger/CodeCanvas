import { GetStudentsResponse, User } from "@/service/types/usernator";
import ApiError from "@/service/types/error";
import {
  Assignment,
  AssignmentLanguage,
  AssignmentsResponse,
  Group,
  GroupJoinRequest,
  GroupJoinRequestResponse,
  GroupsResponse,
  MongoTestFile,
  QuestionCatalogueElement,
  QuestionSolution,
  RunnerConfig,
  Solution,
  SolutionFilesResponse,
  SolutionsResponse,
  AssignmentWish,
  CodeComment,
  AssignmentWishesResponse,
  Notification, GroupJoinRequestPolicy, TaskyUser,
} from "@/service/types/tasky";
import { FileStructureTree } from "@/components/FileStructure";
import { Spotlight3Response } from "@/service/types/spotlight";

export interface GenericMessage {
  message: string;
}

class ApiService {
  private apiUrl: string;

  constructor() {
    this.apiUrl =
      process.env.NODE_ENV === "production"
        ? "https://api.code-canvas.app"
        : "http://localhost:3002";
  }

  public async self(): Promise<User | string> {
    return await this.get<User>("/usernator/self");
  }

  public async registerUser(username: string, password: string): Promise<User> {
    return await this.post<User>("/usernator/register", { username, password });
  }

  public async loginUser(
    username: string,
    password: string,
  ): Promise<GenericMessage> {
    return await this.post<GenericMessage>("/usernator/login", {
      username,
      password,
    });
  }

  public async getStudents(page?: number): Promise<GetStudentsResponse> {
    return await this.get<GetStudentsResponse>(
      `/usernator/all-students?page=${page}`,
    );
  }

  public async createGroup(title: string, join_policy: GroupJoinRequestPolicy): Promise<Group> {
    return await this.post<Group>(`/tasky/create_group`, { title, join_policy });
  }

  public async updateGroup(groupId: number, title: string, join_policy: GroupJoinRequestPolicy): Promise<Group> {
    return await this.post<Group>(`/tasky/groups/${groupId}`, { title, join_policy });
  }

  public async getGroups(page?: number): Promise<GroupsResponse> {
    return await this.get<GroupsResponse>(`/tasky/groups?page=${page ?? 1}`);
  }

  public async getMyGroups(page?: number): Promise<GroupsResponse> {
    return await this.get<GroupsResponse>(`/tasky/my_groups?page=${page ?? 1}`);
  }

  public async getGroup(id: number): Promise<Group> {
    return await this.get<Group>("/tasky/groups/" + id);
  }

  public async getGroupJoinRequests(
    id: number,
    page?: number,
  ): Promise<GroupJoinRequestResponse> {
    return await this.get<GroupJoinRequestResponse>(
      `/tasky/groups/${id}/join_requests?page=${page ?? 1}`,
    );
  }

  public async createGroupJoinRequest(id: number): Promise<GroupJoinRequest> {
    return await this.post<GroupJoinRequest>(
      `/tasky/groups/${id}/create_join_request`,
      {},
    );
  }

  public async approveGroupJoinRequest(
    groupId: number,
    id: number,
  ): Promise<Group> {
    return await this.post<Group>(
      `/tasky/groups/${groupId}/join_requests/${id}/approve`,
      {},
    );
  }

  public async rejectGroupJoinRequest(
    groupId: number,
    id: number,
  ): Promise<Group> {
    return await this.post<Group>(
      `/tasky/groups/${groupId}/join_requests/${id}/reject`,
      {},
    );
  }

  public async removeUserFromGroup(groupId: number, memberId: number): Promise<void> {
    await this.delete<any>(`/tasky/groups/${groupId}/members/${memberId}`);
  }

  public async createAssignment(
    groupId: number,
    title: string,
    due_date: Date | null,
    description: string,
    language: AssignmentLanguage,
  ): Promise<Assignment> {
    return await this.post<Assignment>(`/tasky/groups/${groupId}/assignments`, {
      title,
      due_date,
      description,
      language,
    });
  }

  public async getAssignmentsForGroup(
    id: number,
    page?: number,
  ): Promise<AssignmentsResponse> {
    return await this.get<AssignmentsResponse>(
      `/tasky/groups/${id}/assignments?page=${page ?? 1}`,
    );
  }

  public async getAssignmentForGroup(
    groupId: number,
    assignmentId: number,
  ): Promise<Assignment> {
    return await this.get<Assignment>(
      `/tasky/groups/${groupId}/assignments/${assignmentId}`,
    );
  }

  public async updateAssignment(
    groupId: number,
    assignmentId: number,
    title: string,
    due_date: Date | null,
    description: string,
  ): Promise<Assignment> {
    return await this.post<Assignment>(
      `/tasky/groups/${groupId}/assignments/${assignmentId}/update`,
      { title, due_date, description },
    );
  }

  public async getCodeTestsFiles(
    groupId: number,
    assignmentId: number,
    fileIds: string[],
  ): Promise<MongoTestFile[]> {
    return await this.get<MongoTestFile[]>(
      `/tasky/groups/${groupId}/assignments/${assignmentId}/code_test_files?object_ids=${fileIds.join(",")}`,
    );
  }

  public async getPersonalSolutions(page?: number): Promise<SolutionsResponse> {
    return await this.get<SolutionsResponse>(
      `/tasky/personal_solutions?page=${page ?? 1}`,
    );
  }

  public async getSolution(id: number): Promise<Solution> {
    return await this.get<Solution>(`/tasky/solutions/${id}`);
  }

  public async getSolutionFiles(
    id: number,
    testFiles: string[],
    taskFiles: string[],
  ): Promise<SolutionFilesResponse> {
    return await this.get<SolutionFilesResponse>(
      `/tasky/solutions/${id}/files?task_files=${taskFiles.join(",")}&test_files=${testFiles.join(",")}`,
    );
  }

  public async getSolutionsForAssignment(
    id: number,
    page?: number,
  ): Promise<SolutionsResponse> {
    return await this.get<SolutionsResponse>(
      `/tasky/assignments/${id}/solutions?page=${page ?? 1}`,
    );
  }

  public async approveSolution(id: number): Promise<Solution> {
    return await this.post<Solution>(`/tasky/solutions/${id}/approve`, {});
  }

  public async rejectSolution(id: number): Promise<Solution> {
    return await this.post<Solution>(`/tasky/solutions/${id}/reject`, {});
  }

  public async createOrUpdateQuestionCatalogue(
    groupId: number,
    assignmentId: number,
    questions: QuestionCatalogueElement[],
  ): Promise<Assignment> {
    return await this.post<Assignment>(
      `/tasky/groups/${groupId}/assignments/${assignmentId}/question_catalogue`,
      { questions },
    );
  }

  public async reportBug(title: string, body: string): Promise<any> {
    return await this.post<any>("/report_issue", { title, body });
  }

  public async createAssignmentWish(
    groupId: number,
    title: string,
    description: string,
  ): Promise<AssignmentWish> {
    return await this.post<AssignmentWish>(
      `/tasky/groups/${groupId}/assignment_wishes`,
      { title, description },
    );
  }

  public async getAssignmentWishes(
    groupId: number,
    page?: number,
  ): Promise<AssignmentWishesResponse> {
    return await this.get<AssignmentWishesResponse>(
      `/tasky/groups/${groupId}/assignment_wishes?page=${page ?? 1}`,
    );
  }

  public async deleteAssignmentWish(
    groupId: number,
    wishId: number,
  ): Promise<void> {
    await this.delete<any>(
      `/tasky/groups/${groupId}/assignment_wishes/${wishId}`,
    );
  }

  public async getCodeComments(solutionId: number): Promise<CodeComment[]> {
    return await this.get<CodeComment[]>(
      `/tasky/solutions/${solutionId}/code_comments`,
    );
  }

  public async createCodeComment(
    solutionId: number,
    title: string,
    content: string,
  ): Promise<CodeComment> {
    return await this.post<CodeComment>(
      `/tasky/solutions/${solutionId}/code_comments`,
      { title, content },
    );
  }

  public async createTutor(username: string, password: string): Promise<User> {
    return await this.post<User>("/usernator/create_tutor", {
      username,
      password,
      email: "",
    });
  }

  public async getTutors(
    page?: number,
  ): Promise<{ tutors: User[]; total: number }> {
    return await this.get(`/usernator/all-tutors?page=${page ?? 1}`);
  }

  public async getSpotlightResult(search: string): Promise<Spotlight3Response> {
    return await this.get(`/tasky/spotlight?search=${search}`);
  }

  public async getNotifications(): Promise<Notification[]> {
    return await this.get<Notification[]>(`/tasky/notifications`);
  }

  public async removeNotificationForUser(id: number): Promise<void> {
    await this.delete<any>(`/tasky/notifications/${id}`);
  }

  public async removeAllNotificationsForUser(): Promise<void> {
    await this.delete<any>("/tasky/notifications");
  }

  public async searchUsersToEnlist(groupId: number, search: string): Promise<TaskyUser[]> {
    return await this.get<TaskyUser[]>(`/tasky/groups/${groupId}/enlistable?search=${search}`);
  }

  public async enlistUser(groupId: number, userId: number): Promise<void> {
    await this.post<any>(`/tasky/groups/${groupId}/enlist/${userId}`, {});
  }

  public async switchToTutorAccount(): Promise<void> {
    await this.post<any>("/usernator/switch_tutor", {});
  }

  public async leaveGroup(groupId: number): Promise<void> {
    await this.post<any>(`/tasky/groups/${groupId}/leave`, {});
  }

  public async deleteGroup(groupId: number): Promise<void> {
    await this.delete<any>(`/tasky/groups/${groupId}`);
  }

  public async createOrUpdateCodeTests(
    groupId: number,
    assignmentId: number,
    fileStructure: FileStructureTree,
    files: File[],
    runnerConfig: RunnerConfig,
    update: boolean = false,
  ): Promise<Assignment> {
    try {
      const formData = new FormData();
      formData.set(
        "file_structure",
        new Blob([JSON.stringify(fileStructure)], { type: "application/json" }),
      );
      for (const file of files) {
        formData.append("files", file, file.name);
      }
      formData.set(
        "runner_config",
        new Blob([JSON.stringify(runnerConfig)], { type: "application/json" }),
      );

      const resp = await fetch(
        `${this.apiUrl}/tasky/groups/${groupId}/assignments/${assignmentId}/code_test${update ? "/update" : ""}`,
        {
          method: "POST",
          mode: "cors",
          body: formData,
          credentials: "include",
          headers: {
            Accept: "application/json",
          },
        },
      );
      const txt = await resp.text();
      const obj = this.getObject(txt);
      if (resp.status !== 200) {
        throw new ApiError(resp.status, obj.message);
      }
      return obj;
    } catch (e) {
      if (e instanceof Error) {
        throw new ApiError(-1, e.message);
      }
      throw new ApiError(-1, `${e}`);
    }
  }

  public async createSolution(
    assignmentId: number,
    files: File[],
    answers: Map<string, QuestionSolution> | undefined = undefined,
  ): Promise<Solution> {
    try {
      const formData = new FormData();
      for (const file of files) {
        formData.append("files", file, file.name);
      }
      if (answers !== undefined) {
        formData.set(
          "answers",
          new Blob([JSON.stringify(Object.fromEntries(answers.entries()))], {
            type: "application/json",
          }),
        );
      } else {
        formData.set(
          "answers",
          new Blob(["{}"], {
            type: "application/json",
          }),
        );
      }
      const resp = await fetch(
        `${this.apiUrl}/tasky/assignments/${assignmentId}/solutions`,
        {
          method: "POST",
          mode: "cors",
          body: formData,
          credentials: "include",
          headers: {
            Accept: "application/json",
          },
        },
      );
      const txt = await resp.text();
      const obj = this.getObject(txt);
      if (resp.status !== 200) {
        throw new ApiError(resp.status, obj.message);
      }
      return obj;
    } catch (e) {
      if (e instanceof Error) {
        throw new ApiError(-1, e.message);
      }
      throw new ApiError(-1, `${e}`);
    }
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
   * Executes a general delete request
   *
   * @param path The path
   * @param body The json body
   * @throws ApiError The api error
   * @private
   */
  private async delete<T>(path: string, body?: object): Promise<T> {
    return await this.fetch<T>(path, "DELETE", body);
  }

  /**
   * Executes a generic HTTP fetch
   *
   * @param path The path
   * @param method The method
   * @param body The body if exists
   * @private
   */
  private async fetch<T>(
    path: string,
    method: string,
    body?: object,
  ): Promise<T> {
    try {
      const resp = await fetch(`${this.apiUrl}${path}`, {
        method,
        mode: "cors",
        body: body ? JSON.stringify(body) : undefined,
        credentials: "include",
        headers: {
          "Content-Type": "application/json",
          Accept: "*/*",
        },
      });
      const txt = await resp.text();
      const obj = this.getObject(txt);
      if (resp.status !== 200 && resp.status !== 201) {
        throw new ApiError(resp.status, obj.message);
      }
      return obj as T;
    } catch (e) {
      if (e instanceof Error) {
        throw new ApiError(-1, e.message);
      }
      throw new ApiError(-1, `${e}`);
    }
  }

  private getObject(text: string): any | GenericMessage {
    try {
      return JSON.parse(text);
    } catch (_) {
      return { message: text } as GenericMessage;
    }
  }
}

export default ApiService;
