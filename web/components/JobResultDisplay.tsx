import { Job } from "@/service/types/tasky";
import {Code, Highlight, Paper, TextInput} from "@mantine/core";
import { useState } from "react";

interface JobResultDisplayProps {
    job: Job;
}

const JobResultDisplay = ({job}: JobResultDisplayProps) => {
    const [highlight, setHighlight] = useState("");
    return (
        <Paper withBorder p={5}>
            <TextInput mb={10} value={highlight} onChange={(e) => setHighlight(e.target.value as string)} />
            <Highlight highlight={highlight} style={{whiteSpace: 'pre-line'}}>
                {job.execution[0].error ?? job.execution[0].result ?? ""}
            </Highlight>
        </Paper>
    );
}

export default JobResultDisplay;
