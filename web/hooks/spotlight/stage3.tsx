import { useEffect, useState } from "react";
import { useDebouncedValue } from "@mantine/hooks";
import useApiServiceClient from "@/hooks/useApiServiceClient";
import { Spotlight3Response } from "@/service/types/spotlight";
import { SpotlightActionGroupData } from "@mantine/spotlight";
import { useTranslation } from "react-i18next";
import { useRouter } from "next/navigation";

/**
 * Gets all stage3 actions and fetches them from the server.
 * A debounced value is used for fetching. If the user stops typing for 300ms, the request will be submitted
 *
 * @param search The search
 */
const useStage3Actions = (search: string): SpotlightActionGroupData[] => {
  const [actions, setActions] = useState<SpotlightActionGroupData[]>([]);
  const api = useApiServiceClient();
  const { t } = useTranslation("common");
  const router = useRouter();

  const [debounced] = useDebouncedValue(search, 300);

  const buildActions = (response: Spotlight3Response) => {
    const actions = [
      {
        group: t("spotlight.groups"),
        actions: response.groups.map((group) => ({
          id: `group-${group.id}`,
          label: group.title,
          description: "",
          onClick: () => router.push(`/groups/${group.id}`),
        })),
      },
      {
        group: t("spotlight.assignments"),
        actions: response.assignments.map((assignment) => ({
          id: `assignment-${assignment.id}`,
          label: assignment.title,
          description: "",
          onClick: () =>
            router.push(
              `/groups/${assignment.group_id}/assignments/${assignment.id}`,
            ),
        })),
      },
    ];

    setActions(actions);
  };

  useEffect(() => {
    if (debounced.trim() !== "") {
      api.getSpotlightResult(debounced).then(buildActions);
    }
  }, [debounced]);

  return actions;
};

export default useStage3Actions;
