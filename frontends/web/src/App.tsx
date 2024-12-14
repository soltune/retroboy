import FileUploadIcon from "@mui/icons-material/FileUpload";
import FullscreenIcon from "@mui/icons-material/Fullscreen";
import HelpIcon from "@mui/icons-material/Help";
import PauseIcon from "@mui/icons-material/Pause";
import PhotoCameraIcon from "@mui/icons-material/PhotoCamera";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import RefreshIcon from "@mui/icons-material/Refresh";
import {
    Button,
    CssBaseline,
    IconButton,
    ThemeProvider,
    Typography,
    createTheme,
    styled,
    ToggleButton,
    ToggleButtonGroup,
} from "@mui/material";
import { useRef, useState } from "react";

import {
    BufferFileUpload,
    FileBufferObject,
} from "./components/bufferFileUpload";
import { CssGrid, GapSize, Orientation, Position } from "./components/cssGrid";
import GameScreen from "./components/gameScreen";
import HelpModal from "./components/helpModal";
import { initializeEmulator, resetEmulator } from "./core/retroboyCore";
import useAudioSync from "./hooks/useAudioSync";
import useKeyListeners from "./hooks/useKeyListeners";
import useWasmInitializer from "./hooks/useWasmInitializer";

const AppGrid = styled(CssGrid)`
    height: 100%;
`;

const InterfaceGrid = styled(CssGrid)`
    padding: 32px;
    width: 530px;
`;

const GameSelectionGrid = styled(CssGrid)`
    height: 50px;
    width: 100%;
`;

const Header = styled("div")`
    width: 100%;
`;

const darkTheme = createTheme({
    palette: {
        mode: "dark",
    },
});

const App = (): JSX.Element => {
    const wasmInitialized = useWasmInitializer();

    const [romBuffer, setRomBuffer] = useState(null as FileBufferObject | null);

    const [playing, setPlaying] = useState(false);
    const [paused, setPaused] = useState(false);
    const [mode, setMode] = useState("DMG");

    useKeyListeners(playing);

    const [showHelpText, setShowHelpText] = useState(false);

    const canvasRef = useRef<HTMLCanvasElement | null>(null);

    const resetGame = (): void => {
        setPlaying(false);
        setPaused(false);
        resetEmulator();
        setRomBuffer(null);
    };

    const [audioContextRef, startReset] = useAudioSync(playing, resetGame);

    const playGame = (): void => {
        if (romBuffer) {
            let metadata = initializeEmulator(romBuffer.data, mode);
            console.log(`Game Title: ${metadata.title}`);
        }

        if (!audioContextRef.current) {
            audioContextRef.current = new AudioContext();
        }

        setPlaying(true);
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
        setMode(newMode);
    };

    return (
        <ThemeProvider theme={darkTheme}>
            <CssBaseline />
            <AppGrid justifyContent={Position.center}>
                <InterfaceGrid
                    orientation={Orientation.vertical}
                    gap={GapSize.extraLarge}
                    justifyItems={Position.start}
                >
                    {wasmInitialized ? (
                        <>
                            <Header>
                                <CssGrid
                                    orientation={Orientation.horizontal}
                                    alignItems={Position.center}
                                    template="1fr auto auto"
                                >
                                    <Typography variant="h3">
                                        Retro Boy
                                    </Typography>
                                    <IconButton
                                        onClick={downloadScreenshot}
                                        aria-label="screenshot"
                                    >
                                        <PhotoCameraIcon />
                                    </IconButton>
                                    <IconButton
                                        aria-label="help"
                                        onClick={() => setShowHelpText(true)}
                                    >
                                        <HelpIcon />
                                    </IconButton>
                                </CssGrid>
                                <Typography variant="h6">
                                    A simple Game Boy emulator for the web.
                                </Typography>
                            </Header>
                            <GameScreen
                                wasmInitialized={wasmInitialized}
                                playing={playing}
                                paused={paused}
                                ref={canvasRef}
                            />
                            <GameSelectionGrid
                                orientation={Orientation.horizontal}
                                alignItems={Position.center}
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
                                    <ToggleButton value="DMG">
                                        Monochrome
                                    </ToggleButton>
                                    <ToggleButton value="CGB">
                                        Color
                                    </ToggleButton>
                                </ToggleButtonGroup>
                            </GameSelectionGrid>
                            <CssGrid
                                orientation={Orientation.horizontal}
                                gap={GapSize.medium}
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
                            <HelpModal
                                showHelpText={showHelpText}
                                onClose={() => setShowHelpText(false)}
                            />
                        </>
                    ) : (
                        <div>Loading...</div>
                    )}
                </InterfaceGrid>
            </AppGrid>
        </ThemeProvider>
    );
};

export default App;
