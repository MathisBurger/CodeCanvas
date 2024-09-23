'use client';

import {useEffect, useState} from "react";

interface ClientQueryProps<T> {
    query: () => T;
}

function useClientQuery<T>(query: () => Promise<T>) {
    const [state, setState] = useState<T|null>(null);

    useEffect(() => {
        query().then((result) => setState(result as T));
    }, []);

    return state;
}

export default useClientQuery;
