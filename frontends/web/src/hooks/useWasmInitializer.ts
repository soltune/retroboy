import { useEffect, useState } from "react";

import init from "../core/retroboyCore";

const useWasmInitializer = () => {
    const [wasmInitialized, setWasmInitialized] = useState(false);

    const initalizeWasm = (): void => {
        init().then(() => {
            setWasmInitialized(true);
        });
    };

    useEffect(() => {
        initalizeWasm();
    }, []);

    return wasmInitialized;
};

export default useWasmInitializer;
