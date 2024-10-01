'use client';
import { useState } from "react";
import FileStructure, {FileStructureTree} from "@/components/FileStructure";
import {Alert, Modal} from "@mantine/core";
import {useSetState} from "@mantine/hooks";

interface AssignmentCreateOrUpdateCodeTestModalProps {
    onClose: () => void;
}

const AssignmentCreateOrUpdateCodeTestModal = ({onClose}: AssignmentCreateOrUpdateCodeTestModalProps) => {

    const [fileStructure, setFileStructure] = useSetState<FileStructureTree>({folders: [], files: [], current_folder_name: null});

    return (
        <Modal opened={true} onClose={onClose} size="xl">
            <Alert color="red" variant="light" title="Performance issues" mb={30}>Please note that large structures can lead to performance issues. This will be optimized soon.</Alert>
            <FileStructure structure={fileStructure} setStructure={setFileStructure} editable={true} />
        </Modal>
    )
}

export default AssignmentCreateOrUpdateCodeTestModal;
