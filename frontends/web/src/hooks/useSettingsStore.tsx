import { createContext, useContext, useEffect, useState } from "react";

export const gameControls = {
    up: "Up",
    down: "Down",
    left: "Left",
    right: "Right",
    start: "Start",
    select: "Select",
    b: "B",
    a: "A",
};

const initialKeyMap = {
    [gameControls.up]: "ArrowUp",
    [gameControls.down]: "ArrowDown",
    [gameControls.left]: "ArrowLeft",
    [gameControls.right]: "ArrowRight",
    [gameControls.start]: "Enter",
    [gameControls.select]: "Space",
    [gameControls.b]: "x",
    [gameControls.a]: "z",
} as Record<string, string>;

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
    readonly keyMap: Record<string, string>;
}

interface SettingsStoreProviderProps {
    readonly children: React.ReactNode;
}
