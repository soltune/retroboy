import {
    createContext,
    useCallback,
    useContext,
    useEffect,
    useState,
} from "react";

import { Key } from "../core/retroboyCore";

const initialKeyMap = {
    ArrowDown: Key.Down,
    ArrowUp: Key.Up,
    ArrowLeft: Key.Left,
    ArrowRight: Key.Right,
    Enter: Key.Start,
    Space: Key.Select,
    KeyZ: Key.A,
    KeyX: Key.B,
} as Record<string, Key>;

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
    const [settings, setSettings] = useState(initialSettings);

    useEffect(() => {
        const storedSettings = localStorage.getItem("settings");
        if (storedSettings) {
            setSettings(JSON.parse(storedSettings));
        }
    }, []);

    const storeSettings = useCallback((newSettings: EmulatorSettings): void => {
        setSettings(newSettings);
        localStorage.setItem("settings", JSON.stringify(newSettings));
    }, []);

    const [settingsStoreState] = useState({
        settings,
        storeSettings,
    } as SettingsStoreState);

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
    readonly keyMap: Record<string, Key>;
}

interface SettingsStoreProviderProps {
    readonly children: React.ReactNode;
}
