import { createContext, useContext, useEffect, useState } from "react";

import { Cheat } from "../modals/cheatsModal";
import { initialKeyMap, KeyMapping } from "../modals/controlsModal";

const initialSettings = {
    keyMap: initialKeyMap,
} as EmulatorSettings;

const initialSettingsStoreState = {
    settings: initialSettings,
    storeSettings: () => {},
} as SettingsStoreState;

const SettingsStoreContext = createContext(initialSettingsStoreState);

export const SettingsStoreProvider = ({
    children,
}: SettingsStoreProviderProps): JSX.Element => {
    const [settingsStoreState, setSettingsStoreState] = useState({
        settings: initialSettings,
        storeSettings,
    } as SettingsStoreState);

    useEffect(() => {
        const storedSettings = localStorage.getItem("settings");
        if (storedSettings) {
            setSettingsStoreState({
                settings: JSON.parse(storedSettings),
                storeSettings,
            });
        }
    }, []);

    function storeSettings(newSettings: EmulatorSettings): void {
        setSettingsStoreState({ settings: newSettings, storeSettings });
        localStorage.setItem("settings", JSON.stringify(newSettings));
    }

    return (
        <SettingsStoreContext.Provider value={settingsStoreState}>
            {children}
        </SettingsStoreContext.Provider>
    );
};

export const useSettingsStore = (): SettingsStoreState =>
    useContext(SettingsStoreContext);

interface SettingsStoreState {
    readonly settings: EmulatorSettings;
    readonly storeSettings: (newSettings: EmulatorSettings) => void;
}

interface EmulatorSettings {
    readonly keyMap: Record<string, string | KeyMapping>;
    readonly cheats?: Record<string, Record<string, Cheat>>;
}

interface SettingsStoreProviderProps {
    readonly children: React.ReactNode;
}
