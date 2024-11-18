import {useTranslation} from "react-i18next";
import {Badge} from "@mantine/core";
import {Tooltip} from "@mantine/core";


const VerifiedBadge = () => {

    const {t} = useTranslation("group");

    return (
        <Tooltip label={t('group:text.verified-tooltip')}>
            <Badge color="cyan">{t('group:cols.verified')} ✓</Badge>
        </Tooltip>
    );
}

export default VerifiedBadge;
