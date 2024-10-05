package services

import (
	"errors"
	"github.com/knadh/koanf/maps"
	"github.com/runabol/tork/input"
	"strconv"
)

func BuildTask(er ExecRequest) (input.Task, error) {
	var image string
	var run string

	switch er.Assignment.Language {
	case LanguageGo:
		image = "golang:1.19"
		run = "go run main.go > $TORK_OUTPUT"
	default:
		return input.Task{}, errors.New("invalid language")
	}

	return input.Task{
		Name:  er.Assignment.Title + " - " + strconv.Itoa(er.Solution.Id),
		Image: image,
		Run:   run,
		Limits: &input.Limits{
			CPUs:   er.Assignment.RunnerCpu,
			Memory: er.Assignment.RunnerMemory,
		},
		Timeout:  er.Assignment.RunnerTimeout, // terminate container after 5 seconds
		Networks: []string{"none"},            // disable networking
		Files: map[string]string{
			filename: er.Code,
		}}, nil
}

func buildFs(solution Solution) map[string]string {
	flattenedFs := flattenFs(solution.FileStructure, "")

}

func flattenFs(structure ExecutionFileStructure, basePath string) map[string]interface{} {
	var mapping map[string]interface{}
	folderName := ""
	if structure.CurrentFolderName != nil {
		folderName = *structure.CurrentFolderName
	}
	if structure.Files != nil {
		for _, file := range *structure.Files {
			path := basePath + folderName + "/" + file.Filename
			mapping[path] = file.ObjectId
		}
	}
	if structure.Folders != nil {
		for _, folder := range *structure.Folders {
			res := flattenFs(folder, basePath+folderName)
			maps.Merge(res, mapping)
		}
	}
	return mapping
}
