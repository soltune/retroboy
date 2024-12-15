import { useEffect, useRef } from "react";

import {
    getCartridgeRam,
    RomMetadata,
    setCartridgeRam,
} from "../core/retroboyCore";

const base64ToUint8Array = (base64: string): Uint8Array => {
    const binaryString = atob(base64);
    const len = binaryString.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
};

export const loadCartridgeRam = (gameTitle: string): void => {
    const encodedCartridgeRam = localStorage.getItem(gameTitle);
    if (encodedCartridgeRam) {
        const cartridgeRam = base64ToUint8Array(encodedCartridgeRam);
        setCartridgeRam(cartridgeRam);
    }
};

export const storeCartridgeRam = (gameTitle: string): void => {
    const cartridgeRam = getCartridgeRam();
    const encodedCartridgeRam = btoa(
        String.fromCharCode.apply(null, Array.from(cartridgeRam)),
    );
    localStorage.setItem(gameTitle, encodedCartridgeRam);
};

const BACKUP_FREQUENCY_IN_MS = 5000;

export const useCartridgeRamSaver = (
    playing: boolean,
    romMetadata: RomMetadata | null,
): void => {
    const { hasBattery, title } = romMetadata || {};

    const intervalHandleRef = useRef<number | null>(null);

    useEffect(() => {
        if (title && hasBattery && playing) {
            intervalHandleRef.current = window.setInterval(() => {
                storeCartridgeRam(title);
            }, BACKUP_FREQUENCY_IN_MS);
        }

        return () => {
            if (title && hasBattery && playing && intervalHandleRef.current) {
                window.clearInterval(intervalHandleRef.current);
            }
        };
    }, [playing, title, hasBattery]);
};
