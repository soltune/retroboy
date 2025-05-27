import BuildIcon from "@mui/icons-material/Build";
import FileDownloadIcon from "@mui/icons-material/FileDownload";
import FileUploadIcon from "@mui/icons-material/FileUpload";
import FullscreenIcon from "@mui/icons-material/Fullscreen";
import GamepadIcon from "@mui/icons-material/Gamepad";
import PauseIcon from "@mui/icons-material/Pause";
import PhotoCameraIcon from "@mui/icons-material/PhotoCamera";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import RefreshIcon from "@mui/icons-material/Refresh";
import VideogameAssetIcon from "@mui/icons-material/VideogameAsset";
import { Button, Typography, styled, Divider } from "@mui/material";
import { RefObject } from "react";

import { FileBufferObject } from "../components/bufferFileUpload";
import { CssGrid, GapSize, Orientation, Position } from "../components/cssGrid";
import { openFileDialog } from "../components/fileUploadButton";
import GamePad from "../components/gamePad";
import GameScreen from "../components/gameScreen";
import { HiddenInput } from "../components/inputStyles";
import { MenuButton } from "../components/menuButton";
import { GameBoyMode, ModeSwitch } from "../components/modeSwitch";
import { RomSelector, RomInfo } from "../components/romSelector";
import {
    ResponsiveBreakpoint,
    useResponsiveBreakpoint,
} from "../hooks/useResponsiveBreakpoint";
import { availableRoms } from "../romList";

const AppGrid = styled(CssGrid)`
    height: 100%;
    width: 100%;
`;

const HeaderGrid = styled(CssGrid)`
    margin-bottom: 8px;
`;

const GameSelectionGrid = styled(CssGrid)`
    max-width: 530px;
    margin: 16px;
`;

const GameScreenWrapper = styled("div")`
    margin-bottom: 32px;
`;

const Logo = (): JSX.Element => (
    <img src="/retroboy/logo.png" width="150" height="150" />
);

