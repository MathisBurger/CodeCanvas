import {SpotlightActionData, SpotlightActionGroupData,} from "@mantine/spotlight";
import {useMemo} from "react";
import {useStaticGeneralActions, useStaticRoutesActions,} from "@/hooks/spotlight/staticProvider";
import {useStage2Actions} from "@/hooks/spotlight/stage2";

const useActionsFactory = (): (
  | SpotlightActionGroupData
  | SpotlightActionData
)[] => {
  const staticRoutes = useStaticRoutesActions();
  const staticGeneral = useStaticGeneralActions();
  const stage2Actions = useStage2Actions();

  return useMemo<
      (SpotlightActionGroupData | SpotlightActionData)[]
  >(() => [...staticRoutes, ...staticGeneral, ...stage2Actions], [stage2Actions, staticGeneral, staticRoutes]);
};

export default useActionsFactory;
