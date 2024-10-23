import {createContext, useContext, useMemo} from "react";
import {Assignment, Group, Solution} from "@/service/types/tasky";
import {SpotlightActionData, SpotlightActionGroupData} from "@mantine/spotlight";
import useCurrentUser from "@/hooks/useCurrentUser";
import {useRouter} from "next/navigation";

/**
 * User boundary type to ensure data is user account specific if user uses two different accounts in one browser
 */
type UserRelatedType = {userId: number|undefined, die: Date};

/**
 * Data type that is the schema of the stored data
 */
export interface Stage2Type {
    groups: (UserRelatedType & Group)[];
    assignments: (UserRelatedType & Assignment & {groupId: number})[];
    solutions: (UserRelatedType & Solution)[];
}

export interface Stage2ContextType {
    content: Stage2Type;
    setContent: (content: Stage2Type) => void;
}

/**
 * Contains context data for stage2 spotlight search
 */
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
    })), [data, router, user?.id]);

    const assignments = useMemo<SpotlightActionData[]>(() => data.assignments.filter((a) => a.userId === user?.id).map((a) => ({
        id: `assignment-${a.id}-${a.userId}`,
        label: a.title,
        description: "",
        onClick: () => router.push(`/groups/${a.groupId}/assignments/${a.id}`),
    })), [data, router, user?.id]);

    const solutions = useMemo<SpotlightActionData[]>(() => data.solutions.filter((s) => s.userId === user?.id).map((s) => ({
        id: `assignment-${s.id}-${s.userId}`,
        label: `${s.submitter.username} - ${s.assignment.title}`,
        description: "",
        onClick: () => router.push(`/solutions/${s.id}`),
    })), [data, router, user?.id]);


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

/**
 * Provides functions to add entities to spotlight search
 */
export const useSpotlightStage2 = () => {

    const context = useContext(Stage2Context);
    const {user} = useCurrentUser();

    /**
     * Adds a group to spotlight
     *
     * @param group The group that should be added
     */
    const addGroup = (group: Group) => {
        if (undefined === context.content.groups.find((g) => g.userId === user?.id && g.id === group.id)) {
            const copy = Object.assign({}, context.content);
            const die = new Date();
            die.setDate(new Date().getDate() +7);
            copy.groups.push({
                ...group,
                userId: user?.id,
                die
            });
            context.setContent(copy);
        }
    }

    /**
     * Adds an assignment to spotlight
     *
     * @param assignment The assignment that should be added
     * @param groupId Group ID of the assignment
     */
    const addAssignment = (assignment: Assignment, groupId: number) => {
        if (undefined === context.content.assignments.find((g) => g.userId === user?.id && g.id === assignment.id)) {
            const copy = Object.assign({}, context.content);
            const die = new Date();
            die.setDate(new Date().getDate() +7);
            copy.assignments.push({
                ...assignment,
                userId: user?.id,
                groupId,
                die
            });
            context.setContent(copy);
        }
    }

    /**
     * Adds a solution that should be added
     *
     * @param solution The solution that should be added
     */
    const addSolution = (solution: Solution) => {
        if (undefined === context.content.solutions.find((g) => g.userId === user?.id && g.id === solution.id)) {
            const copy = Object.assign({}, context.content);
            const die = new Date();
            die.setDate(new Date().getDate() +7);
            copy.solutions.push({
                ...solution,
                userId: user?.id,
                die
            });
            context.setContent(copy);
        }
    }

    return {addGroup, addAssignment, addSolution};
}
