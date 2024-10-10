import useActionsFactory from "@/hooks/spotlight/useActionsFactory";
import { rem } from "@mantine/core";
import { Spotlight } from "@mantine/spotlight";
import { IconSearch } from "@tabler/icons-react";

const SpotlightWrapper = () => {
  const actions = useActionsFactory();

  return (
    <Spotlight
      actions={actions}
      nothingFound="Nothing found..."
      highlightQuery
      limit={20}
      searchProps={{
        leftSection: (
          <IconSearch
            style={{ width: rem(20), height: rem(20) }}
            stroke={1.5}
          />
        ),
        placeholder: "Search...",
      }}
    />
  );
};

export default SpotlightWrapper;
