import {Button, Group, Modal, Text} from "@mantine/core";
import {useTranslation} from "react-i18next";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import {showNotification} from "@mantine/notifications";

interface SwitchToTutorModalProps {
    onClose: () => void;
}

const SwitchToTutorModal = ({onClose}: SwitchToTutorModalProps) => {

    const {t} = useTranslation('common');
    const api = useApiServiceClient();

    const switchAccount = async () => {
        try {
            await api.switchToTutorAccount();
            document.cookie = 'session=""';
            window.location.reload();
        } catch (e: any) {
            showNotification({
                title: t('common:messages.error'),
                message: e?.message ?? "" ,
            })
        }
    }

    return (
        <Modal opened onClose={onClose}>
            <Text>
                {t('common:messages.switch-to-tutor-text')}
            </Text>
            <Group mt={10}>
                <Button onClick={switchAccount}>{t("actions.save")}</Button>
                <Button onClick={onClose} color="gray">
                    {t("actions.cancel")}
                </Button>
            </Group>
        </Modal>
    );
}

export default SwitchToTutorModal;
