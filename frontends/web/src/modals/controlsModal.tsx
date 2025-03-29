import styled from "@emotion/styled";
import Typography from "@mui/material/Typography";
import { useEffect, useState } from "react";

import { CssGrid, GapSize, Orientation, Position } from "../components/cssGrid";
import { ListGrid, ListItemGrid } from "../components/list";
import { Modal, ModalGridButton } from "../components/modal";
import { useIsMobile } from "../hooks/useResponsiveBreakpoint";
import { useSettingsStore } from "../hooks/useSettingsStore";

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

export const initialKeyMap = {
    [gameControls.up]: { key: "ArrowUp", code: "ArrowUp" },
    [gameControls.down]: { key: "ArrowDown", code: "ArrowDown" },
    [gameControls.left]: { key: "ArrowLeft", code: "ArrowLeft" },
    [gameControls.right]: { key: "ArrowRight", code: "ArrowRight" },
    [gameControls.start]: { key: "Enter", code: "Enter" },
    [gameControls.select]: { key: " ", code: "Space" },
    [gameControls.b]: { key: "x", code: "KeyX" },
    [gameControls.a]: { key: "z", code: "KeyZ" },
} as Record<string, string | KeyMapping>;

const buildKeyDisplay = (keyMapping: string | KeyMapping): string => {
    const usingLegacyApproach = typeof keyMapping === "string";
    const display = usingLegacyApproach ? keyMapping : keyMapping.key;
    return display === " " ? "Space" : display;
};

export const ControlsModal = ({ onClose }: ControlsModalProps): JSX.Element => {
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
                    [selectedControl]: {
                        key: event.key,
                        code: event.code,
                    },
                };

                storeSettings({ ...settings, keyMap: updatedMap });

                setSelectedControl(null);
            };

            window.addEventListener("keydown", listener);

            return () => {
                window.removeEventListener("keydown", listener);
            };
        }
    }, [selectedControl]);

    return (
        <Modal heading="Game Controls" open={true} onClose={onClose}>
            <CssGrid orientation={Orientation.vertical} gap={GapSize.large}>
                <div>
                    {selectedControl
                        ? "Press any key to setup new mapping..."
                        : "Click any key mapping below to change it."}
                </div>
                <ListGrid
                    orientation={Orientation.vertical}
                    gap={GapSize.medium}
                    isMobile={isMobile}
                >
                    {Object.entries(keyMap).map(([control, keyMapping]) => (
                        <ListItemGrid
                            orientation={Orientation.horizontal}
                            alignItems={Position.center}
                            template="1fr auto"
                            key={control}
                            onClick={() => setSelectedControl(control)}
                            selected={control === selectedControl}
                        >
                            <Typography variant="body1">{control}</Typography>
                            <Key>{buildKeyDisplay(keyMapping)}</Key>
                        </ListItemGrid>
                    ))}
                </ListGrid>
                <ModalGridButton
                    variant="contained"
                    onClick={onClose}
                    isMobile={isMobile}
                >
                    Close
                </ModalGridButton>
            </CssGrid>
        </Modal>
    );
};

export interface KeyMapping {
    key: string;
    code: string;
}

interface ControlsModalProps {
    readonly onClose: () => void;
}