const StandardView = ({
    gameKey,
    onOpenControls,
    onOpenCheats,
    playing,
    paused,
    mode,
    rom,
    selectedRomInfo,
    onPlay,
    onPause,
    onResume,
    onScreenshot,
    onReset,
    onFullscreen,
    onModeChange,
    onRomSelect,
    onLoadStateChange,
    onSaveState,
    canvasRef,
    loadStateRef,
}: StandardViewProps): JSX.Element => {
    const breakpoint = useResponsiveBreakpoint();

    const isMobile = breakpoint === ResponsiveBreakpoint.xs;
    const isTablet = breakpoint === ResponsiveBreakpoint.sm;

    return (
        <AppGrid
            justifyContent={isTablet || isMobile ? undefined : Position.center}
            alignItems={isTablet || isMobile ? Position.end : Position.center}
        >
            <HiddenInput
                type="file"
                accept=".rbs"
                ref={loadStateRef}
                onChange={onLoadStateChange}
            />
            <CssGrid
                gap={isTablet || isMobile ? GapSize.large : GapSize.giant}
                alignItems={Position.center}
                justifyItems={Position.center}
                orientation={
                    isTablet || isMobile
                        ? Orientation.vertical
                        : Orientation.horizontal
                }
            >
                <GameSelectionGrid
                    alignItems={Position.end}
                    gap={GapSize.extraLarge}
                >
                    <div>
                        <HeaderGrid
                            orientation={
                                isMobile
                                    ? Orientation.vertical
                                    : Orientation.horizontal
                            }
                            gap={GapSize.large}
                            template={isMobile ? undefined : "1fr auto auto"}
                            justifyContent={
                                isMobile ? Position.stretch : undefined
                            }
                            alignItems={Position.center}
                        >
                            <Logo />
                            {!isMobile && !isTablet && (
                                <Button
                                    variant="contained"
                                    color="secondary"
                                    startIcon={<GamepadIcon />}
                                    onClick={onOpenControls}
                                >
                                    Controls
                                </Button>
                            )}
                            <MenuButton
                                variant="contained"
                                color="secondary"
                                disabled={!gameKey}
                                startIcon={<BuildIcon />}
                                withMobileMenu={true}
                                mobileMenuTitle="Game Tools"
                                menuItems={[
                                    {
                                        display: "Cheats",
                                        icon: (
                                            <VideogameAssetIcon fontSize="small" />
                                        ),
                                        action: onOpenCheats,
                                        key: "cheats",
                                    },
                                    {
                                        display: "Screenshot",
                                        icon: (
                                            <PhotoCameraIcon fontSize="small" />
                                        ),
                                        action: onScreenshot,
                                        key: "screenshot",
                                    },
                                    {
                                        display: "Load State",
                                        icon: (
                                            <FileUploadIcon fontSize="small" />
                                        ),
                                        action: () => {
                                            openFileDialog(loadStateRef);
                                        },
                                        key: "load-state",
                                    },
                                    {
                                        display: "Save State",
                                        icon: (
                                            <FileDownloadIcon fontSize="small" />
                                        ),
                                        action: onSaveState,
                                        key: "save-state",
                                    },
                                ]}
                            >
                                Game Tools
                            </MenuButton>
                        </HeaderGrid>
                        <Divider />
                    </div>
                    <Typography>
                        Retro Boy is a Game Boy emulator that can be played on
                        the web. To use, simply select a ROM from the dropdown
                        menu below and click "Play".
                    </Typography>
                    <CssGrid
                        orientation={
                            isMobile
                                ? Orientation.vertical
                                : Orientation.horizontal
                        }
                        gap={isMobile ? GapSize.large : undefined}
                        template="1fr auto"
                    >
                        <RomSelector
                            selectedRomName={selectedRomInfo?.name || null}
                            onRomSelect={onRomSelect}
                            roms={availableRoms}
                        />
                        <ModeSwitch
                            disabled={playing || paused}
                            mode={mode}
                            onModeChange={onModeChange}
                        />
                    </CssGrid>
                    <CssGrid
                        orientation={
                            isMobile
                                ? Orientation.vertical
                                : Orientation.horizontal
                        }
                        gap={isMobile ? GapSize.large : GapSize.medium}
                        justifyContent={
                            isMobile ? Position.stretch : Position.start
                        }
                    >
                        {!playing || paused ? (
                            <Button
                                variant="contained"
                                disabled={!rom}
                                onClick={paused ? onResume : onPlay}
                                startIcon={<PlayArrowIcon />}
                            >
                                {paused ? "Resume" : "Play"}
                            </Button>
                        ) : (
                            <Button
                                variant="contained"
                                onClick={onPause}
                                startIcon={<PauseIcon />}
                            >
                                Pause
                            </Button>
                        )}
                        <Button
                            variant="contained"
                            onClick={onReset}
                            disabled={!playing && !paused}
                            startIcon={<RefreshIcon />}
                        >
                            Reset
                        </Button>
                        <Button
                            variant="contained"
                            onClick={onFullscreen}
                            disabled={!playing}
                            startIcon={<FullscreenIcon />}
                        >
                            Fullscreen
                        </Button>
                    </CssGrid>
                </GameSelectionGrid>
                <GameScreenWrapper>
                    <GameScreen
                        playing={playing}
                        paused={paused}
                        ref={canvasRef}
                        fullscreen={false}
                    />
                </GameScreenWrapper>
            </CssGrid>
            {(isTablet || isMobile) && <GamePad playing={playing} />}
        </AppGrid>
    );
};

interface StandardViewProps {
    readonly gameKey: string | null;
    readonly playing: boolean;
    readonly paused: boolean;
    readonly mode: GameBoyMode;
    readonly rom: FileBufferObject | null;
    readonly selectedRomInfo: RomInfo | null;
    readonly onOpenControls: () => void;
    readonly onOpenCheats: () => void;
    readonly onPlay: () => void;
    readonly onPause: () => void;
    readonly onResume: () => void;
    readonly onScreenshot: () => void;
    readonly onReset: () => void;
    readonly onFullscreen: () => void;
    readonly onModeChange: (mode: GameBoyMode) => void;
    readonly onRomSelect: (romInfo: RomInfo | null) => void;
    readonly onLoadStateChange: (
        event: React.ChangeEvent<HTMLInputElement>,
    ) => Promise<void>;
    readonly onSaveState: () => void;
    readonly canvasRef: RefObject<HTMLCanvasElement>;
    readonly loadStateRef: RefObject<HTMLInputElement>;
}

export default StandardView;
