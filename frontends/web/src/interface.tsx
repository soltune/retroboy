import { useEffect, useRef, useState } from "react";

import {
    buildFileBufferObject,
    FileBufferObject,
} from "./components/bufferFileUpload";
import { gameBoyModes } from "./components/modeSwitch";
import { RomInfo } from "./components/romSelector";
import {
    applySaveState,
    EmulatorSettings,
    encodeSaveState,
    initializeEmulator,
    registerGamegenieCheat,
    registerGamesharkCheat,
    resetEmulator,
} from "./core/retroboyCore";
import useAudioSync from "./hooks/useAudioSync";
import { useKeyListeners } from "./hooks/useKeyListeners";
import { useIsMobile } from "./hooks/useResponsiveBreakpoint";
import { useSettingsStore } from "./hooks/useSettingsStore";
import { useTopLevelRenderer } from "./hooks/useTopLevelRenderer";
import { CheatType } from "./modals/cheatsModal";
import { CheatsModal } from "./modals/cheatsModal";
import { ControlsModal } from "./modals/controlsModal";
import { MessageDialog } from "./modals/dialog";
import FullscreenView from "./views/fullscreenView";
import StandardView from "./views/standardView";

const errorModalKey = "error-modal";
const controlsModalKey = "controls-modal";
const cheatsModalKey = "cheats-modal";

