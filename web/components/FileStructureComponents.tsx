import {IconPlus, IconTrash} from "@tabler/icons-react";
import {
  Checkbox,
  FocusTrap,
  Group,
  GroupProps,
  TextInput,
  Text, ActionIcon,
} from "@mantine/core";
import { useState } from "react";
import FileIcon from "@/components/FileIcon";
import { useForm } from "@mantine/form";
import {useTranslation} from "react-i18next";
import {FileStructureTree} from "@/components/FileStructure";
import {removeFile} from "@/utils/FileStructure";

interface FileStructureNewInputProps {
  label: string;
  addFunc: (elementName: string) => void;
  fileNames: string[];
}

export const FileStructureNewInput = (
  props: FileStructureNewInputProps & GroupProps,
) => {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { addFunc: _, label: _1, fileNames: _2, ...groupProps } = props;
  const {t} = useTranslation('common');
  const [isNew, setIsNew] = useState(false);
  const form = useForm({
    initialValues: {
      name: "",
    },
    validate: {
      name: (value: string) =>
        props.fileNames.indexOf(value) > -1
          ? t('errors.folder-exists')
          : null,
    },
  });

  const submit = form.onSubmit((values) => {
    props.addFunc(values.name);
    setIsNew(false);
    form.setFieldValue("name", "");
  });

  return (
    <Group gap={5} {...groupProps} onClick={() => setIsNew(true)}>
      <IconPlus size={18} />
      {isNew ? (
        <form onSubmit={submit}>
          <FocusTrap active>
            <TextInput key={form.key("name")} {...form.getInputProps("name")} />
          </FocusTrap>
        </form>
      ) : (
        <span>{props.label}</span>
      )}
    </Group>
  );
};

interface FileStructureElementProps {
  label: string;
  isTestFile: boolean;
  isFolder: boolean;
  expanded: boolean;
  setIsTestFile: (is: boolean) => void;
  fileStructure: FileStructureTree;
  setFileStructure: (structure: FileStructureTree) => void;
  editable: boolean;
}

export const FileStructureElement = (
  props: FileStructureElementProps & GroupProps,
) => {
  /* eslint-disable @typescript-eslint/no-unused-vars */
  const {
    label: _,
    isFolder: _1,
    expanded: _2,
    isTestFile: _3,
    setIsTestFile: _4,
    editable: _5,
    setFileStructure: _6,
    fileStructure: _7,
    ...elementProps
  } = props;
  /* eslint-enable @typescript-eslint/no-unused-vars */

  const {t} = useTranslation('common');
  return (
    <Group gap={5} {...elementProps}>
      <FileIcon
        name={props.label}
        isFolder={props.isFolder}
        expanded={props.expanded}
      />
      <Text>{props.label}</Text>
      <Group style={{marginLeft: "auto"}}>
        {!props.isFolder && props.editable && (
            <Checkbox
                label={t('fields.test-file')}
                checked={props.isTestFile}
                onChange={(e) => props.setIsTestFile(e.target.checked)}

            />
        )}
        {props.editable && (
            <ActionIcon
                variant="light"
                color="red"
                style={{ marginLeft: "auto" }}
                onClick={() => props.setFileStructure(removeFile(props.fileStructure, props.label, props.isFolder))}
            >
              <IconTrash />
            </ActionIcon>
        )}
      </Group>
    </Group>
  );
};
