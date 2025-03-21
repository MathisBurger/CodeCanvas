import {
  SpotlightActionData,
  SpotlightActionGroupData,
} from "@mantine/spotlight";
import { useMemo } from "react";
import {
  useStaticGeneralActions,
  useStaticRoutesActions,
} from "@/hooks/spotlight/staticProvider";
import { useStage2Actions } from "@/hooks/spotlight/stage2";
import useStage3Actions from "@/hooks/spotlight/stage3";

const useActionsFactory = (
  search: string,
): (SpotlightActionGroupData | SpotlightActionData)[] => {
  const staticRoutes = useStaticRoutesActions();
  const staticGeneral = useStaticGeneralActions();
  const stage2Actions = useStage2Actions();
  const stage3Actions = useStage3Actions(search);

  const markedStage3Actions = useMemo<SpotlightActionGroupData[]>(() => {
    return stage3Actions.map((group) => {
      group.actions = group.actions.map((action) => {
        action.id = action.id + "-stage3";
        return action;
      });
      return group;
    });
  }, [stage3Actions]);

  const filterOut = (
    stage3: SpotlightActionData[],
    stage2: SpotlightActionData[],
  ) => {
    const stage3Ids = stage3.map((element) => element.id);
    return stage2.filter((element) => stage3Ids.indexOf(element.id) === -1);
  };

  /**
   * Merges spotlight stage2 and stage3 groups together
   */
  const reducedStage2 = useMemo<SpotlightActionGroupData[]>(() => {
    return stage2Actions.map((group, i) => {
      const actionsStage3 = stage3Actions[i]?.actions ?? [];
      group.actions = filterOut(actionsStage3, group.actions);
      return group;
    });
  }, [stage2Actions, stage3Actions]);

  return useMemo<(SpotlightActionGroupData | SpotlightActionData)[]>(
    () => [
      ...staticRoutes,
      ...staticGeneral,
      ...reducedStage2,
      ...markedStage3Actions,
    ],
    [reducedStage2, markedStage3Actions, staticGeneral, staticRoutes],
  );
};

export default useActionsFactory;
