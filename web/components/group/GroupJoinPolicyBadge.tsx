import {GroupJoinRequestPolicy} from "@/service/types/tasky";
import {Badge} from "@mantine/core";
import {useTranslation} from "react-i18next";

interface GroupJoinPolicyBadgeProps {
    policy: string;
}

const GroupJoinPolicyBadge = ({policy}: GroupJoinPolicyBadgeProps) => {

    const {t} = useTranslation('group');

    switch (policy) {
        case GroupJoinRequestPolicy.Request:
            return <Badge color="indigo">{t('join-policy.request')}</Badge>;
        case GroupJoinRequestPolicy.Closed:
            return <Badge color="red">{t('join-policy.closed')}</Badge>;
        case GroupJoinRequestPolicy.Open:
            return <Badge color="green">{t('join-policy.open')}</Badge>;
        default:
            return null;
    }
}

export default GroupJoinPolicyBadge;
