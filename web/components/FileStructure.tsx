import { useMemo } from "react";
import {Group, Paper, Tree, TreeNodeData} from "@mantine/core";
import {IconPlus} from "@tabler/icons-react";
import FileIcon from "@/components/FileIcon";
import classes from "./FileStructure.module.scss";

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
const buildDataFromStructure = (structure: FileStructureTree, fileRoot: string = '', editable: boolean = false): TreeNodeData[] => {
    const data = [];
    for (const folder of (structure.folders ?? [])) {
        const folderRoot = fileRoot + '/' + (folder.current_folder_name ?? '');
        data.push({
            value: folderRoot,
            label: folder.current_folder_name,
            children: buildDataFromStructure(folder, folderRoot)
        });
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
    }
    if (editable) {
        data.push({
            value: fileRoot + '/createNewFile',
            label: 'Create new file',
        })
    }
    return data;
}

const FileStructure = ({structure, setStructure, editable}: FileStructureProps) => {

    const treeData = useMemo(() => buildDataFromStructure(structure, '', editable), [structure, editable]);



    return (
        <Tree data={treeData} renderNode={({node, expanded, hasChildren, elementProps}) => (
            <Group {...elementProps}>
                <Paper radius="sm" p="sm" {...elementProps}>
                    <Group gap={5} {...elementProps}>
                        {node.value.indexOf('createNewFile') > -1 ? (
                            <IconPlus size={18} />
                        ) : (
                            <FileIcon name={(node.label ?? '') as string} isFolder={hasChildren} expanded={expanded} />
                        )}

                        <span>{node.label}</span>
                    </Group>
                </Paper>
            </Group>
        )} selectOnClick clearSelectionOnOutsideClick classNames={classes} />
    );
}

export default FileStructure;
