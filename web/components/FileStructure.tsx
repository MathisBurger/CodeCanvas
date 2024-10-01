'use client';
import { useMemo } from "react";
import {Group, Paper, Tree, TreeNodeData} from "@mantine/core";
import {IconPlus} from "@tabler/icons-react";
import FileIcon from "@/components/FileIcon";
import classes from "./FileStructure.module.scss";
import {FileStructureElement, FileStructureNewInput} from "@/components/FileStructureComponents";

export interface FileStructureFile {
    filename: string;
    object_id: string|null;
    is_test_file: boolean;
}

export interface FileStructureTree {
    files: FileStructureFile[];
    folders: FileStructureTree[]|null;
    current_folder_name: string|null;
}


interface FileStructureProps {
    structure: FileStructureTree;
    setStructure: (structure: FileStructureTree) => void;
    editable: boolean;
}

/**
 * Builds tree data from file structure interface.
 *
 * @param structure The file structure
 * @param fileRoot The current file root
 */
const buildDataFromStructure = (structure: FileStructureTree, fileRoot: string, editable: boolean): [string[], TreeNodeData[]] => {
    const data = [];
    const fileNames = [];
    for (const folder of (structure.folders ?? [])) {
        const folderRoot = fileRoot + '/' + (folder.current_folder_name ?? '');
        const [existingNames, children] = buildDataFromStructure(folder, folderRoot, editable)
        data.push({
            value: folderRoot,
            label: folder.current_folder_name,
            children
        });
        fileNames.push(...existingNames);
    }
    for (const file of structure.files) {
        data.push({
            value: fileRoot + '/' + file.filename,
            label: file.filename,
            nodeProps: {
                object_id: file.object_id,
                is_test_file: file.is_test_file,

            }
        });
        fileNames.push(file.filename);
    }
    if (editable) {
        data.push({
            value: fileRoot + '/createNewFile',
            label: 'Create new file',
            nodeProps: {
                file_root: fileRoot
            }
        });
        data.push({
            value: fileRoot + '/createNewFolder',
            label: 'Create new folder',
            nodeProps: {
                file_root: fileRoot,
                folders: (structure.folders ?? []).map((f) => f.current_folder_name)
            }
        })
    }
    return [fileNames, data];
}

const createFolder = (structure: FileStructureTree, path: string, folderName: string): FileStructureTree => {
    const spl = path.split('/');
    if (spl.length === 1) {
        if (structure.folders === null) structure.folders = [];
        structure.folders.push({files: [], folders: [], current_folder_name: folderName});
        return structure;
    }
    if (spl.length > 1) {
        // @ts-ignore
        for (const [index, folder] of (structure.folders ?? []).entries()) {
            if (folder.current_folder_name === spl[1]) {
                // @ts-ignore
                structure.folders[index] = createFolder(folder, spl.splice(1).join('/'), folderName);
                break;
            }
        }

    }
    return structure;
}

const createFile = (structure: FileStructureTree, path: string, fileName: string): FileStructureTree => {
    const spl = path.split('/');
    if (spl.length === 1) {
        if (structure.files === null) structure.files = [];
        structure.files.push({filename: fileName, is_test_file: false, object_id: null});
        return structure;
    }
    if (spl.length > 1) {
        // @ts-ignore
        for (const [index, folder] of (structure.folders ?? []).entries()) {
            if (folder.current_folder_name === spl[1]) {
                // @ts-ignore
                structure.folders[index] = createFile(folder, spl.splice(1).join('/'), fileName);
                break;
            }
        }

    }
    return structure;
}

const FileStructure = ({structure, setStructure, editable}: FileStructureProps) => {

    const [fileNames, treeData] = useMemo(() => buildDataFromStructure(structure, '', editable), [structure, editable]);



    return (
        <Tree data={treeData} renderNode={({node, expanded, hasChildren, elementProps}) => (
            <Group {...elementProps}>
                <Paper radius="sm" p="sm"  {...elementProps} style={{width: '100%'}}>
                    {node.value.indexOf('createNewFile') > -1 || node.value.indexOf('createNewFolder') > -1 ? (
                        <FileStructureNewInput
                            label={(node.label ?? '') as string}
                            fileNames={node.value.indexOf('createNewFolder') > -1 ? node?.nodeProps?.folders ?? [] : fileNames}
                            addFunc={(name) => setStructure((node.value.indexOf('createNewFolder') > -1 ? createFolder : createFile)(structure, node?.nodeProps?.file_root ?? '', name))}
                            {...elementProps}
                        />
                    ) : (
                        <FileStructureElement
                            label={(node.label ?? '') as string}
                            isFolder={hasChildren}
                            expanded={expanded}
                            {...elementProps}
                        />
                    )}
                </Paper>
            </Group>
        )} selectOnClick clearSelectionOnOutsideClick classNames={classes} />
    );
}

export default FileStructure;
