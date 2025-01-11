import { ToggleButton, ToggleButtonGroup, styled } from "@mui/material";

import { useIsMobile } from "../hooks/useResponsiveBreakpoint";

const StretchableToggleButton = styled(ToggleButton, {
    shouldForwardProp: prop => prop !== "stretch",
})<{ stretch: boolean }>(({ stretch }) => ({
    width: stretch ? "50%" : undefined,
}));

export const gameBoyModes = {
    dmg: "DMG",
    cgb: "CGB",
} as Record<string, GameBoyMode>;

export type GameBoyMode = "DMG" | "CGB";

export const ModeSwitch = ({
    disabled,
    mode,
    onModeChange,
}: ModeSwitchProps): JSX.Element => {
    const isMobile = useIsMobile();

    const handleModeChange = (
        _: React.MouseEvent<HTMLElement>,
        newMode: GameBoyMode,
    ) => {
        if (newMode) {
            onModeChange(newMode);
        }
    };

    return (
        <ToggleButtonGroup
            color="primary"
            value={mode}
            exclusive
            onChange={handleModeChange}
            aria-label="Mode"
            size="small"
            disabled={disabled}
        >
            <StretchableToggleButton value="DMG" stretch={isMobile}>
                Monochrome
            </StretchableToggleButton>
            <StretchableToggleButton value="CGB" stretch={isMobile}>
                Color
            </StretchableToggleButton>
        </ToggleButtonGroup>
    );
};

interface ModeSwitchProps {
    readonly disabled: boolean;
    readonly mode: GameBoyMode;
    readonly onModeChange: (newMode: GameBoyMode) => void;
}
