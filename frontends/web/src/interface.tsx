import FileUploadIcon from "@mui/icons-material/FileUpload";
import FullscreenIcon from "@mui/icons-material/Fullscreen";
import FullscreenExitIcon from "@mui/icons-material/FullscreenExit";
import PauseIcon from "@mui/icons-material/Pause";
import PhotoCameraIcon from "@mui/icons-material/PhotoCamera";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import RefreshIcon from "@mui/icons-material/Refresh";
import SettingsIcon from "@mui/icons-material/Settings";
import {
    Button,
    Typography,
    styled,
    ToggleButton,
    ToggleButtonGroup,
    Divider,
} from "@mui/material";
import { useEffect, useRef, useState } from "react";

import {
    BufferFileUpload,
    FileBufferObject,
} from "./components/bufferFileUpload";
import { CssGrid, GapSize, Orientation, Position } from "./components/cssGrid";
import GamePad from "./components/gamePad";
import GameScreen from "./components/gameScreen";
import Modal from "./components/modal";
import {
    EmulatorSettings,
    initializeEmulator,
    pressKey,
    releaseKey,
    resetEmulator,
    RomMetadata,
} from "./core/retroboyCore";
import useAudioSync from "./hooks/useAudioSync";
import {
    useCartridgeRamSaver,
    loadCartridgeRam,
} from "./hooks/useCartridgeRamSaver";
import { useKeyListeners } from "./hooks/useKeyListeners";
import {
    ResponsiveBreakpoint,
    useResponsiveBreakpoint,
} from "./hooks/useResponsiveBreakpoint";
import { useTopLevelRenderer } from "./hooks/useTopLevelRenderer";
import SettingsModal from "./settingsModal";

const AppGrid = styled(CssGrid, {
    shouldForwardProp: prop => prop !== "mobileFullscreenMode",
})<{ mobileFullscreenMode?: boolean }>(({ mobileFullscreenMode }) => ({
    height: "100%",
    width: "100%",
    background: mobileFullscreenMode ? "black" : undefined,
}));

const HeaderGrid = styled(CssGrid)`
    margin-bottom: 8px;
`;

const GameSelectionGrid = styled(CssGrid)`
    max-width: 530px;
    margin: 16px;
`;

const GameScreenGrid = styled(CssGrid, {
    shouldForwardProp: prop =>
        prop !== "isMobile" && prop != "mobileFullscreenMode",
})<{ isMobile?: boolean; mobileFullscreenMode?: boolean }>(
    ({ isMobile, mobileFullscreenMode }) => ({
        marginBottom: mobileFullscreenMode ? undefined : "32px",
        justifySelf: isMobile ? "stretch" : undefined,
        margin: mobileFullscreenMode ? undefined : "16px",
    }),
);

const CenteredGameScreen = styled(GameScreen)`
    justify-self: center;
`;

const StretchableToggleButton = styled(ToggleButton, {
    shouldForwardProp: prop => prop !== "stretch",
})<{ stretch: boolean }>(({ stretch }) => ({
    width: stretch ? "50%" : undefined,
}));

const ExitFullscreenButton = styled(Button)`
    background: black;
    color: white;
`;

const WhiteFullscreenExitIcon = styled(FullscreenExitIcon)`
    color: white;
`;

const Logo = (): JSX.Element => (
    <img src="/retroboy/logo.png" width="150" height="150" />
);

const errorModalKey = "error-modal";
const settingsModalKey = "settings-modal";

