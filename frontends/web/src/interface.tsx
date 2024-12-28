import FileUploadIcon from "@mui/icons-material/FileUpload";
import FullscreenIcon from "@mui/icons-material/Fullscreen";
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
import { useRef, useState } from "react";

import {
    BufferFileUpload,
    FileBufferObject,
} from "./components/bufferFileUpload";
import { CssGrid, GapSize, Orientation, Position } from "./components/cssGrid";
import GameScreen from "./components/gameScreen";
import Modal from "./components/modal";
import {
    initializeEmulator,
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
import useWasmInitializer from "./hooks/useWasmInitializer";
import SettingsModal from "./settingsModal";

const AppGrid = styled(CssGrid)`
    height: 100%;
`;

const HeaderGrid = styled(CssGrid)`
    margin-bottom: 8px;
`;

const GameSelectionGrid = styled(CssGrid)`
    max-width: 530px;
    margin: 16px;
`;

const GameScreenGrid = styled(CssGrid, {
    shouldForwardProp: prop => prop !== "isMobile",
})<{ isMobile?: boolean }>(({ isMobile }) => ({
    marginBottom: "32px",
    justifySelf: isMobile ? "stretch" : undefined,
    margin: "16px",
}));

const CenteredGameScreen = styled(GameScreen)`
    justify-self: center;
`;

const StretchableToggleButton = styled(ToggleButton, {
    shouldForwardProp: prop => prop !== "stretch",
})<{ stretch: boolean }>(({ stretch }) => ({
    width: stretch ? "50%" : undefined,
}));

const Logo = (): JSX.Element => (
    <img src="/retroboy/logo.png" width="150" height="150" />
);

const errorModalKey = "error-modal";
const settingsModalKey = "settings-modal";

const Interface = (): JSX.Element => {
    const wasmInitialized = useWasmInitializer();
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

    const playGame = () => {
        if (romBuffer) {
            const { error, metadata } = initializeEmulator(
                romBuffer.data,
                mode,
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

                if (!audioContextRef.current) {
                    audioContextRef.current = new AudioContext();
                }

                setPlaying(true);
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
    };

    const setFullscreen = (): void => {
        if (canvasRef.current) {
            canvasRef.current.requestFullscreen();
        }
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

    return (
        <AppGrid justifyContent={Position.center} alignItems={Position.center}>
            {wasmInitialized ? (
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
                    <GameScreenGrid
                        gap={GapSize.large}
                        orientation={Orientation.vertical}
                        justifyItems={isMobile ? undefined : Position.start}
                        isMobile={isMobile}
                    >
                        <Button
                            startIcon={<PhotoCameraIcon />}
                            onClick={downloadScreenshot}
                            disabled={!playing && !paused}
                            color="secondary"
                            variant="contained"
                        >
                            Screenshot
                        </Button>
                        <CenteredGameScreen
                            wasmInitialized={wasmInitialized}
                            playing={playing}
                            paused={paused}
                            ref={canvasRef}
                        />
                    </GameScreenGrid>
                </CssGrid>
            ) : (
                <div>Loading...</div>
            )}
        </AppGrid>
    );
};

export default Interface;
