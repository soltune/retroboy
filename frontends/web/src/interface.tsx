import { useEffect, useRef, useState } from "react";

import { FileBufferObject } from "./components/bufferFileUpload";
import Modal from "./components/modal";
import { gameBoyModes } from "./components/modeSwitch";
import {
    EmulatorSettings,
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
import { useIsMobile } from "./hooks/useResponsiveBreakpoint";
import { useTopLevelRenderer } from "./hooks/useTopLevelRenderer";
import SettingsModal from "./settingsModal";
import FullscreenView from "./views/fullscreenView";
import StandardView from "./views/standardView";

const errorModalKey = "error-modal";
const settingsModalKey = "settings-modal";

const Interface = (): JSX.Element => {
    const isMobile = useIsMobile();

    const { displayTopLevelComponent, removeTopLevelComponent } =
        useTopLevelRenderer();

    const [romBuffer, setRomBuffer] = useState(null as FileBufferObject | null);

    const [playing, setPlaying] = useState(false);
    const [paused, setPaused] = useState(false);
    const [mode, setMode] = useState(gameBoyModes.dmg);
    const [romMetadata, setRomMetadata] = useState(null as RomMetadata | null);
    const [fullscreenMode, setFullscreenMode] = useState(false);

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

    const downloadScreenshot = (): void => {
        if (canvasRef.current) {
            const dataUrl = canvasRef.current.toDataURL("image/png");
            const link = document.createElement("a");
            link.href = dataUrl;
            link.download = "retroboy-screenshot.png";
            link.click();
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
            playing={playing}
            paused={paused}
            romBuffer={romBuffer}
            mode={mode}
            onRomBufferChange={setRomBuffer}
            onModeChange={setMode}
            onPlay={playGame}
            onPause={pauseGame}
            onResume={resumeGame}
            onReset={startReset}
            onFullscreen={setFullscreen}
            onScreenshot={downloadScreenshot}
            onOpenSettings={openSettings}
            canvasRef={canvasRef}
        />
    );
};

export default Interface;
