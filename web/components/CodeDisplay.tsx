import { MongoTaskFile, MongoTestFile } from "@/service/types/tasky";
import { Card, Text } from "@mantine/core";
import {CodeHighlightTabs} from "@mantine/code-highlight";
import {useCallback, useEffect, useState} from "react";
import FileIcon from "@/components/FileIcon";
import {useTranslation} from "react-i18next";

interface CodeDisplayProps {
  files: (MongoTestFile | MongoTaskFile)[];
}

const CodeDisplay = ({ files }: CodeDisplayProps) => {

  const [tab, setTab] = useState<number|undefined>(undefined);
  const {t} = useTranslation('common');

  useEffect(() => {
    if (files.length > 0) {
      setTab(files.length-1)
    }
  }, [files]);

  const language = useCallback((file: MongoTestFile | MongoTaskFile) => {
    if (null === file) return undefined;
    switch (file.file_name.split(".").pop()) {
      case "java":
        return "java";
      case "go":
        return "golang";
      case "json":
        return "json";
      default:
        return file.file_name.split(".").pop();
    }
  }, []);

  if (files.length === 0) {
    return (
      <Card>
        <Text>{t('messages.no-files-selected')}</Text>
      </Card>
    );
  }

  return (
    <Card>
      <CodeHighlightTabs
          activeTab={tab}
          onTabChange={setTab}
        code={files.map((file: MongoTestFile|MongoTaskFile) => ({
          fileName: file.file_name,
            language: language(file),
            code: file.content ?? "",
            icon: <FileIcon
                name={file.file_name}
                isFolder={false}
                expanded={false}
            />
        }))}
          withExpandButton
        copyLabel={t('actions.copy')}
        copiedLabel={t('messages.copied-code')}
      />
    </Card>
  );
};

export default CodeDisplay;
