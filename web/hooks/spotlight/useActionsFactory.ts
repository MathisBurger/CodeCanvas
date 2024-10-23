import {
  SpotlightActionData,
  SpotlightActionGroupData,
} from "@mantine/spotlight";
import { useMemo, useState } from "react";
import {
  useStaticGeneralActions,
  useStaticRoutesActions,
} from "@/hooks/spotlight/staticProvider";
import {useStage2Actions} from "@/hooks/spotlight/stage2";

const useActionsFactory = (): (
  | SpotlightActionGroupData
  | SpotlightActionData
)[] => {
  const staticRoutes = useStaticRoutesActions();
  const staticGeneral = useStaticGeneralActions();
  const stage2Actions = useStage2Actions();

  const staticActions = useMemo<
    (SpotlightActionGroupData | SpotlightActionData)[]
  >(() => [...staticRoutes, ...staticGeneral, ...stage2Actions], [staticRoutes, staticGeneral]);

  const [actions, setActions] =
    useState<(SpotlightActionGroupData | SpotlightActionData)[]>(staticActions);

  return actions;
};

export default useActionsFactory;
