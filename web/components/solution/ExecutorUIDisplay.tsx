import {Modal} from "@mantine/core";

export interface ExecutorUIDisplayProps {
    jobId: string;
    onClose: () => void;
}

const ExecutorUIDisplay = ({jobId, onClose}: ExecutorUIDisplayProps) => {

    const apiUrl = process.env.EXECUTOR_UI_URL ?? "http://localhost:3007";

    return (
        <Modal opened onClose={onClose} size="100%">
            <iframe height="500" width="100%" src={`${apiUrl}/jobs/${jobId}`}>
            </iframe>
        </Modal>
    );
}

export default ExecutorUIDisplay;
