import useActionsFactory from "@/hooks/spotlight/useActionsFactory";
import { rem } from "@mantine/core";
import {
  Spotlight,
  SpotlightActionGroupData,
  SpotlightFilterFunction,
} from "@mantine/spotlight";
import { IconSearch } from "@tabler/icons-react";
import { useTranslation } from "react-i18next";
import { useState } from "react";
import { SpotlightActionData } from "@mantine/spotlight";

const SpotlightWrapper = () => {
  const [search, setSearch] = useState<string>("");
  const actions = useActionsFactory(search);
  const { t } = useTranslation("common");

  const filter: SpotlightFilterFunction = (
    query: string,
    actions: (SpotlightActionGroupData | SpotlightActionData)[],
  ): (SpotlightActionGroupData | SpotlightActionData)[] => {
    return actions.filter((group: any) => {
      group.actions = group.actions.filter((action: any) => {
        return (
          action.id.indexOf("stage3") > -1 ||
          (action.title ?? "").indexOf(query) > -1 ||
          (action.description ?? "").indexOf(query) > -1
        );
      });
      return group;
    });
  };

  return (
    <Spotlight
      actions={actions}
      nothingFound={t("spotlight.nothing-found")}
      highlightQuery
      filter={filter}
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
        placeholder: t("spotlight.search"),
      }}
    />
  );
};

export default SpotlightWrapper;
