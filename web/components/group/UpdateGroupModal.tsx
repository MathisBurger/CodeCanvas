import {useTranslation} from "react-i18next";
import {useForm} from "@mantine/form";
import {GroupJoinRequestPolicy} from "@/service/types/tasky";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {notifications} from "@mantine/notifications";
import {Button, Group, Modal, Select, Stack, TextInput} from "@mantine/core";
import {Group as TaskyGroup} from "@/service/types/tasky";

interface UpdateGroupModalProps {
    group: TaskyGroup;
    onClose: () => void;
    refetch: () => void;
}

const UpdateGroupModal = ({group, onClose, refetch}: UpdateGroupModalProps) => {

    const { t } = useTranslation("common");

    const form = useForm({
        initialValues: {
            title: group.title,
            join_policy: group.join_policy
        },
        validate: {
            title: (val) => (val.trim() == "" ? t("errors.title-empty") : null),
        },
    });
    const api = useApiServiceClient();

    const submit = form.onSubmit(async (values) => {
        try {
            await api.updateGroup(group.id, values.title, values.join_policy);
            refetch();
            onClose();
        } catch (e: any) {
            notifications.show({
                title: t("messages.error"),
                message: e?.message ?? t("errors.update-group"),
            });
        }
    });

    return (
        <Modal opened onClose={onClose} title={t("titles.update-group")}>
            <form onSubmit={submit}>
                <Stack gap={10}>
                    <TextInput
                        label={t("fields.title")}
                        key={form.key("title")}
                        {...form.getInputProps("title")}
                    />
                    <Select
                        label={t('group:cols.join-policy')}
                        key={form.key("join_policy")}
                        {...form.getInputProps("join_policy")}
                        data={[
                            {value: GroupJoinRequestPolicy.Request, label: t('group:join-policy.request')},
                            {value: GroupJoinRequestPolicy.Open, label: t('group:join-policy.open')},
                            {value: GroupJoinRequestPolicy.Closed, label: t('group:join-policy.closed')}
                        ]}
                    />
                </Stack>
                <Group mt={10}>
                    <Button type="submit">{t("actions.save")}</Button>
                    <Button onClick={onClose} color="gray">
                        {t("actions.cancel")}
                    </Button>
                </Group>
            </form>
        </Modal>
    );
}

export default UpdateGroupModal;
