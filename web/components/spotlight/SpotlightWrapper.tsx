import useActionsFactory from "@/hooks/spotlight/useActionsFactory";
import { rem } from "@mantine/core";
import { Spotlight } from "@mantine/spotlight";
import { IconSearch } from "@tabler/icons-react";
import {useTranslation} from "react-i18next";

const SpotlightWrapper = () => {
  const actions = useActionsFactory();
  const {t} = useTranslation('common')

  return (
    <Spotlight
      actions={actions}
      nothingFound={t('spotlight.nothing-found')}
      highlightQuery
      limit={20}
      searchProps={{
        leftSection: (
          <IconSearch
            style={{ width: rem(20), height: rem(20) }}
            stroke={1.5}
          />
        ),
        placeholder: t('spotlight.search'),
      }}
    />
  );
};

export default SpotlightWrapper;
