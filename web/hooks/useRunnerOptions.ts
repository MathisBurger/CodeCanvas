import useApiServiceClient from "@/hooks/useApiServiceClient";
import {useMemo} from "react";
import useClientQuery from "@/hooks/useClientQuery";

/**
 * Gets all runner options
 *
 * @param groupId The ID of the group
 */
const useRunnerOptions = (groupId: number) => {

    const api = useApiServiceClient();
    const [group] = useClientQuery(() => api.getGroup(groupId), [groupId]);

    const verified = useMemo<boolean>(() => group?.verified ?? false, [group]);

    const cpuOptions = verified ? [".5", "1"] :[".5"];
    const memoryOptions = verified ? ["50m", "100m", "200m", "300m", "500m"] : ["50m", "100m", "200m"];
    const timeoutOptions = verified ? ["20s", "60s", "120s", "180s", "240s", "300s"] : ["20s", "60s"];

    return {cpuOptions, memoryOptions, timeoutOptions};
};

export default useRunnerOptions;
