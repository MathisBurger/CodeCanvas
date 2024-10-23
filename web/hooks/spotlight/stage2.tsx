import {createContext, useContext, useMemo} from "react";
import {Assignment, Group, Solution} from "@/service/types/tasky";
import {SpotlightActionData, SpotlightActionGroupData} from "@mantine/spotlight";
import useCurrentUser from "@/hooks/useCurrentUser";
import {useRouter} from "next/navigation";

type UserRelatedType = {userId: number|undefined};

export interface Stage2Type {
    groups: (UserRelatedType & Group)[];
    assignments: (UserRelatedType & Assignment & {groupId: number})[];
    solutions: (UserRelatedType & Solution)[];
}

export interface Stage2ContextType {
    content: Stage2Type;
    setContent: (content: Stage2Type) => void;
}


export const Stage2Context = createContext<Stage2ContextType>({content: {groups: [], assignments: [], solutions: []}, setContent: () => {}});

/**
 * Used to fetch stage2 actions for spotlight
 */
export const useStage2Actions = (): SpotlightActionGroupData[] => {
    const context = useContext(Stage2Context);
    const data = context.content;
    const {user} = useCurrentUser();
    const router = useRouter();

    const groups = useMemo<SpotlightActionData[]>(() => data.groups.filter((g) => g.userId === user?.id).map((g) => ({
        id: `group-${g.id}-${g.userId}`,
        label: g.title,
        description: "",
        onClick: () => router.push(`/groups/${g.id}`),
    })), [data.groups, router, user?.id]);

    const assignments = useMemo<SpotlightActionData[]>(() => data.assignments.filter((a) => a.userId === user?.id).map((a) => ({
        id: `assignment-${a.id}-${a.userId}`,
        label: a.title,
        description: "",
        onClick: () => router.push(`/groups/${a.groupId}/assignments/${a.id}`),
    })), [data.assignments, router, user?.id]);

    const solutions = useMemo<SpotlightActionData[]>(() => data.solutions.filter((s) => s.userId === user?.id).map((s) => ({
        id: `assignment-${s.id}-${s.userId}`,
        label: `${s.submitter.username} - ${s.assignment.title}`,
        description: "",
        onClick: () => router.push(`/solutions/${s.id}`),
    })), [data.solutions, router, user?.id]);


    return [
        {
            group: "Groups",
            actions: groups
        },
        {
            group: "Assignments",
            actions: assignments
        },
        {
            group: "Solutions",
            actions: solutions
        }
    ];
}

export const useSpotlightStage2 = () => {

    const context = useContext(Stage2Context);
    const {user} = useCurrentUser();

    const addGroup = (group: Group) => {
        if (undefined === context.content.groups.find((g) => g.userId === user?.id && g.id === group.id)) {
            const copy = Object.assign({}, context.content);
            copy.groups.push({
                ...group,
                userId: user?.id
            });
            context.setContent(copy);
        }
    }

    return {addGroup};
}
