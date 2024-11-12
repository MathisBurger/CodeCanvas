import { TreeNodeData } from "@mantine/core";
import {
  DisplayMode,
  FileStructureFile,
  FileStructureTree,
} from "@/components/FileStructure";
import { FileWithPath } from "@mantine/dropzone";

/**
 * Builds the mantine tree data from structure
 *
 * @param structure The base structure
 * @param fileRoot The fileRoot
 * @param editable If the data should be editable in tree
 */
export const buildDataFromStructure = (
  structure: FileStructureTree,
  fileRoot: string,
  editable: boolean,
): [string[], TreeNodeData[]] => {
  const data = [];
  const fileNames = [];
  for (const folder of structure.folders ?? []) {
    const folderRoot = fileRoot + "/" + (folder.current_folder_name ?? "");
    const [existingNames, children] = buildDataFromStructure(
      folder,
      folderRoot,
      editable,
    );
    data.push({
      value: folderRoot,
      label: folder.current_folder_name,
      children,
    });
    fileNames.push(...existingNames);
  }
  for (const file of structure.files) {
    data.push({
      value: fileRoot + "/" + file.filename,
      label: file.filename,
      nodeProps: {
        object_id: file.object_id,
        is_test_file: file.is_test_file,
      },
    });
    fileNames.push(file.filename);
  }
  if (editable) {
    data.push({
      value: fileRoot + "/createNewFile",
      label: "Create new file",
      nodeProps: {
        file_root: fileRoot,
      },
    });
    data.push({
      value: fileRoot + "/createNewFolder",
      label: "Create new folder",
      nodeProps: {
        file_root: fileRoot,
        folders: (structure.folders ?? []).map((f) => f.current_folder_name),
      },
    });
  }
  return [fileNames, data];
};

/**
 * Creates a new folder in file structure
 *
 * @param structure The initial file structure
 * @param path The path of the folder the new folder is located in
 * @param folderName The name of the new folder
 */
export const createFolder = (
  structure: FileStructureTree,
  path: string,
  folderName: string,
): FileStructureTree => {
  const spl = path.split("/");
  if (spl.length === 1) {
    if (structure.folders === null) structure.folders = [];
    structure.folders.push({
      files: [],
      folders: [],
      current_folder_name: folderName,
    });
    return structure;
  }
  if (spl.length > 1) {
    // @ts-ignore
    for (const [index, folder] of (structure.folders ?? []).entries()) {
      if (folder.current_folder_name === spl[1]) {
        // @ts-ignore
        structure.folders[index] = createFolder(
          folder,
          spl.splice(1).join("/"),
          folderName,
        );
        break;
      }
    }
  }
  return structure;
};

/**
 * Creates a new file in file structure
 *
 * @param structure The initial file structure
 * @param path The path where the file will be located
 * @param fileName The name of the file
 */
export const createFile = (
  structure: FileStructureTree,
  path: string,
  fileName: string,
): FileStructureTree => {
  const spl = path.split("/");
  if (spl.length === 1) {
    if (structure.files === null) structure.files = [];
    structure.files.push({
      filename: fileName,
      is_test_file: false,
      object_id: null,
    });
    return structure;
  }
  if (spl.length > 1) {
    // @ts-ignore
    for (const [index, folder] of (structure.folders ?? []).entries()) {
      if (folder.current_folder_name === spl[1]) {
        // @ts-ignore
        structure.folders[index] = createFile(
          folder,
          spl.splice(1).join("/"),
          fileName,
        );
        break;
      }
    }
  }
  return structure;
};

/**
 * Updates the test file state of a test file
 *
 * @param structure The file structure to perform the update on
 * @param path The path to folder of file
 * @param fileName The file name
 * @param state The test file state
 */
