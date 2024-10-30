import useApiServiceClient from "@/hooks/useApiServiceClient";
import useClientQuery from "@/hooks/useClientQuery";
import { Solution } from "@/service/types/tasky";
import {Badge, Button, Card, Group, Stack, Title} from "@mantine/core";
import AssignmentDateDisplay from "@/components/assignments/AssignmentDateDisplay";
import RichTextDisplay from "@/components/display/RichTextDisplay";
import useCurrentUser from "@/hooks/useCurrentUser";
import {IconPlus} from "@tabler/icons-react";
import {useState} from "react";
import CreateCommentModal from "@/components/solution/CreateCommentModal";

interface CommentTabProps {
    solution: Solution;
}

const CommentTab = ({solution}: CommentTabProps) => {

    const api = useApiServiceClient();
    const {user} = useCurrentUser();
    const [createModalOpen, setCreateModalOpen] = useState(false);
    const [comments, refetch] = useClientQuery(() => api.getCodeComments(solution.id));

    return (
        <>
            <Stack gap={10}>
                <Group justify="flex-end">
                    <Button onClick={() => setCreateModalOpen(true)}><IconPlus />
                        &nbsp;Create Comment</Button>
                </Group>
                {(comments ?? []).map((comment) => (
                    <Card shadow="sm" padding="lg" radius="md" withBorder key={comment.id}>
                        <Group>
                            <Title order={4}>{comment.title}</Title>
                            {comment.commentor === user?.id && (
                                <Badge color="green">Your comment</Badge>
                            )}
                        </Group>
                        <RichTextDisplay content={comment.content} fullSize={false} />
                    </Card>
                ))}
            </Stack>
            {createModalOpen && (
                <CreateCommentModal
                    solution={solution}
                    refetch={refetch}
                    onClose={() => setCreateModalOpen(false)}
                />
            )}
        </>
    );
}

export default CommentTab;
