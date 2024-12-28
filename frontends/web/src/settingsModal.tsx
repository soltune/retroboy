import styled from "@emotion/styled";
import { alpha, Typography } from "@mui/material";
import { useEffect, useState } from "react";

import { CssGrid, GapSize, Orientation, Position } from "./components/cssGrid";
import Modal from "./components/modal";
import { asKeyMapping } from "./hooks/useKeyListeners";
import { useIsMobile } from "./hooks/useResponsiveBreakpoint";
import { useSettingsStore } from "./hooks/useSettingsStore";

const GameControlsGrid = styled(CssGrid, {
    shouldForwardProp: prop => prop !== "isMobile",
})<{ isMobile: boolean }>(({ isMobile }) => ({
    maxHeight: isMobile ? "50%" : undefined,
    overflowY: isMobile ? "scroll" : undefined,
    marginBottom: "16px",
}));

const KeyMappingGrid = styled(CssGrid, {
    shouldForwardProp: prop => prop !== "selected",
})<{ selected: boolean }>(({ selected, theme }) => ({
    background: selected
        ? theme.palette.primary.main
        : theme.palette.background.paper,
    color: selected
        ? theme.palette.primary.contrastText
        : theme.palette.text.primary,
    border: `1px solid ${theme.palette.text.secondary}`,
    padding: "4px 8px",
    "&:hover": {
        background: selected
            ? undefined
            : alpha(theme.palette.primary.dark, 0.2),
        cursor: "pointer",
    },
}));

const Key = styled.div`
    background-color: #eee;
    color: #000;
    border: 1px solid #ccc;
    border-radius: 5px;
    padding: 5px 10px;
    font-family: monospace;
    font-size: 12px;
    text-align: center;
`;

const SettingsModal = ({ onClose }: SettingsModalProps): JSX.Element => {
    const isMobile = useIsMobile();
    const { settings, storeSettings } = useSettingsStore();

    const { keyMap } = settings;

    const [selectedControl, setSelectedControl] = useState<string | null>(null);

    useEffect(() => {
        if (selectedControl) {
            const listener = (event: KeyboardEvent): void => {
                event.preventDefault();

                const updatedMap = {
                    ...keyMap,
                    [selectedControl]: asKeyMapping(event.key),
                };

                storeSettings({ keyMap: updatedMap });

                setSelectedControl(null);
            };

            window.addEventListener("keydown", listener);

            return () => {
                window.removeEventListener("keydown", listener);
            };
        }
    }, [selectedControl]);

    return (
        <Modal heading="Settings" open={true} onClose={onClose}>
            <Typography variant="h6">Game Controls</Typography>
            <div>
                {selectedControl
                    ? "Press any key to setup new mapping..."
                    : "Click any key mapping below to change it."}
            </div>
            <GameControlsGrid
                orientation={Orientation.vertical}
                gap={GapSize.medium}
                isMobile={isMobile}
            >
                {Object.entries(keyMap).map(([control, keyMapping]) => (
                    <KeyMappingGrid
                        orientation={Orientation.horizontal}
                        alignItems={Position.center}
                        template="1fr auto"
                        key={control}
                        onClick={() => setSelectedControl(control)}
                        selected={control === selectedControl}
                    >
                        <Typography variant="body1">{control}</Typography>
                        <Key>{keyMapping}</Key>
                    </KeyMappingGrid>
                ))}
            </GameControlsGrid>
        </Modal>
    );
};

interface SettingsModalProps {
    readonly onClose: () => void;
}

export default SettingsModal;
