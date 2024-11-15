import {Button, Group, Modal, Text} from "@mantine/core";
import {useTranslation} from "react-i18next";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useRouter} from "next/navigation";
import {showNotification} from "@mantine/notifications";


interface DeleteGroupModalProps {
    groupId: number;
    onClose: () => void;
}

const DeleteGroupModal = ({groupId, onClose}: DeleteGroupModalProps) => {

    const {t} = useTranslation(['group', 'common']);
    const api = useApiServiceClient();
    const router = useRouter();

    const deleteGroup = async () => {
        try {
            await api.deleteGroup(groupId);
            showNotification({
                title: t('common:messages.success'),
                message: t('group:messages.delete-group')
            });
            router.push('/my-groups');
        } catch (e: any) {
            showNotification({
                title: t('common:messages.error'),
                message: e?.message ?? "",
            })
        }
    }

    return (
        <Modal opened onClose={onClose} title={t('common:actions.delete')}>
            <Text>
                {t('group:text.delete-group')}
            </Text>
            <Group mt={10}>
                <Button onClick={deleteGroup} color="red">{t("common:actions.delete")}</Button>
                <Button onClick={onClose} color="gray">
                    {t("common:actions.cancel")}
                </Button>
            </Group>
        </Modal>
    );
}

export default DeleteGroupModal;
