package services

const (
	LanguageGo   = "Golang"
	LanguageJava = "Java"
)

type ExecutionFile struct {
	Filename   string `json:"filename"`
	ObjectId   string `json:"object_id"`
	FileSize   int64  `json:"file_size"`
	IsTestFile bool   `json:"is_test_file"`
}

type ExecutionFileStructure struct {
	Files             *[]ExecutionFile          `json:"files"`
	Folders           *[]ExecutionFileStructure `json:"folders"`
	CurrentFolderName *string                   `json:"current_folder_name"`
}

type Solution struct {
	Id            int                    `json:"id"`
	SubmitterId   int                    `json:"submitter_id"`
	FileStructure ExecutionFileStructure `json:"file_structure"`
}

type Assignment struct {
	Id            int    `json:"id"`
	Title         string `json:"title"`
	Language      string `json:"language"`
	RunnerCpu     string `json:"runner_cpu"`
	RunnerMemory  string `json:"runner_memory"`
	RunnerTimeout string `json:"runner_timeout"`
}

type ExecRequest struct {
	Solution   Solution   `json:"solution"`
	Assignment Assignment `json:"assignment"`
}
