import {MongoTestFile} from "@/service/types/tasky";
import {Card, Text} from "@mantine/core";
import {CodeHighlight} from "@mantine/code-highlight";
import { useMemo } from "react";

interface CodeDisplayProps {
    file: MongoTestFile|null;
}


const CodeDisplay = ({file}: CodeDisplayProps) => {

    const language = useMemo<string|undefined>(() => {
        if (null === file) return undefined;
        switch (file.file_name.split('.').pop()) {
            case "java":
                return "java";
            case "go":
                return "golang";
            case "json":
                return "json";
            default:
                return file.file_name.split(".").pop();
        }
    }, [file])

    if (null === file) {
        return (
            <Card>
                <Text>No file selected</Text>
            </Card>
        )
    }

    return (
        <Card>
            <CodeHighlight code={file?.content} language={language} copyLabel="Copy Code" copiedLabel="Copied!" />
        </Card>
    );
}

export default CodeDisplay;
