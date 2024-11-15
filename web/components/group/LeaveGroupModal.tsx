import {Button, Group, Modal, Text} from "@mantine/core";
import {useTranslation} from "react-i18next";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useRouter} from "next/navigation";
import {showNotification} from "@mantine/notifications";


interface LeaveGroupModalProps {
    groupId: number;
    onClose: () => void;
}

const LeaveGroupModal = ({groupId, onClose}: LeaveGroupModalProps) => {

    const {t} = useTranslation(['group', 'common']);
    const api = useApiServiceClient();
    const router = useRouter();

    const leave = async () => {
        try {
            await api.leaveGroup(groupId);
            showNotification({
                title: t('common:messages.success'),
                message: t('group:messages.left-group')
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
        <Modal opened onClose={onClose} title={t('group:actions.leave')}>
            <Text>
                {t('group:text.leave-group')}
            </Text>
            <Group mt={10}>
                <Button onClick={leave} color="red">{t("group:actions.leave")}</Button>
                <Button onClick={onClose} color="gray">
                    {t("common:actions.cancel")}
                </Button>
            </Group>
        </Modal>
    );
}

export default LeaveGroupModal;
