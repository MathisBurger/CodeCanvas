import {Autocomplete, Button, Group, Modal, Stack} from "@mantine/core";
import {useTranslation} from "react-i18next";
import {useForm} from "@mantine/form";
import {useEffect, useMemo, useState} from "react";
import {TaskyUser} from "@/service/types/tasky";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useDebouncedValue} from "@mantine/hooks";
import {showNotification} from "@mantine/notifications";

interface EnlistUserModalProps {
    onClose: () => void;
    groupId: number;
    refetch: () => void;
}

const EnlistUserModal = ({onClose, groupId, refetch}: EnlistUserModalProps) => {

    const {t} = useTranslation(['common', 'group']);
    const api = useApiServiceClient();
    const [enlistable, setEnlistable] = useState<TaskyUser[]>([]);

    const usernameMap = useMemo<Map<string, number>>(() => {
        const map = new Map<string, number>();
        for (const usr of enlistable) {
            map.set(usr.username, usr.id);
        }
        return map;
    }, [enlistable]);

    const usernames = useMemo<string[]>(() => enlistable.map((e) => e.username), [enlistable]);

    const form = useForm({
        initialValues: {
            username: "",
        },
        validate: {
            username: (v) => !usernameMap.has(v) ? t('group:messages.unknown-user') : null,
        }
    });

    const [debouncedSearch] = useDebouncedValue(form.values.username, 500);

    useEffect(() => {
        api.searchUsersToEnlist(groupId, debouncedSearch)
            .then((users) => setEnlistable(users));
    }, [debouncedSearch, groupId]);

    const submit = form.onSubmit( async (values) => {
        try {
            await api.enlistUser(groupId, usernameMap.get(values.username)!);
            refetch();
            onClose();
        } catch (e: any) {
            showNotification({
                title: t('common:messages.error'),
                message: e.message ?? "",
            });
        }
    });


    return (
        <Modal opened onClose={onClose} title={t('group:actions.enlist-user')}>
            <form onSubmit={submit}>
                <Stack gap={10}>
                    <Autocomplete
                        label={t('common:fields.username')}
                        maxDropdownHeight={200}
                        data={usernames}
                        key={form.key("username")}
                        {...form.getInputProps("username")}
                    />
                    <Group>
                        <Button type="submit" disabled={enlistable.length === 0}>{t("group:actions.enlist")}</Button>
                        <Button onClick={onClose} color="gray">
                            {t("common:actions.cancel")}
                        </Button>
                    </Group>
                </Stack>
            </form>
        </Modal>
    );
}

export default EnlistUserModal;
