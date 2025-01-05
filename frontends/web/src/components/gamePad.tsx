import TriangleIcon from "@mui/icons-material/ChangeHistory";
import CircleIcon from "@mui/icons-material/FiberManualRecord";
import { styled } from "@mui/material";

import { CssGrid, GapSize, Orientation, Position } from "./cssGrid";

import { useIsTablet } from "../hooks/useResponsiveBreakpoint";
import { gameControls } from "../hooks/useSettingsStore";

const GamePadWrapperGrid = styled(CssGrid)`
    background-color: white;
    padding: 16px;
    width: 100%;
    user-select: none;
    -webkit-touch-callout: none;
`;

const CircularButtonGrid = styled(CssGrid)`
    margin-top: 32px;
    border-radius: 50%;
    width: 48px;
    height: 48px;
    background: ${({ theme }) => theme.palette.background.default};
    touch-action: none;
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
    touch-action: none;
`;

const DirectionalPadGrid = styled(CssGrid)`
    margin-top: 16px;
    line-height: 0;
`;

const DirectionalPadArea = styled("div")`
    background: ${({ theme }) => theme.palette.background.default};
    touch-action: none;
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
            >
                <DirectionalIcon rotation={0} />
            </DirectionalPadArea>
            <div />
            <DirectionalPadArea
                onTouchStart={() => onTouchStart(gameControls.left)}
                onTouchEnd={() => onTouchEnd(gameControls.left)}
            >
                <DirectionalIcon rotation={270} />
            </DirectionalPadArea>
            <DirectionalPadArea>
                <DirectionalPadCenterIcon />
            </DirectionalPadArea>
            <DirectionalPadArea
                onTouchStart={() => onTouchStart(gameControls.right)}
                onTouchEnd={() => onTouchEnd(gameControls.right)}
            >
                <DirectionalIcon rotation={90} />
            </DirectionalPadArea>
            <div />
            <DirectionalPadArea
                onTouchStart={event => onTouchStart(gameControls.down)}
                onTouchEnd={() => onTouchEnd(gameControls.down)}
            >
                <DirectionalIcon rotation={180} />
            </DirectionalPadArea>
            <div />
        </DirectionalPadGrid>
    );
};

const GamePad = ({ onTouchStart, onTouchEnd }: GamePadProps): JSX.Element => {
    const isTablet = useIsTablet();
    return (
        <GamePadWrapperGrid
            orientation={Orientation.vertical}
            gap={GapSize.medium}
        >
            <CssGrid
                orientation={Orientation.horizontal}
                gap={GapSize.medium}
                template="auto 1fr 1fr auto auto"
            >
                <DirectionalPad
                    onTouchStart={onTouchStart}
                    onTouchEnd={onTouchEnd}
                />
                <div />
                <div />
                <BGrid
                    alignItems={Position.center}
                    justifyContent={Position.center}
                    onTouchStart={() => onTouchStart(gameControls.b)}
                    onTouchEnd={() => onTouchEnd(gameControls.b)}
                >
                    B
                </BGrid>
                <AGrid
                    alignItems={Position.center}
                    justifyContent={Position.center}
                    onTouchStart={() => onTouchStart(gameControls.a)}
                    onTouchEnd={() => onTouchEnd(gameControls.a)}
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
                        onTouchStart={() => onTouchStart(gameControls.select)}
                        onTouchEnd={() => onTouchEnd(gameControls.select)}
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
                        onTouchStart={() => onTouchStart(gameControls.start)}
                        onTouchEnd={() => onTouchEnd(gameControls.start)}
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
    readonly onTouchStart: (gameControl: string) => void;
    readonly onTouchEnd: (gameControl: string) => void;
}

export default GamePad;
