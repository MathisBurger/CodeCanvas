package services

import (
	"context"
	"errors"
	"executor/internal/global"
	"fmt"
	"github.com/knadh/koanf/maps"
	"github.com/runabol/tork/input"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"strconv"
)

func BuildTask(er ExecRequest) (input.Task, error) {
	var image string

	switch er.Assignment.Language {
	case LanguageGo:
		image = "golang:1.19"
	default:
		return input.Task{}, errors.New("invalid language")
	}

	fs := buildFs(er.Solution)
	fmt.Println(fs)

	return input.Task{
		Name:  er.Assignment.Title + " - " + strconv.Itoa(er.Solution.Id),
		Image: image,
		Run:   er.Assignment.RunnerCmd + " > $TORK_OUTPUT",
		Limits: &input.Limits{
			CPUs:   er.Assignment.RunnerCpu,
			Memory: er.Assignment.RunnerMemory,
		},
		Timeout:  er.Assignment.RunnerTimeout, // terminate container after 5 seconds
		Networks: []string{"none"},            // disable networking
		Files:    fs,
	}, nil
}

func buildFs(solution Solution) map[string]string {
	fsMapping := make(map[string]string)
	flattenedFs := flattenFs(solution.FileStructure, "")
	files := getFiles(flattenedFs)
	contents := getFileContents(files)
	for k, v := range flattenedFs {
		value, _ := v.(ExecutionFile)
		fsMapping[k] = contents[value.Filename]
	}
	return fsMapping
}

func flattenFs(structure ExecutionFileStructure, basePath string) map[string]interface{} {
	mapping := make(map[string]interface{})
	folderName := ""
	if structure.CurrentFolderName != nil {
		folderName = *structure.CurrentFolderName + "/"
	}
	if structure.Files != nil {
		for _, file := range *structure.Files {
			path := basePath + folderName + file.Filename
			mapping[path] = file
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

func getFiles(mapping map[string]interface{}) []ExecutionFile {
	var files []ExecutionFile
	for _, file := range mapping {
		parsedFile, _ := file.(ExecutionFile)
		files = append(files, parsedFile)
	}
	return files
}

type testFile struct {
	Id          primitive.ObjectID `bson:"_id"`
	FileName    string             `bson:"file_name"`
	ContentSize int64              `bson:"content_size"`
	Content     string             `bson:"content"`
	SolutionId  int64              `bson:"solution_id"`
}

type taskFile struct {
	Id           primitive.ObjectID `bson:"_id"`
	FileName     string             `bson:"file_name"`
	ContentSize  int64              `bson:"content_size"`
	Content      string             `bson:"content"`
	AssignmentId int64              `bson:"assignment_id"`
}

func getFileContents(files []ExecutionFile) map[string]string {
	testFileCollection := global.MongoDB.Collection("test_files")
	taskFileCollection := global.MongoDB.Collection("task_files")

	testFiles := filterFiles(files, true)
	taskFiles := filterFiles(files, false)

	testFilesCursor, _ := testFileCollection.Find(context.Background(), bson.M{"_id": bson.M{"$in": testFiles}})
	taskFilesCursor, _ := taskFileCollection.Find(context.Background(), bson.M{"_id": bson.M{"$in": taskFiles}})

	testFilesResult := readCursorTestFile[testFile](testFilesCursor)
	taskFilesResult := readCursorTestFile[taskFile](taskFilesCursor)

	mapping := make(map[string]string)
	for _, file := range testFilesResult {
		mapping[file.FileName] = file.Content
	}
	for _, file := range taskFilesResult {
		mapping[file.FileName] = file.Content
	}
	return mapping
}

func filterFiles(files []ExecutionFile, testFiles bool) []primitive.ObjectID {
	var result []primitive.ObjectID
	for _, file := range files {
		if (testFiles && file.IsTestFile) || (!testFiles && !file.IsTestFile) {
			objId, _ := primitive.ObjectIDFromHex(file.ObjectId)
			result = append(result, objId)
		}
	}
	return result
}

func readCursorTestFile[T testFile | taskFile](cursor *mongo.Cursor) []T {
	var result []T
	for cursor.Next(context.Background()) {
		var file T
		_ = cursor.Decode(&file)
		result = append(result, file)
	}
	return result
}
