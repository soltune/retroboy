import { useEffect } from "react";

import { useSettingsStore } from "./useSettingsStore";

import { pressKey, releaseKey } from "../core/retroboyCore";

export const asKeyMapping = (eventKey: string): string =>
    eventKey === " " ? "Space" : eventKey;

const invert = (keyMap: Record<string, string>): Record<string, string> => {
    return Object.entries(keyMap).reduce(
        (acc, [key, value]) => ({
            ...acc,
            [value]: key,
        }),
        {} as Record<string, string>,
    );
};

export const useKeyListeners = (playing: boolean): void => {
    const { settings } = useSettingsStore();

    const keyMap = settings.keyMap;

    const invertedKeyMap = invert(keyMap);
    const keys = Object.keys(invertedKeyMap);

    const handleKeyDown = (event: KeyboardEvent): void => {
        const key = asKeyMapping(event.key);
        if (keys.includes(key)) {
            event.preventDefault();
            pressKey(invertedKeyMap[key]);
        }
    };

    const handleKeyUp = (event: KeyboardEvent): void => {
        const key = asKeyMapping(event.key);
        if (keys.includes(key)) {
            event.preventDefault();
            releaseKey(invertedKeyMap[key]);
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
