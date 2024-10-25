import FileStructure, {
  FileStructureFile,
  FileStructureTree,
} from "@/components/FileStructure";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {
  MongoTaskFile,
  MongoTestFile,
  SolutionFilesResponse,
} from "@/service/types/tasky";
import { Grid } from "@mantine/core";
import { useCallback, useEffect, useMemo, useState } from "react";
import CentralLoading from "@/components/CentralLoading";
import CodeDisplay from "@/components/CodeDisplay";
import useCurrentUser from "@/hooks/useCurrentUser";
import { isGranted } from "@/service/auth";
import { UserRoles } from "@/service/types/usernator";

interface FileStructureDisplayProps {
  structure: FileStructureTree;
  groupId?: number;
  assignmentId: number;
  solutionId?: number;
}

const flattenStructureToFiles = (
  structure: FileStructureTree,
): FileStructureFile[] => {
  const files = [...structure.files];
  for (const folder of structure.folders ?? []) {
    files.push(...flattenStructureToFiles(folder));
  }
  return files;
};

const FileStructureDisplay = ({
  structure,
  groupId,
  assignmentId,
  solutionId,
}: FileStructureDisplayProps) => {
  if (solutionId === undefined && groupId === undefined) {
    throw new Error("Invalid combination of props");
  }
  const { user } = useCurrentUser();
  const api = useApiServiceClient();

  const filesFlattened = useMemo<FileStructureFile[]>(
    () => flattenStructureToFiles(structure),
    [structure],
  );
  const testObjectIds = useMemo<string[]>(
    () =>
      filesFlattened
        .filter((f) => f.object_id !== null && f.is_test_file)
        .map((f) => f.object_id) as string[],
    [filesFlattened],
  );
  const taskObjectIds = useMemo<string[]>(
    () =>
      filesFlattened
        .filter((f) => f.object_id !== null && !f.is_test_file)
        .map((f) => f.object_id) as string[],
    [filesFlattened],
  );
  const objectIds = useMemo<string[]>(
    () => [...testObjectIds, ...taskObjectIds],
    [testObjectIds, taskObjectIds],
  );
  const cumulatedSize = useMemo<number>(
    () => {
      if (filesFlattened.length === 0) {
        return 0;
      }
      return filesFlattened.reduce((a, b) => ({
        ...a,
        file_size: (a.file_size ?? 0) + (b.file_size ?? 0),
      })).file_size!
    },
    [filesFlattened],
  );
  const loadAll = useMemo<boolean>(
    () => cumulatedSize <= 5 * 1014 ** 2,
    [cumulatedSize],
  );

  const [contents, setContents] = useState<
    Map<string, MongoTestFile | MongoTaskFile>
  >(new Map());
  const [selected, setSelected] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const getApiCall = useCallback(
    async (
      ids: string[],
      furtherIds?: string[],
    ): Promise<MongoTestFile[] | SolutionFilesResponse> => {
      if (ids.length === 0) {
        return (async () => [])();
      }
      if (groupId) {
        return api.getCodeTestsFiles(groupId, assignmentId, ids);
      }
      return api.getSolutionFiles(solutionId ?? -1, furtherIds ?? [], ids);
    },
    [groupId, assignmentId, solutionId],
  );

  const getSelectedValue = useCallback(():
    | MongoTestFile
    | MongoTaskFile
    | null => {
    if (selected === null) return null;

    if (contents.has(selected)) {
      return contents.get(selected)!;
    }

    let selectedCopy = selected;
    let isTestFile = false;
    if (selected.startsWith("task-")) {
      selectedCopy = selected.replaceAll("task-", "");
    }
    if (selected.startsWith("test-")) {
      selectedCopy = selected.replaceAll("test-", "");
      isTestFile = true;
    }

    getApiCall(
      solutionId === undefined ? [selected] : isTestFile ? [selectedCopy] : [],
      solutionId === undefined
        ? undefined
        : !isTestFile
          ? [selectedCopy]
          : undefined,
    ).then((result) => {
      if (Array.isArray(result)) {
        if (result.length > 0) {
          setContents(contents.set(result[0]._id.$oid, result[0]));
          setLoading(false);
        }
      } else {
        if (result.task_files.length > 0) {
          setContents(
            contents.set(
              "task-" + result.task_files[0]._id.$oid,
              result.task_files[0],
            ),
          );
        }
        if (result.test_files.length > 0) {
          setContents(
            contents.set(
              "test-" + result.test_files[0]._id.$oid,
              result.test_files[0],
            ),
          );
        }
      }
    });
    setLoading(true);
    return null;
  }, [selected, contents, getApiCall, solutionId]);

  useEffect(() => {
    if (loadAll) {
      getApiCall(
        solutionId !== undefined ? taskObjectIds : objectIds,
        solutionId !== undefined ? testObjectIds : undefined,
      ).then((res) => {
        const map = new Map<string, MongoTestFile | MongoTaskFile>();
        if (Array.isArray(res)) {
          for (const file of res) {
            map.set(file._id.$oid, file);
          }
        } else {
          for (const testFile of res.test_files) {
            map.set("test-" + testFile._id.$oid, testFile);
          }
          for (const taskFile of res.task_files) {
            map.set("task-" + taskFile._id.$oid, taskFile);
          }
        }
        setContents(map);
      });
    }
  }, [getApiCall, loadAll, objectIds, solutionId, taskObjectIds, testObjectIds]);


  return (
    <Grid>
      <Grid.Col span={3}>
        <FileStructure
          structure={structure}
          editable={false}
          setSelected={setSelected}
          displayMode={
            solutionId !== undefined
              ? isGranted(user, [UserRoles.Admin, UserRoles.Tutor])
                ? "all"
                : "task"
              : "test"
          }
          solutionMode={solutionId !== undefined}
        />
      </Grid.Col>
      <Grid.Col span={9}>
        {loading ? (
          <CentralLoading />
        ) : (
          <CodeDisplay file={getSelectedValue()} />
        )}
      </Grid.Col>
    </Grid>
  );
};

export default FileStructureDisplay;
