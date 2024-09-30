import { useState } from "react";
import FileStructure, {FileStructureTree} from "@/components/FileStructure";
import CentralLoading from "@/components/CentralLoading";
import {Modal} from "@mantine/core";

interface AssignmentCreateOrUpdateCodeTestModalProps {
    onClose: () => void;
}

const AssignmentCreateOrUpdateCodeTestModal = ({onClose}: AssignmentCreateOrUpdateCodeTestModalProps) => {

    const [fileStructure, setFileStructure] = useState<FileStructureTree|null>({
        current_folder_name: null,
        folders: [
            {
                current_folder_name: 'Lol',
                folders: [],
                files: [
                    {
                        filename: 'Lol2',
                        object_id: null,
                        is_test_file: true
                    }
                ]
            }
        ],
        files: [
            {
                filename: 'Lol1',
                object_id: null,
                is_test_file: true
            }
        ]
    });

    if (fileStructure === null) {
        return <CentralLoading />;
    }

    return (
        <Modal opened onClose={onClose}>
            <FileStructure structure={fileStructure} setStructure={setFileStructure} editable={true} />
        </Modal>
    )
}

export default AssignmentCreateOrUpdateCodeTestModal;
