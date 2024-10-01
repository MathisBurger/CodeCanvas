import { RichTextEditor, Link, getTaskListExtension } from '@mantine/tiptap';
import {BubbleMenu, useEditor } from '@tiptap/react';
import Highlight from '@tiptap/extension-highlight';
import StarterKit from '@tiptap/starter-kit';
import Underline from '@tiptap/extension-underline';
import TextAlign from '@tiptap/extension-text-align';
import Superscript from '@tiptap/extension-superscript';
import SubScript from '@tiptap/extension-subscript';
import { Color } from '@tiptap/extension-color';
import TextStyle from '@tiptap/extension-text-style';
import {IconColorPicker} from "@tabler/icons-react";
import {CodeBlockLowlight} from "@tiptap/extension-code-block-lowlight";
import { lowlightFactory } from '@/utils/lowlight';
import TaskItem from '@tiptap/extension-task-item';
import TipTapTaskList from '@tiptap/extension-task-list';

interface RichTextInputProps {
    content: string;
    setContent: (content: string) => void;
}

const RichTextInput = ({content, setContent}: RichTextInputProps) => {



    const editor = useEditor({
        extensions: [
            StarterKit.configure({codeBlock: false}),
            Underline,
            Link,
            Superscript,
            SubScript,
            Highlight,
            Color,
            TextStyle,
            TextAlign.configure({ types: ['heading', 'paragraph'] }),
            CodeBlockLowlight.configure({ lowlight: lowlightFactory() }),
            getTaskListExtension(TipTapTaskList),
            TaskItem.configure({
                nested: true,
                HTMLAttributes: {
                    class: 'test-item',
                },
            }),
        ],
        content,
        immediatelyRender: false,
        onUpdate(props) {
            setContent(props.editor.getHTML());
        }
    });

    return (
        <RichTextEditor editor={editor}>
            <RichTextEditor.Toolbar sticky stickyOffset={60}>
                <RichTextEditor.ColorPicker
                    colors={[
                        '#25262b',
                        '#868e96',
                        '#fa5252',
                        '#e64980',
                        '#be4bdb',
                        '#7950f2',
                        '#4c6ef5',
                        '#228be6',
                        '#15aabf',
                        '#12b886',
                        '#40c057',
                        '#82c91e',
                        '#fab005',
                        '#fd7e14',
                    ]}
                />
                <RichTextEditor.ControlsGroup>
                    <RichTextEditor.Control interactive={false}>
                        <IconColorPicker size="1rem" stroke={1.5} />
                    </RichTextEditor.Control>
                    <RichTextEditor.Color color="#F03E3E" />
                    <RichTextEditor.Color color="#7048E8" />
                    <RichTextEditor.Color color="#1098AD" />
                    <RichTextEditor.Color color="#37B24D" />
                    <RichTextEditor.Color color="#F59F00" />
                </RichTextEditor.ControlsGroup>
                <RichTextEditor.ControlsGroup>
                    <RichTextEditor.Bold />
                    <RichTextEditor.Italic />
                    <RichTextEditor.Underline />
                    <RichTextEditor.Strikethrough />
                    <RichTextEditor.ClearFormatting />
                    <RichTextEditor.Highlight />
                    <RichTextEditor.Code />
                    <RichTextEditor.CodeBlock />
                </RichTextEditor.ControlsGroup>

                <RichTextEditor.ControlsGroup>
                    <RichTextEditor.H1 />
                    <RichTextEditor.H2 />
                    <RichTextEditor.H3 />
                    <RichTextEditor.H4 />
                </RichTextEditor.ControlsGroup>

                <RichTextEditor.ControlsGroup>
                    <RichTextEditor.Blockquote />
                    <RichTextEditor.Hr />
                    <RichTextEditor.BulletList />
                    <RichTextEditor.OrderedList />
                    <RichTextEditor.Subscript />
                    <RichTextEditor.Superscript />
                </RichTextEditor.ControlsGroup>

                <RichTextEditor.ControlsGroup>
                    <RichTextEditor.Link />
                    <RichTextEditor.Unlink />
                </RichTextEditor.ControlsGroup>

                <RichTextEditor.ControlsGroup>
                    <RichTextEditor.AlignLeft />
                    <RichTextEditor.AlignCenter />
                    <RichTextEditor.AlignJustify />
                    <RichTextEditor.AlignRight />
                </RichTextEditor.ControlsGroup>

                <RichTextEditor.ControlsGroup>
                    <RichTextEditor.Undo />
                    <RichTextEditor.Redo />
                </RichTextEditor.ControlsGroup>
            </RichTextEditor.Toolbar>

            <RichTextEditor.Content />
        </RichTextEditor>
    );
}

export default RichTextInput;
