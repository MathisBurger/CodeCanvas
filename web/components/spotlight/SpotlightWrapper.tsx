import useActionsFactory from "@/hooks/spotlight/useActionsFactory";
import { rem } from "@mantine/core";
import { Spotlight } from "@mantine/spotlight";
import { IconSearch } from "@tabler/icons-react";
import {useTranslation} from "react-i18next";
import {useState} from "react";

const SpotlightWrapper = () => {
  const [search, setSearch] = useState<string>("");
  const actions = useActionsFactory(search);
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
          value: search,
          onChange: (e) => setSearch(e.target.value),
        placeholder: t('spotlight.search'),
      }}
    />
  );
};

export default SpotlightWrapper;
