import debounce from "lodash/debounce";

import { WasmRTCState } from "./core/retroboyCore";

const DEBOUNCE_DELAY = 2000;

const currentTimeMillis = () => Date.now();

const loadRTCState = (key: string): WasmRTCState | null => {
    const item = localStorage.getItem(key);
    if (item) {
        const parsed = JSON.parse(item);
        return new WasmRTCState(
            parsed.milliseconds,
            parsed.seconds,
            parsed.minutes,
            parsed.hours,
            parsed.days,
            parsed.base_timestamp,
            parsed.halted,
            parsed.day_carry,
        );
    }
    return null;
};

const saveRTCState = debounce((key: string, value: WasmRTCState) => {
    localStorage.setItem(
        key,
        JSON.stringify({
            milliseconds: value.milliseconds,
            seconds: value.seconds,
            minutes: value.minutes,
            hours: value.hours,
            days: value.days,
            base_timestamp: value.base_timestamp,
            halted: value.halted,
            day_carry: value.day_carry,
        }),
    );
}, DEBOUNCE_DELAY);

const base64ToUint8Array = (base64: string): Uint8Array => {
    const binaryString = atob(base64);
    const len = binaryString.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
};

const loadRam = (key: string): Uint8Array | null => {
    const encodedCartridgeRam = localStorage.getItem(key);
    if (encodedCartridgeRam) {
        return base64ToUint8Array(encodedCartridgeRam);
    }
    return null;
};

const saveRam = debounce((key: string, value: Uint8Array) => {
    const encodedCartridgeRam = btoa(
        String.fromCharCode.apply(null, Array.from(value)),
    );
    localStorage.setItem(key, encodedCartridgeRam);
}, DEBOUNCE_DELAY);

const registerCartridgeEffects = () => {
    (window as any).currentTimeMillis = currentTimeMillis;
    (window as any).loadRTCState = loadRTCState;
    (window as any).saveRTCState = saveRTCState;
    (window as any).loadRam = loadRam;
    (window as any).saveRam = saveRam;
};

export default registerCartridgeEffects;