export const updateTestFileState = (
  structure: FileStructureTree,
  path: string,
  fileName: string,
  state: boolean,
): FileStructureTree => {
  const spl = path.split("/");
  if (spl.length === 2) {
    // @ts-ignore
    for (const [index, file] of structure.files.entries()) {
      if (file.filename === fileName) {
        file.is_test_file = state;
        structure.files[index] = file;
        return structure;
      }
    }
  }
  if (spl.length > 2) {
    // @ts-ignore
    for (const [index, folder] of (structure.folders ?? []).entries()) {
      if (folder.current_folder_name === spl[1]) {
        // @ts-ignore
        structure.folders[index] = updateTestFileState(
          folder,
          spl.splice(1).join("/"),
          fileName,
          state,
        );
        return structure;
      }
    }
  }
  return structure;
};

/**
 * Finds a file in tree
 *
 * @param structure The file structure that should be searched
 * @param path The path to the file
 */
export const findObjectIdInStructure = (
  structure: FileStructureTree,
  path: string,
): FileStructureFile | null => {
  const spl = path.split("/");
  if (spl.length === 2) {
    for (const file of structure.files) {
      if (file.filename === spl[1]) {
        return file;
      }
    }
  }
  for (const folder of structure.folders ?? []) {
    if (folder.current_folder_name === spl[1]) {
      return findObjectIdInStructure(folder, spl.splice(1).join("/"));
    }
  }
  return null;
};

/**
 * Filters file structure to only display test files or task files or all
 *
 * @param structure The structure that should be searched
 * @param displayMode The display mode
 */
export const filterFileStructureForDisplayMode = (
  structure: FileStructureTree,
  displayMode: DisplayMode,
): FileStructureTree => {
  if (displayMode === "all") {
    return structure;
  }
  structure.files = structure.files.filter((f) =>
    displayMode === "test" ? f.is_test_file : !f.is_test_file,
  );
  const newFolders = [];
  for (const folder of structure.folders ?? []) {
    if (folder.files.length !== 0 || (folder.folders ?? []).length !== 0) {
      newFolders.push(filterFileStructureForDisplayMode(folder, displayMode));
    }
  }
  return structure;
};

/**
 * Extracts all files from the file structure
 *
 * @param structure The file structure
 */
export const extractFilesFromFileStructure = (
  structure: FileStructureTree,
): string[] => {
  const files = [];
  for (const file of structure.files) {
    files.push(file.filename);
  }
  for (const folder of structure.folders ?? []) {
    files.push(...extractFilesFromFileStructure(folder));
  }
  return files;
};

/**
 * Removes all object IDs from the files listed in the second parameter.
 * This is required for the backend to notice new files.
 *
 * @param structure The file structure
 * @param fileNames All new uploaded files
 */
export const removeObjectIds = (
  structure: FileStructureTree,
  fileNames: string[],
): FileStructureTree => {
  structure.folders = (structure.folders ?? []).map((folder) =>
    removeObjectIds(folder, fileNames),
  );
  structure.files = structure.files.map((file) => {
    if (fileNames.indexOf(file.filename) > -1) {
      return { ...file, object_id: null };
    }
    return file;
  });
  return structure;
};

/**
 * Removes a file from the file structure
 *
 * @param structure The file structure
 * @param fileName The name of the file
 * @param isFolder If the file to delete is a folder
 */
export const removeFile = (
  structure: FileStructureTree,
  fileName: string,
  isFolder: boolean,
): FileStructureTree => {
  if (isFolder) {
    const beforeSize = (structure.folders ?? []).length;
    structure.folders = (structure.folders ?? []).filter(
      (f) => f.current_folder_name !== fileName,
    );

    // Early return to prevent tree from being further searched
    if (structure.folders.length != beforeSize) return structure;
  } else {
    const beforeSize = structure.files.length;
    structure.files = structure.files.filter((f) => f.filename !== fileName);

    // Early return to prevent tree from being further searched
    if (structure.files.length != beforeSize) return structure;
  }

  structure.folders = (structure.folders ?? []).map((folder) =>
    removeFile(folder, fileName, isFolder),
  );
  return structure;
};
