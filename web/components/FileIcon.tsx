import {IconFile} from "@tabler/icons-react";
import { CssIcon, NpmIcon, TypeScriptCircleIcon } from '@mantinex/dev-icons';
import { IconFolder, IconFolderOpen } from '@tabler/icons-react';
import {GoIcon, JavaIcon} from "@/components/Icons";


interface FileIconProps {
    name: string;
    isFolder: boolean;
    expanded: boolean;
}

const FileIcon = ({name, isFolder, expanded}: FileIconProps) => {
    if (name.endsWith('package.json')) {
        return <NpmIcon size={18} />;
    }

    if (name.endsWith('.ts') || name.endsWith('.tsx') || name.endsWith('tsconfig.json')) {
        return <TypeScriptCircleIcon size={18} />;
    }

    if (name.endsWith('.css')) {
        return <CssIcon size={18} />;
    }
    if (name.endsWith('.java')) {
        return <JavaIcon size={18} />;
    }
    if (name.endsWith('.go')) {
        return <GoIcon size={18} />;
    }

    if (isFolder) {
        return expanded ? (
            <IconFolderOpen color="var(--mantine-color-yellow-9)" size={18} stroke={2.5} />
        ) : (
            <IconFolder color="var(--mantine-color-yellow-9)" size={18} stroke={2.5} />
        );
    }

    return <IconFile size={18} />;
}

export default FileIcon;
