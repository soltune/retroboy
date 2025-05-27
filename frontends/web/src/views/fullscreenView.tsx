import FullscreenExitIcon from "@mui/icons-material/FullscreenExit";
import { Button, styled } from "@mui/material";
import { RefObject, useLayoutEffect, useRef, useState } from "react";

import { CssGrid, GapSize, Orientation, Position } from "../components/cssGrid";
import GamePad from "../components/gamePad";
import GameScreen, {
    GAMEBOY_HEIGHT,
    GAMEBOY_WIDTH,
} from "../components/gameScreen";
import {
    ResponsiveBreakpoint,
    useResponsiveBreakpoint,
} from "../hooks/useResponsiveBreakpoint";

const FullscreenWrapperGrid = styled(CssGrid)`
    height: 100%;
    width: 100%;
    background: black;
    touch-action: none;
    overscroll-behavior: none;
    overflow: hidden;
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
    const screenWrapperRef = useRef(null as HTMLDivElement | null);
    const breakpoint = useResponsiveBreakpoint();

    const isMobile = breakpoint === ResponsiveBreakpoint.xs;
    const isTablet = breakpoint === ResponsiveBreakpoint.sm;

    const [scale, setScale] = useState(undefined as number | undefined);

    useLayoutEffect(() => {
        if (screenWrapperRef.current) {
            const screenWrapper = screenWrapperRef.current;

            const screenWrapperHeight = screenWrapper.clientHeight;
            const heightScale = screenWrapperHeight / GAMEBOY_HEIGHT;

            const windowWidth = window.innerWidth;
            const widthScale = windowWidth / GAMEBOY_WIDTH;

            const smallerScale = Math.min(heightScale, widthScale);

            setScale(smallerScale);
        }
    }, []);

    return (
        <FullscreenWrapperGrid
            orientation={Orientation.vertical}
            gap={GapSize.small}
            justifyContent={Position.stretch}
            template={isMobile || isTablet ? "auto 1fr" : "auto 1fr auto"}
        >
            <ExitFullscreenButton
                onClick={onExitFullscreen}
                startIcon={<WhiteFullscreenExitIcon />}
            >
                Exit Fullscreen
            </ExitFullscreenButton>
            <CssGrid
                alignItems={Position.center}
                justifyContent={Position.center}
                ref={screenWrapperRef}
            >
                <GameScreen
                    playing={playing}
                    paused={paused}
                    ref={canvasRef}
                    scale={scale}
                    fullscreen
                />
            </CssGrid>
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
