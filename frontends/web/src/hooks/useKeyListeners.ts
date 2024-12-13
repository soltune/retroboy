import { useEffect } from "react";

import { pressKey, releaseKey } from "../core/retroboyCore";

const keys = [
    "ArrowDown",
    "ArrowUp",
    "ArrowLeft",
    "ArrowRight",
    "Enter",
    "Space",
    "KeyX",
    "KeyZ",
];

const useKeyListeners = (playing: boolean): void => {
    const handleKeyDown = (event: KeyboardEvent): void => {
        if (keys.includes(event.code)) {
            event.preventDefault();
            pressKey(event.code);
        }
    };

    const handleKeyUp = (event: KeyboardEvent): void => {
        if (keys.includes(event.code)) {
            event.preventDefault();
            releaseKey(event.code);
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
