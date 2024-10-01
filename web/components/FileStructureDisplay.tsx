import FileStructure, {FileStructureFile, FileStructureTree} from "@/components/FileStructure";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { MongoTestFile } from "@/service/types/tasky";
import {Grid} from "@mantine/core";
import {useCallback, useEffect, useMemo, useState} from "react";
import CentralLoading from "@/components/CentralLoading";
import CodeDisplay from "@/components/CodeDisplay";

// TODO: For later use with task files just set groupId to optional and add another optional solutionId
// Then one of the values needs to be set in order to fetch objects.
interface FileStructureDisplayProps {
    structure: FileStructureTree;
    groupId: number;
    assignmentId: number;
}

const flattenStructureToFiles = (structure: FileStructureTree): FileStructureFile[] => {
    const files = structure.files;
    for (const folder of structure.folders ?? []) {
        files.push(...flattenStructureToFiles(folder))
    }
    return files;
}

const FileStructureDisplay = ({structure, groupId, assignmentId}: FileStructureDisplayProps) => {

    const api = useApiServiceClient();

    const filesFlattened = useMemo<FileStructureFile[]>(() => flattenStructureToFiles(structure), [structure]);
    const objectIds = useMemo<string[]>(() => filesFlattened.map((f) => f.object_id).filter((f) => f !== null), [filesFlattened]);
    const cumulatedSize = useMemo<number>(() => filesFlattened.reduce((a, b) => ({...a, file_size: (a.file_size ?? 0) + (b.file_size ?? 0)})).file_size!, [filesFlattened]);
    const loadAll = useMemo<boolean>(() => cumulatedSize <= 5 * 1014 **2, [cumulatedSize]);

    const [contents, setContents] = useState<Map<string, MongoTestFile>>(new Map());
    const [selected, setSelected] = useState<string|null>(null);
    const [loading, setLoading] = useState(false);

    const getApiCall = useCallback(async (ids: string[]): Promise<MongoTestFile[]> => {
        return api.getCodeTestsFiles(groupId, assignmentId, ids);
    }, [groupId, assignmentId]);

    const getSelectedValue = useCallback((): MongoTestFile|null => {

        if (selected === null) return null;

        if (contents.has(selected)) {
            return contents.get(selected)!;
        }

        getApiCall([selected]).then(result => {
            if (result.length > 0) {
                setContents(contents.set(result[0]._id.$oid, result[0]));
                setLoading(false);
            }
        });
        setLoading(true);
        return null;
    }, [selected, contents, getApiCall]);

    useEffect(() => {
        if (loadAll) {
            getApiCall(objectIds)
                .then((res) => {
                   const map = new Map<string, MongoTestFile>();
                   for (const file of res) {
                       map.set(file._id.$oid, file);
                   }
                   setContents(map);
                });
        }
    }, [loadAll, objectIds]);

    return (
        <Grid>
            <Grid.Col span={3}>
                <FileStructure
                    structure={structure}
                    editable={false}
                    setSelected={setSelected}
                    displayMode="test"
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
}

export default FileStructureDisplay;
