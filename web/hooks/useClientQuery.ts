'use client';

import {useCallback, useEffect, useState} from "react";

function useClientQuery<T>(query: () => Promise<T>, deps?: any[]): [T|null, () => void] {
    const [state, setState] = useState<T|null>(null);

    useEffect(() => {
        query().then((result) => setState(result as T));
    }, deps ?? []);

    const refetch = useCallback(() => {
        query().then((result) => setState(result as T));
    }, deps ?? []);

    return [state, refetch];
}

export default useClientQuery;
