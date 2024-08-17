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
} from "@mui/material";
import { useCallback, useEffect, useRef, useState } from "react";

import {
    BufferFileUpload,
    FileBufferObject,
} from "./components/bufferFileUpload";
import { CssGrid, GapSize, Orientation, Position } from "./components/cssGrid";
import GameScreen from "./components/gameScreen";
import HelpModal from "./components/helpModal";
import init, {
    initializeEmulator,
    pressKey,
    releaseKey,
    resetEmulator,
    stepUntilNextAudioBuffer,
} from "./core/retroboyCore";

const AppGrid = styled(CssGrid)`
    height: 100%;
`;

const InterfaceGrid = styled(CssGrid)`
    padding: 32px;
`;

const Header = styled("div")`
    width: 100%;
`;

const darkTheme = createTheme({
    palette: {
        mode: "dark",
    },
});

const keys = [
    "ArrowDown",
    "ArrowUp",
    "ArrowLeft",
    "ArrowRight",
    "Enter",
    "Space",
    "KeyX",
    "KeyZ",
];

const defaultMode = "DMG";

const App = (): JSX.Element => {
    const [wasmInitialized, setWasmInitialized] = useState(false);

    const [romBuffer, setRomBuffer] = useState(null as FileBufferObject | null);

    const [playing, setPlaying] = useState(false);
    const [paused, setPaused] = useState(false);

    const [showHelpText, setShowHelpText] = useState(false);

    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const audioContextRef = useRef<AudioContext | null>(null);
    const scheduledResetRef = useRef<boolean>(false);

    const initalizeWasm = (): void => {
        init().then(() => {
            setWasmInitialized(true);
        });
    };

    const playGame = (): void => {
        if (romBuffer) {
            initializeEmulator(romBuffer.data, defaultMode);
        }

        if (!audioContextRef.current) {
            audioContextRef.current = new AudioContext();
        }

        setPlaying(true);
    };

    const resetGame = (): void => {
        setPlaying(false);
        setPaused(false);
        resetEmulator();
        setRomBuffer(null);
        scheduledResetRef.current = false;
    };

    const startReset = (): void => {
        if (playing) {
            scheduledResetRef.current = true;
        } else {
            resetGame();
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

    const handleKeyDown = (event: KeyboardEvent): void => {
        if (keys.includes(event.code)) {
            event.preventDefault();
            pressKey(event.code);
        }
    };

    const handleKeyUp = (event: KeyboardEvent): void => {
        if (keys.includes(event.code)) {
            event.preventDefault();
            releaseKey(event.code);
        }
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

    useEffect(() => {
        initalizeWasm();
    }, []);

    const step = useCallback(() => {
        if (scheduledResetRef.current) {
            resetGame();
        } else if (playing) {
            stepUntilNextAudioBuffer();
        }
    }, [playing]);

    useEffect(() => {
        if (wasmInitialized) {
            (window as any).playAudioSamples = (
                leftAudioSamples: number[],
                rightAudioSamples: number[],
            ): void => {
                const audioContext = audioContextRef.current;

                if (audioContext) {
                    const bufferLength = leftAudioSamples.length;
                    if (bufferLength === 0) {
                        return;
                    }
                    const audioBuffer = audioContext.createBuffer(
                        2,
                        bufferLength,
                        48000,
                    );

                    const leftChannel = audioBuffer.getChannelData(0);
                    const rightChannel = audioBuffer.getChannelData(1);

                    for (let i = 0; i < bufferLength; i++) {
                        leftChannel[i] = leftAudioSamples[i];
                        rightChannel[i] = rightAudioSamples[i];
                    }

                    const bufferSource = audioContext.createBufferSource();
                    bufferSource.buffer = audioBuffer;

                    bufferSource.onended = () => {
                        step();
                    };

                    const gainNode = audioContext.createGain();
                    gainNode.gain.value = 0.25;
                    gainNode.connect(audioContext.destination);

                    bufferSource.connect(gainNode);
                    bufferSource.start();
                }
            };
        }
    }, [wasmInitialized, playing]);

    useEffect(() => {
        if (playing) {
            step();

            window.addEventListener("keydown", handleKeyDown);
            window.addEventListener("keyup", handleKeyUp);
        }

        return () => {
            if (playing) {
                window.removeEventListener("keydown", handleKeyDown);
                window.removeEventListener("keyup", handleKeyUp);
            }
        };
    }, [playing]);

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
                            <BufferFileUpload
                                label="Load ROM"
                                onFileSelect={setRomBuffer}
                                uploadedFile={romBuffer}
                                variant="contained"
                                accept=".gb"
                                startIcon={<FileUploadIcon />}
                            />
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
