import {IconPlus} from "@tabler/icons-react";
import {Checkbox, FocusTrap, Group, GroupProps, TextInput} from "@mantine/core";
import {useState} from "react";
import FileIcon from "@/components/FileIcon";
import {useForm} from "@mantine/form";

interface FileStructureNewInputProps {
    label: string;
    addFunc: (elementName: string) => void;
    fileNames: string[];
}

export const FileStructureNewInput = (props: FileStructureNewInputProps & GroupProps) => {

    const {addFunc: _, label: _1, fileNames: _2, ...groupProps} = props;
    const [isNew, setIsNew] = useState(false);
    const form = useForm({
        initialValues: {
            name: ''
        },
        validate: {
            name: (value: string) => props.fileNames.indexOf(value) > -1 ? 'File/Folder already exists' : null
        }
    });

    const submit = form.onSubmit((values) => {
        props.addFunc(values.name);
        setIsNew(false);
        form.setFieldValue("name", "");
    })


    return (
        <Group gap={5} {...groupProps} onClick={() => setIsNew(true)}>
            <IconPlus size={18} />
            {isNew ? (
                <form onSubmit={submit}>
                    <FocusTrap active>
                        <TextInput key={form.key('name')} {...form.getInputProps('name')} />
                    </FocusTrap>
                </form>
            ) : (
                <span>{props.label}</span>
            )}
        </Group>
    )
}

interface FileStructureElementProps {
    label: string;
    isFolder: boolean;
    expanded: boolean;
}

export const FileStructureElement = (props: FileStructureElementProps & GroupProps) => {

    const {label:_, isFolder: _1, expanded: _2, ...elementProps} = props;

    return (
        <Group gap={5} {...elementProps}>
            <FileIcon name={props.label} isFolder={props.isFolder}
                      expanded={props.expanded}/>
            <span>{props.label}</span>
            {!props.isFolder && (
                <Checkbox label="Test file" style={{justifySelf: 'flex-end'}} />
            )}
        </Group>
    )
}
