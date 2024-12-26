import { createContext, Fragment, useContext, useState } from "react";

const topLevelRendererInitialState = {
    displayTopLevelComponent: () => {},
    removeTopLevelComponent: () => {},
} as TopLevelRendererState;

export const TopLevelRendererContext = createContext(
    topLevelRendererInitialState,
);

export const TopLevelRendererProvider = ({
    children,
}: TopLevelRendererProviderProps): JSX.Element => {
    const [componentMap, setComponentMap] = useState(
        {} as Record<string, React.ReactNode>,
    );
    const [topLevelRendererState] = useState({
        displayTopLevelComponent,
        removeTopLevelComponent,
    } as TopLevelRendererState);

    function displayTopLevelComponent(key: string, node: React.ReactNode) {
        setComponentMap(prevState => ({ ...prevState, [key]: node }));
    }

    function removeTopLevelComponent(key: string) {
        setComponentMap(prevState => {
            const newComponentMap = { ...prevState };
            delete newComponentMap[key];
            return newComponentMap;
        });
    }

    return (
        <TopLevelRendererContext.Provider value={topLevelRendererState}>
            {children}
            {Object.entries(componentMap).map(([key, node]) => (
                <Fragment key={key}>{node}</Fragment>
            ))}
        </TopLevelRendererContext.Provider>
    );
};

export interface TopLevelRendererState {
    displayTopLevelComponent: (key: string, node: React.ReactNode) => void;
    removeTopLevelComponent: (key: string) => void;
}

export const useTopLevelRenderer = (): TopLevelRendererState =>
    useContext(TopLevelRendererContext);

interface TopLevelRendererProviderProps {
    children: React.ReactNode;
}
