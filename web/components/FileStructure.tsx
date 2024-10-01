'use client';
import {useEffect, useMemo} from "react";
import {Button, Group, Paper, Tree, TreeNodeData, useTree} from "@mantine/core";
import classes from "./FileStructure.module.scss";
import {FileStructureElement, FileStructureNewInput} from "@/components/FileStructureComponents";
import path from "node:path";
import {
    buildDataFromStructure,
    createFile,
    createFolder, filterFileStructureForDisplayMode,
    findObjectIdInStructure,
    updateTestFileState
} from "@/utils/FileStructure";

export interface FileStructureFile {
    filename: string;
    object_id: string|null;
    is_test_file: boolean;
    file_size?: number;
}

export interface FileStructureTree {
    files: FileStructureFile[];
    folders: FileStructureTree[]|null;
    current_folder_name: string|null;
}

export type DisplayMode = 'all' | 'test' | 'task';

interface FileStructureProps {
    structure: FileStructureTree;
    setStructure?: (structure: FileStructureTree) => void;
    editable: boolean;
    setSelected?: (objectId: string) => void;
    displayMode?: DisplayMode;
}



const FileStructure = ({structure, setStructure, editable, setSelected, displayMode = 'all'}: FileStructureProps) => {

    const [fileNames, treeData] = useMemo(() => buildDataFromStructure(filterFileStructureForDisplayMode(structure, displayMode), '', editable), [structure, editable, displayMode]);
    const tree = useTree();

    useEffect(() => {
        if (setSelected && tree.selectedState.length > 0) {
            const selected = tree.selectedState[0];
            const object = findObjectIdInStructure(structure, selected);
            if (null !== object && object.object_id !== null) {
                setSelected(object.object_id);
            }
        }
    }, [tree.selectedState])

    return (
       <Paper withBorder>
           <Button.Group>
               <Button variant="default" onClick={() => tree.expandAllNodes()}>Expand All</Button>
               <Button variant="default" onClick={() => tree.collapseAllNodes()}>Collapse All</Button>
           </Button.Group>
           <Tree data={treeData} tree={tree} renderNode={({node, expanded, hasChildren, elementProps}) => (
               <Group {...elementProps}>
                   <Paper radius="sm" p="sm"  {...elementProps} style={{width: '100%'}}>
                       {node.value.indexOf('createNewFile') > -1 || node.value.indexOf('createNewFolder') > -1 ? (
                           <FileStructureNewInput
                               label={(node.label ?? '') as string}
                               fileNames={node.value.indexOf('createNewFolder') > -1 ? node?.nodeProps?.folders ?? [] : fileNames}
                               addFunc={(name) => setStructure ? setStructure((node.value.indexOf('createNewFolder') > -1 ? createFolder : createFile)(structure, node?.nodeProps?.file_root ?? '', name)) : null}
                               {...elementProps}
                           />
                       ) : (
                           <FileStructureElement
                               label={(node.label ?? '') as string}
                               isFolder={hasChildren}
                               expanded={expanded}
                               isTestFile={node?.nodeProps?.is_test_file ?? false}
                               setIsTestFile={(s) => setStructure ? setStructure(updateTestFileState(structure, node.value, node.label as string, s)) : null}
                               editable={editable}
                               {...elementProps}
                           />
                       )}
                   </Paper>
               </Group>
           )}
                 selectOnClick
                 clearSelectionOnOutsideClick
                 classNames={classes}
           />
       </Paper>
    );
}

export default FileStructure;
