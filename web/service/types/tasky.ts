import { FileStructureTree } from "@/components/FileStructure";

export interface GroupsResponse {
  groups: MinifiedGroup[];
  total: number;
  page: number;
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
  total: number;
}

export enum AssignmentLanguage {
  QuestionBased = "QuestionBased",
  Java = "Java",
  Golang = "Golang",
}

export interface Assignment {
  id: number;
  title: string;
  due_date: string|null;
  description: string;
  language: AssignmentLanguage;
  file_structure: FileStructureTree | null;
  question_catalogue: QuestionCatalogue | null;
  completed_by: TaskyUser[];
  runner_cpu: string|null;
  runner_memory: string|null;
  runner_timeout: string|null;
  runner_cmd: string|null;
  completed: boolean|null;
}

export interface AssignmentsResponse {
  assignments: Assignment[];
  total: number;
}

export interface RunnerConfig {
  runner_cpu: string;
  runner_memory: string;
  runner_timeout: string;
  runner_cmd: string;
}

export interface MongoTestFile {
  _id: { $oid: string };
  file_name: string;
  content_size: string;
  content: string;
  assignment_id: string;
}

export interface MongoTaskFile {
  _id: { $oid: string };
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
  result: string | null;
  error: string | null;
}

export interface Solution {
  id: number;
  submitter: TaskyUser;
  assignment: Assignment;
  approval_status: string | null;
  file_structure: FileStructureTree | null;
  question_results: ValidatedQuestionSolution[] | null;
  job: Job | null;
  group_id: number;
}

export interface SolutionsResponse {
  solutions: Omit<Solution, 'job'>[];
  total: number;
}

export interface SolutionFilesResponse {
  task_files: MongoTaskFile[];
  test_files: MongoTestFile[];
}

export enum AnswerType {
  String = "String",
  Number = "Number",
  Boolean = "Boolean",
  StrContains = "StrContains",
}

export interface QuestionCatalogueElement {
  question: string;
  answer: any;
  answer_type: AnswerType | null;
}

export interface QuestionCatalogue {
  catalogue: Record<string, QuestionCatalogueElement>;
}

export interface QuestionSolution {
  answer: any;
}

export interface ValidatedQuestionSolution {
  question: string;
  answer: any;
  correct: boolean;
}

export interface AssignmentWish {
  id: number;
  title: string;
  description: string;
}

export interface AssignmentWishesResponse {
  results: AssignmentWish[];
  total: number
}

export interface CodeComment {
  id: number;
  title: string;
  content: string;
  commentor: number;
}

export interface Notification {
  id: number;
  title: string;
  content: string;
}
