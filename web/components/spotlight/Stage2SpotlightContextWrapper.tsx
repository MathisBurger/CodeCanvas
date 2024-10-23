import React, {useEffect, useState} from "react";
import {Stage2Context, Stage2Type} from "@/hooks/spotlight/stage2";



const Stage2SpotlightContextWrapper = ({children}: React.PropsWithChildren<unknown>) => {

    const [state, setState] = useState<Stage2Type>({groups: [], assignments: [], solutions: []});


    const updateContent = (content: Stage2Type) => {
        setState(content);
        localStorage.setItem("spotlight-stage2", JSON.stringify(content));
    }

    useEffect(() => {
        const data = localStorage.getItem("spotlight-stage2");
        if (data) {
            setState(JSON.parse(data));
        }
    }, []);


    return (
        <Stage2Context.Provider value={{content: state, setContent: updateContent}}>
            {children}
        </Stage2Context.Provider>
    );
}

export default Stage2SpotlightContextWrapper;