const Interface = (): JSX.Element => {
    const breakpoint = useResponsiveBreakpoint();
    const isMobile = breakpoint === ResponsiveBreakpoint.xs;
    const isTablet = breakpoint === ResponsiveBreakpoint.sm;

    const { displayTopLevelComponent, removeTopLevelComponent } =
        useTopLevelRenderer();

    const [romBuffer, setRomBuffer] = useState(null as FileBufferObject | null);

    const [playing, setPlaying] = useState(false);
    const [paused, setPaused] = useState(false);
    const [mode, setMode] = useState("DMG");
    const [romMetadata, setRomMetadata] = useState(null as RomMetadata | null);
    const [mobileFullscreenMode, setMobileFullscreenMode] = useState(false);

    useKeyListeners(playing);

    const canvasRef = useRef<HTMLCanvasElement | null>(null);

    const resetGame = (): void => {
        setPlaying(false);
        setPaused(false);
        resetEmulator();
        setRomBuffer(null);
    };

    const [audioContextRef, startReset] = useAudioSync(playing, resetGame);

    useCartridgeRamSaver(playing, romMetadata);

    const scrollToGamePad = ({ smooth }: { smooth: boolean }) => {
        window.scrollTo({
            top: document.body.scrollHeight,
            behavior: smooth ? "smooth" : "auto",
        });
    };

    const playGame = () => {
        if (romBuffer) {
            if (!audioContextRef.current) {
                audioContextRef.current = new AudioContext();
            }

            const settings = new EmulatorSettings(
                mode,
                audioContextRef.current.sampleRate,
            );

            const { error, metadata } = initializeEmulator(
                romBuffer.data,
                settings,
            );

            if (error) {
                displayTopLevelComponent(
                    errorModalKey,
                    <Modal
                        heading="Error"
                        open={!!error}
                        onClose={() => removeTopLevelComponent(errorModalKey)}
                    >
                        {error}
                    </Modal>,
                );

                resetGame();
            } else if (metadata) {
                if (metadata.hasBattery) {
                    loadCartridgeRam(metadata.title);
                }
                setRomMetadata(metadata);
                setPlaying(true);
                scrollToGamePad({ smooth: true });
            }
        }
    };

    const pauseGame = (): void => {
        setPaused(true);
        setPlaying(false);
    };

    const resumeGame = (): void => {
        setPaused(false);
        setPlaying(true);
        scrollToGamePad({ smooth: true });
    };

    const setFullscreen = (): void => {
        if (isMobile || isTablet) {
            setMobileFullscreenMode(true);
        } else {
            const canvas = canvasRef.current;
            if (canvas && canvas.requestFullscreen) {
                canvas.requestFullscreen();
            }
        }
    };

    const exitMobileFullscreen = (): void => {
        setMobileFullscreenMode(false);
    };

    const downloadScreenshot = (): void => {
        if (canvasRef.current) {
            const dataUrl = canvasRef.current.toDataURL("image/png");
            const link = document.createElement("a");
            link.href = dataUrl;
            link.download = "retroboy-screenshot.png";
            link.click();
        }
    };

    const handleModeChange = (
        _: React.MouseEvent<HTMLElement>,
        newMode: string,
    ) => {
        if (newMode) {
            setMode(newMode);
        }
    };

    const openSettings = (): void => {
        displayTopLevelComponent(
            settingsModalKey,
            <SettingsModal
                onClose={() => removeTopLevelComponent(settingsModalKey)}
            />,
        );
    };

    useEffect(() => {
        if (!isMobile && !isTablet && mobileFullscreenMode) {
            exitMobileFullscreen();
        }
    }, [isMobile, isTablet]);

    useEffect(() => {
        if (playing && !mobileFullscreenMode) {
            setTimeout(() => {
                window.requestAnimationFrame(() =>
                    scrollToGamePad({ smooth: false }),
                );
            });
        }
    }, [mobileFullscreenMode]);

    return (
        <AppGrid
            justifyContent={isTablet || isMobile ? undefined : Position.center}
            alignItems={isTablet || isMobile ? Position.end : Position.center}
            mobileFullscreenMode={mobileFullscreenMode}
        >
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
                {!mobileFullscreenMode && (
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
                                template={isMobile ? undefined : "1fr auto"}
                                justifyContent={
                                    isMobile ? Position.stretch : undefined
                                }
                                alignItems={Position.center}
                            >
                                <Logo />
                                <Button
                                    variant="contained"
                                    color="secondary"
                                    startIcon={<SettingsIcon />}
                                    onClick={openSettings}
                                >
                                    Settings
                                </Button>
                            </HeaderGrid>
                            <Divider />
                        </div>
                        <Typography>
                            Retro Boy is a Game Boy emulator that can be played
                            on the web. To use, simply click "Load ROM" to load
                            your game ROM. Only .gb files are supported. Then
                            click "Play".
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
                            <BufferFileUpload
                                label="Load ROM"
                                onFileSelect={setRomBuffer}
                                uploadedFile={romBuffer}
                                variant="contained"
                                accept=".gb"
                                startIcon={<FileUploadIcon />}
                            />
                            <ToggleButtonGroup
                                color="primary"
                                value={mode}
                                exclusive
                                onChange={handleModeChange}
                                aria-label="Mode"
                                size="small"
                                disabled={playing || paused}
                            >
                                <StretchableToggleButton
                                    value="DMG"
                                    stretch={isMobile}
                                >
                                    Monochrome
                                </StretchableToggleButton>
                                <StretchableToggleButton
                                    value="CGB"
                                    stretch={isMobile}
                                >
                                    Color
                                </StretchableToggleButton>
                            </ToggleButtonGroup>
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
                                    disabled={!romBuffer}
                                    onClick={paused ? resumeGame : playGame}
                                    startIcon={<PlayArrowIcon />}
                                >
                                    {paused ? "Resume" : "Play"}
                                </Button>
                            ) : (
                                <Button
                                    variant="contained"
                                    onClick={pauseGame}
                                    startIcon={<PauseIcon />}
                                >
                                    Pause
                                </Button>
                            )}
                            <Button
                                variant="contained"
                                onClick={startReset}
                                disabled={!playing && !paused}
                                startIcon={<RefreshIcon />}
                            >
                                Reset
                            </Button>
                            <Button
                                variant="contained"
                                onClick={setFullscreen}
                                disabled={!playing && !paused}
                                startIcon={<FullscreenIcon />}
                            >
                                Fullscreen
                            </Button>
                        </CssGrid>
                    </GameSelectionGrid>
                )}
                {mobileFullscreenMode && (
                    <ExitFullscreenButton
                        onClick={exitMobileFullscreen}
                        variant="contained"
                        startIcon={<WhiteFullscreenExitIcon />}
                    >
                        Exit Fullscreen
                    </ExitFullscreenButton>
                )}
                <GameScreenGrid
                    gap={GapSize.large}
                    orientation={Orientation.vertical}
                    justifyItems={isMobile ? undefined : Position.start}
                    isMobile={isMobile}
                    mobileFullscreenMode={mobileFullscreenMode}
                >
                    {!mobileFullscreenMode && (
                        <Button
                            startIcon={<PhotoCameraIcon />}
                            onClick={downloadScreenshot}
                            disabled={!playing && !paused}
                            color="secondary"
                            variant="contained"
                        >
                            Screenshot
                        </Button>
                    )}
                    <CenteredGameScreen
                        playing={playing}
                        paused={paused}
                        mobileFullscreen={mobileFullscreenMode}
                        ref={canvasRef}
                    />
                </GameScreenGrid>
            </CssGrid>
            {(isTablet || isMobile) && (
                <GamePad
                    onTouchStart={gameControl => {
                        if (playing) {
                            pressKey(gameControl);
                        }
                    }}
                    onTouchEnd={gameControl => {
                        if (playing) {
                            releaseKey(gameControl);
                        }
                    }}
                />
            )}
        </AppGrid>
    );
};

export default Interface;
