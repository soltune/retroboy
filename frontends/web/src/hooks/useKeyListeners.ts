import { useEffect } from "react";

import { useSettingsStore } from "./useSettingsStore";

import { pressKey, releaseKey } from "../core/retroboyCore";

const useKeyListeners = (playing: boolean): void => {
    const { settings } = useSettingsStore();

    const keyMap = settings.keyMap;
    const keyCodes = Object.keys(keyMap);

    const handleKeyDown = (event: KeyboardEvent): void => {
        const keyCode = event.code;
        if (keyCodes.includes(keyCode)) {
            event.preventDefault();
            pressKey(keyMap[keyCode]);
        }
    };

    const handleKeyUp = (event: KeyboardEvent): void => {
        const keyCode = event.code;
        if (keyCodes.includes(keyCode)) {
            event.preventDefault();
            releaseKey(keyMap[keyCode]);
        }
    };

    useEffect(() => {
        if (playing) {
            window.addEventListener("keydown", handleKeyDown);
            window.addEventListener("keyup", handleKeyUp);
        }

        return () => {
            if (playing) {
                window.removeEventListener("keydown", handleKeyDown);
                window.removeEventListener("keyup", handleKeyUp);
            }
        };
    }, [playing]);
};

export default useKeyListeners;
