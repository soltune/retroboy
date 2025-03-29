import { useEffect } from "react";

import { useSettingsStore } from "./useSettingsStore";

import { pressKey, releaseKey } from "../core/retroboyCore";
import { KeyMapping } from "../modals/controlsModal";

const buildGameBoyControls = (
    keyMap: Record<string, string | KeyMapping>,
): Record<string, string> => {
    return Object.entries(keyMap).reduce(
        (controls, [gameBoyKey, keyMapping]) => {
            const usingLegacyApproach = typeof keyMapping === "string";
            const mappedKey = usingLegacyApproach
                ? keyMapping
                : keyMapping.code;
            return {
                ...controls,
                [mappedKey]: gameBoyKey,
            };
        },
        {} as Record<string, string>,
    );
};

export const useKeyListeners = (
    playing: boolean,
    usingModal: boolean,
): void => {
    const { settings } = useSettingsStore();

    const keyMap = settings.keyMap;

    const gameBoyControls = buildGameBoyControls(keyMap);

    const handleKeyDown = (event: KeyboardEvent): void => {
        const control =
            gameBoyControls[event.code] || gameBoyControls[event.key];
        if (control && !usingModal) {
            event.preventDefault();
            pressKey(control);
        }
    };

    const handleKeyUp = (event: KeyboardEvent): void => {
        const control =
            gameBoyControls[event.code] || gameBoyControls[event.key];
        if (control && !usingModal) {
            event.preventDefault();
            releaseKey(control);
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
    }, [playing, usingModal]);
};
