import {
  SpotlightActionData,
  SpotlightActionGroupData,
} from "@mantine/spotlight";
import { useMemo, useState } from "react";
import {
  useStaticGeneralActions,
  useStaticRoutesActions,
} from "@/hooks/spotlight/staticProvider";

const useActionsFactory = (): (
  | SpotlightActionGroupData
  | SpotlightActionData
)[] => {
  const staticRoutes = useStaticRoutesActions();
  const staticGeneral = useStaticGeneralActions();

  const staticActions = useMemo<
    (SpotlightActionGroupData | SpotlightActionData)[]
  >(() => [...staticRoutes, ...staticGeneral], [staticRoutes, staticGeneral]);

  const [actions, setActions] =
    useState<(SpotlightActionGroupData | SpotlightActionData)[]>(staticActions);

  return actions;
};

export default useActionsFactory;
