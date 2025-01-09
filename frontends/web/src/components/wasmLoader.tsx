import { useEffect, useState } from "react";

import init from "../core/retroboyCore";

const WasmLoader = ({ children }: WasmLoaderProps): JSX.Element => {
    const [wasmInitialized, setWasmInitialized] = useState(false);

    const initalizeWasm = (): void => {
        init().then(() => {
            setWasmInitialized(true);
        });
    };

    useEffect(() => {
        initalizeWasm();
    }, []);

    return wasmInitialized ? <>{children}</> : <div>Loading...</div>;
};

interface WasmLoaderProps {
    readonly children: React.ReactNode;
}

export default WasmLoader;
