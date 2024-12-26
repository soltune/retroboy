import { useEffect } from "react";

import { Key, pressKey, releaseKey } from "../core/retroboyCore";

const keyMap = {
    ArrowDown: Key.Down,
    ArrowUp: Key.Up,
    ArrowLeft: Key.Left,
    ArrowRight: Key.Right,
    Enter: Key.Start,
    Space: Key.Select,
    KeyZ: Key.A,
    KeyX: Key.B,
} as Record<string, Key>;

const keyCodes = Object.keys(keyMap);

const useKeyListeners = (playing: boolean): void => {
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
