import TriangleIcon from "@mui/icons-material/ChangeHistory";
import CircleIcon from "@mui/icons-material/FiberManualRecord";
import { styled } from "@mui/material";
import { useEffect, useRef } from "react";

import { CssGrid, GapSize, Orientation, Position } from "./cssGrid";

import { pressKey, releaseKey } from "../core/retroboyCore";
import { useIsTablet } from "../hooks/useResponsiveBreakpoint";
import { gameControls } from "../hooks/useSettingsStore";

const GamePadWrapperGrid = styled(CssGrid)`
    background-color: white;
    padding: 4px 16px;
    width: 100%;
`;

const CircularButtonGrid = styled(CssGrid)`
    margin-top: 32px;
    border-radius: 50%;
    width: 48px;
    height: 48px;
    background: ${({ theme }) => theme.palette.background.default};
    font-size: 32px;
`;

const BGrid = styled(CircularButtonGrid)`
    margin-top: 48px;
`;

const AGrid = styled(CircularButtonGrid)`
    margin-top: 16px;
`;

const CylindricalButtonGrid = styled(CssGrid)`
    width: 64px;
    color: black;
`;

const CylindricalButtonGridArea = styled("div")`
    height: 24px;
    width: 48px;
    border-radius: 40%;
    background: ${({ theme }) => theme.palette.background.default};
`;

const DirectionalPadGrid = styled(CssGrid)`
    margin-top: 16px;
    line-height: 0;
`;

const DirectionalPadArea = styled("div")`
    background: ${({ theme }) => theme.palette.background.default};
`;

const DirectionalIcon = styled(TriangleIcon, {
    shouldForwardProp: prop => prop !== "rotation",
})<{ rotation: number }>(({ rotation, theme }) => ({
    transform: `rotate(${rotation}deg)`,
    color: theme.palette.background.paper,
}));

const DirectionalPadCenterIcon = styled(CircleIcon)`
    color: ${({ theme }) => theme.palette.background.paper};
`;

const gameControlClass = "game-control";

const DirectionalPad = ({
    onTouchStart,
    onTouchEnd,
}: DirectionalPadProps): JSX.Element => {
    return (
        <DirectionalPadGrid
            orientation={Orientation.horizontal}
            template="1fr 1fr 1fr"
        >
            <div />
            <DirectionalPadArea
                onTouchStart={() => onTouchStart(gameControls.up)}
                onTouchEnd={() => onTouchEnd(gameControls.up)}
                className={gameControlClass}
            >
                <DirectionalIcon rotation={0} />
            </DirectionalPadArea>
            <div />
            <DirectionalPadArea
                onTouchStart={() => onTouchStart(gameControls.left)}
                onTouchEnd={() => onTouchEnd(gameControls.left)}
                className={gameControlClass}
            >
                <DirectionalIcon rotation={270} />
            </DirectionalPadArea>
            <DirectionalPadArea className={gameControlClass}>
                <DirectionalPadCenterIcon />
            </DirectionalPadArea>
            <DirectionalPadArea
                onTouchStart={() => onTouchStart(gameControls.right)}
                onTouchEnd={() => onTouchEnd(gameControls.right)}
                className={gameControlClass}
            >
                <DirectionalIcon rotation={90} />
            </DirectionalPadArea>
            <div />
            <DirectionalPadArea
                onTouchStart={event => onTouchStart(gameControls.down)}
                onTouchEnd={() => onTouchEnd(gameControls.down)}
                className={gameControlClass}
            >
                <DirectionalIcon rotation={180} />
            </DirectionalPadArea>
            <div />
        </DirectionalPadGrid>
    );
};

const GamePad = ({ playing }: GamePadProps): JSX.Element => {
    const isTablet = useIsTablet();

    const wrapperRef = useRef(null as HTMLDivElement | null);

    const preventDefault = (event: TouchEvent): void => {
        const node = event.target as HTMLElement;
        const isGameControl = node && !!node.closest(`.${gameControlClass}`);
        if (isGameControl) {
            event.preventDefault();
        }
    };

    const handleTouchStart = (gameControl: string): void => {
        if (playing) {
            pressKey(gameControl);
        }
    };

    const handleTouchEnd = (gameControl: string): void => {
        if (playing) {
            releaseKey(gameControl);
        }
    };

    useEffect(() => {
        const wrapper = wrapperRef.current;

        if (wrapper) {
            wrapper.addEventListener("touchstart", preventDefault, {
                passive: false,
            });
        }
        return () => {
            if (wrapper) {
                wrapper.removeEventListener("touchstart", preventDefault);
            }
        };
    }, [wrapperRef]);

    return (
        <GamePadWrapperGrid
            orientation={Orientation.vertical}
            gap={GapSize.medium}
            ref={wrapperRef}
        >
            <CssGrid
                orientation={Orientation.horizontal}
                gap={GapSize.medium}
                template="auto 1fr 1fr auto auto"
            >
                <DirectionalPad
                    onTouchStart={handleTouchStart}
                    onTouchEnd={handleTouchEnd}
                />
                <div />
                <div />
                <BGrid
                    alignItems={Position.center}
                    justifyContent={Position.center}
                    onTouchStart={() => handleTouchStart(gameControls.b)}
                    onTouchEnd={() => handleTouchEnd(gameControls.b)}
                    className={gameControlClass}
                >
                    B
                </BGrid>
                <AGrid
                    alignItems={Position.center}
                    justifyContent={Position.center}
                    onTouchStart={() => handleTouchStart(gameControls.a)}
                    onTouchEnd={() => handleTouchEnd(gameControls.a)}
                    className={gameControlClass}
                >
                    A
                </AGrid>
            </CssGrid>
            <CssGrid
                orientation={Orientation.horizontal}
                gap={GapSize.medium}
                template={isTablet ? "1fr auto 4fr auto 1fr" : undefined}
                justifyContent={!isTablet ? Position.center : undefined}
            >
                <div />
                <CylindricalButtonGrid
                    orientation={Orientation.vertical}
                    alignItems={Position.center}
                    justifyItems={Position.center}
                >
                    <CylindricalButtonGridArea
                        onTouchStart={() =>
                            handleTouchStart(gameControls.select)
                        }
                        onTouchEnd={() => handleTouchEnd(gameControls.select)}
                        className={gameControlClass}
                    />
                    <div>SELECT</div>
                </CylindricalButtonGrid>
                <div />
                <CylindricalButtonGrid
                    orientation={Orientation.vertical}
                    alignItems={Position.center}
                    justifyItems={Position.center}
                >
                    <CylindricalButtonGridArea
                        onTouchStart={() =>
                            handleTouchStart(gameControls.start)
                        }
                        onTouchEnd={() => handleTouchEnd(gameControls.start)}
                        className={gameControlClass}
                    />
                    <div>START</div>
                </CylindricalButtonGrid>
                <div />
            </CssGrid>
        </GamePadWrapperGrid>
    );
};

interface DirectionalPadProps {
    readonly onTouchStart: (gameControl: string) => void;
    readonly onTouchEnd: (gameControl: string) => void;
}

interface GamePadProps {
    readonly playing: boolean;
}

export default GamePad;
