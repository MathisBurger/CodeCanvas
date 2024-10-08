import {Button, Group, List, Modal, Paper} from "@mantine/core";
import {Assignment} from "@/service/types/tasky";
import {FormEvent, useMemo, useState} from "react";
import {FileWithPath} from "@mantine/dropzone";
import {extractFilesFromFileStructure} from "@/utils/FileStructure";
import InternalDropzone from "@/components/InternalDropzone";
import {notifications} from "@mantine/notifications";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useRouter} from "next/navigation";

interface CreateTaskCodeModalProps {
    onClose: () => void;
    assignment: Assignment;
}

const CreateSolutionModal = ({onClose, assignment}: CreateTaskCodeModalProps) => {

    const [files, setFiles] = useState<FileWithPath[]>([]);
    const api = useApiServiceClient();
    const router = useRouter();

    const requiredFiles = useMemo<string[]>(
        () => assignment.file_structure !== null ? extractFilesFromFileStructure(assignment.file_structure) : [],
        [assignment.file_structure]
    );

    const missingFiles = useMemo<string[]>(
        () => {
            const uploaded = files.map((f) => f.name);
            return requiredFiles.filter((f) => !uploaded.includes(f));
        },
        [files, requiredFiles]
    );

    const submit = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (missingFiles.length > 0) {
            notifications.show({
                title: 'Missing file',
                message: `Following files are missing: ${missingFiles.join(', ')}`,
                color: 'red'
            });
            return;
        }
        const resp = await api.createSolution(assignment.id, files);
        router.push(`/solutions/${resp.id}`);
    }

    return (
      <Modal opened onClose={onClose} title="Create new Solution" size="lg">
            <form onSubmit={submit}>
                {missingFiles.length > 0 && (
                    <Paper withBorder mb={20} p={10}>
                        <List>
                            {missingFiles.map((f) => (
                                <List.Item key={f}>{f}</List.Item>
                            ))}
                        </List>
                    </Paper>
                )}
                <InternalDropzone files={files} setFiles={setFiles} />
                <Group mt={10}>
                    <Button type="submit">Create Solution</Button>
                    <Button onClick={onClose} color="gray">Cancel</Button>
                </Group>
            </form>
      </Modal>
    );
}

export default CreateSolutionModal;