const Interface = (): JSX.Element => {
    const isMobile = useIsMobile();

    const { displayTopLevelComponent, removeTopLevelComponent } =
        useTopLevelRenderer();

    const [rom, setRom] = useState(null as FileBufferObject | null);
    const [selectedRomInfo, setSelectedRomInfo] = useState(
        null as RomInfo | null,
    );
    const [loadingRom, setLoadingRom] = useState(false);

    const [gameKey, setGameKey] = useState(null as string | null);
    const [playing, setPlaying] = useState(false);
    const [paused, setPaused] = useState(false);
    const [mode, setMode] = useState(gameBoyModes.dmg);
    const [fullscreenMode, setFullscreenMode] = useState(false);
    const [usingModal, setUsingModal] = useState(false);
    const [muted, setMuted] = useState(false);

    useKeyListeners(playing, usingModal);

    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const loadStateRef = useRef<HTMLInputElement>(null);

    const resetGame = (): void => {
        setGameKey(null);
        setPlaying(false);
        setPaused(false);
        resetEmulator();
        setRom(null);
        //setSelectedRomInfo(null);
    };

    const [audioContextRef, startReset] = useAudioSync(
        playing,
        muted,
        resetGame,
    );

    const scrollToGamePad = ({ smooth }: { smooth: boolean }) => {
        window.scrollTo({
            top: document.body.scrollHeight,
            behavior: smooth ? "smooth" : "auto",
        });
    };

    const { settings } = useSettingsStore();

    const registerCheats = (newGameKey: string): void => {
        const cheatsById = settings.cheats ? settings.cheats[newGameKey] : {};
        const cheats = Object.values(cheatsById || {});
        cheats.forEach(cheat => {
            if (cheat.registered) {
                if (cheat.type == CheatType.GameShark) {
                    registerGamesharkCheat(cheat.id, cheat.code);
                } else {
                    registerGamegenieCheat(cheat.id, cheat.code);
                }
            }
        });
    };

    const playGame = () => {
        if (rom) {
            if (!audioContextRef.current) {
                audioContextRef.current = new AudioContext();
            }

            const emulatorSettings = new EmulatorSettings(
                mode,
                audioContextRef.current.sampleRate,
            );
            const { error, metadata } = initializeEmulator(
                rom.data,
                emulatorSettings,
            );

            if (error) {
                openErrorDialog(error);
                resetGame();
            } else if (metadata) {
                setGameKey(metadata.title);
                registerCheats(metadata.title);
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
        setFullscreenMode(true);
        if (document.body && document.body.requestFullscreen) {
            document.body.requestFullscreen();
        }
    };

    const onFullscreenChange = (): void => {
        if (!document.fullscreenElement) {
            exitFullscreen();
        }
    };

    const exitFullscreen = (): void => {
        setFullscreenMode(false);
        if (document.exitFullscreen) {
            document.exitFullscreen().catch(() => {});
        }
    };

    const toggleMute = (): void => {
        setMuted(!muted);
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

    const openErrorDialog = (message: string): void => {
        displayTopLevelComponent(
            errorModalKey,
            <MessageDialog
                heading="Error"
                message={message}
                onClose={() => removeTopLevelComponent(errorModalKey)}
            />,
        );
    };

    const openControls = (): void => {
        setUsingModal(true);
        displayTopLevelComponent(
            controlsModalKey,
            <ControlsModal
                onClose={() => {
                    removeTopLevelComponent(controlsModalKey);
                    setUsingModal(false);
                }}
            />,
        );
    };

    const handleRomSelect = async (romInfo: RomInfo | null): Promise<void> => {
        // If a game is currently running, stop it first
        if (playing || paused) {
            startReset();
        }

        if (!romInfo) {
            setRom(null);
            setSelectedRomInfo(null);
            return;
        }

        setLoadingRom(true);
        setSelectedRomInfo(romInfo);

        try {
            const response = await fetch(romInfo.path);
            if (!response.ok) {
                throw new Error(`Failed to load ROM: ${response.statusText}`);
            }

            const arrayBuffer = await response.arrayBuffer();
            const data = new Uint8Array(arrayBuffer);
            const filename = romInfo.path.split("/").pop() || romInfo.name;

            const fileObject: FileBufferObject = {
                filename,
                data,
            };

            if (filename.endsWith(".gbc") && mode === gameBoyModes.dmg) {
                setMode(gameBoyModes.cgb);
            }

            setRom(fileObject);
        } catch (error) {
            console.error("Error loading ROM:", error);
            openErrorDialog(
                `Failed to load ROM: ${error instanceof Error ? error.message : "Unknown error"}`,
            );
            setSelectedRomInfo(null);
        } finally {
            setLoadingRom(false);
        }
    };

    const openCheats = (): void => {
        if (gameKey) {
            setUsingModal(true);
            displayTopLevelComponent(
                cheatsModalKey,
                <CheatsModal
                    gameKey={gameKey}
                    onClose={() => {
                        removeTopLevelComponent(cheatsModalKey);
                        setUsingModal(false);
                    }}
                />,
            );
        }
    };

    const handleLoadStateChange = async (
        event: React.ChangeEvent<HTMLInputElement>,
    ): Promise<void> => {
        const fileList = event.target.files;
        if (!fileList) return;
        const file = fileList[0];
        if (file) {
            try {
                const bufferObject = await buildFileBufferObject(file);
                const error = applySaveState(bufferObject.data);
                if (error) {
                    openErrorDialog(error);
                }
            } catch (err) {
                openErrorDialog(
                    "An error occurred while loading the save state.",
                );
                console.error("Error loading save state:", err);
            }

            if (loadStateRef.current) {
                loadStateRef.current.value = "";
            }
        }
    };

    const saveState = (): void => {
        if (gameKey && rom) {
            const result = encodeSaveState();
            if (result.error) {
                openErrorDialog(result.error);
            } else if (result.saveState) {
                const blob = new Blob([result.saveState], {
                    type: "application/octet-stream",
                });
                const url = URL.createObjectURL(blob);
                const a = document.createElement("a");
                a.href = url;
                const saveStateName = rom.filename.split(".")[0];
                const filename = `${saveStateName}.rbs`;
                a.download = filename;
                a.click();
                URL.revokeObjectURL(url);
            }
        }
    };

    useEffect(() => {
        if (playing && !fullscreenMode && isMobile) {
            setTimeout(() => {
                window.requestAnimationFrame(() =>
                    scrollToGamePad({ smooth: false }),
                );
            });
        }
    }, [fullscreenMode]);

    useEffect(() => {
        document.addEventListener("fullscreenchange", onFullscreenChange);
        return () => {
            document.removeEventListener(
                "fullscreenchange",
                onFullscreenChange,
            );
        };
    }, []);

    return fullscreenMode ? (
        <FullscreenView
            playing={playing}
            paused={paused}
            onExitFullscreen={exitFullscreen}
            canvasRef={canvasRef}
        />
    ) : (
        <StandardView
            gameKey={gameKey}
            playing={playing}
            paused={paused}
            rom={rom}
            selectedRomInfo={selectedRomInfo}
            mode={mode}
            muted={muted}
            onRomSelect={handleRomSelect}
            onModeChange={setMode}
            onPlay={playGame}
            onPause={pauseGame}
            onResume={resumeGame}
            onReset={startReset}
            onFullscreen={setFullscreen}
            onScreenshot={downloadScreenshot}
            onOpenControls={openControls}
            onOpenCheats={openCheats}
            onLoadStateChange={handleLoadStateChange}
            onSaveState={saveState}
            onMuteToggle={toggleMute}
            canvasRef={canvasRef}
            loadStateRef={loadStateRef}
        />
    );
};

export default Interface;
