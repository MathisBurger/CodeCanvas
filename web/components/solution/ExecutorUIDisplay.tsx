import { Modal } from "@mantine/core";
import getConfig from "next/config";

export interface ExecutorUIDisplayProps {
  jobId: string;
  onClose: () => void;
}

const ExecutorUIDisplay = ({ jobId, onClose }: ExecutorUIDisplayProps) => {
  const apiUrl =
    getConfig().publicRuntimeConfig.EXECUTOR_UI_URL === ""
      ? "http://localhost:3008"
      : getConfig().publicRuntimeConfig.EXECUTOR_UI_URL;
  return (
    <Modal opened onClose={onClose} size="100%">
      <iframe
        height="500"
        width="100%"
        src={`${apiUrl}/jobs/${jobId}`}
      ></iframe>
    </Modal>
  );
};

export default ExecutorUIDisplay;
