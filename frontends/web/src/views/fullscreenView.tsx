import FullscreenExitIcon from "@mui/icons-material/FullscreenExit";
import { Button, styled } from "@mui/material";
import { RefObject } from "react";

import { CssGrid, GapSize, Orientation, Position } from "../components/cssGrid";
import GamePad from "../components/gamePad";
import GameScreen from "../components/gameScreen";
import {
    ResponsiveBreakpoint,
    useResponsiveBreakpoint,
} from "../hooks/useResponsiveBreakpoint";

const FullscreenWrapperGrid = styled(CssGrid)`
    height: 100%;
    width: 100%;
    background: black;
`;

const ExitFullscreenButton = styled(Button)`
    background: black;
    color: white;
`;

const WhiteFullscreenExitIcon = styled(FullscreenExitIcon)`
    color: white;
`;

const FullscreenView = ({
    playing,
    paused,
    onExitFullscreen,
    canvasRef,
}: FullscreenViewProps): JSX.Element => {
    const breakpoint = useResponsiveBreakpoint();

    const isMobile = breakpoint === ResponsiveBreakpoint.xs;
    const isTablet = breakpoint === ResponsiveBreakpoint.sm;

    return (
        <FullscreenWrapperGrid
            orientation={Orientation.vertical}
            gap={GapSize.small}
            justifyContent={Position.center}
            alignItems={Position.start}
            template={isMobile || isTablet ? "auto 1fr" : "auto 1fr auto"}
        >
            <ExitFullscreenButton
                onClick={onExitFullscreen}
                startIcon={<WhiteFullscreenExitIcon />}
            >
                Exit Fullscreen
            </ExitFullscreenButton>
            <GameScreen
                playing={playing}
                paused={paused}
                ref={canvasRef}
                fullscreen
            />
            {(isTablet || isMobile) && <GamePad playing={playing} />}
        </FullscreenWrapperGrid>
    );
};

interface FullscreenViewProps {
    readonly playing: boolean;
    readonly paused: boolean;
    readonly onExitFullscreen: () => void;
    readonly canvasRef: RefObject<HTMLCanvasElement>;
}

export default FullscreenView;
