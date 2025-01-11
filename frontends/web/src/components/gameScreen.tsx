import { styled } from "@mui/material";
import { useEffect, useState, useRef, forwardRef, RefObject } from "react";

import { useIsMobile } from "../hooks/useResponsiveBreakpoint";

const GAMEBOY_WIDTH = 160;
const GAMEBOY_HEIGHT = 144;

const DEFAULT_SCALE = 2;

const Screen = styled("canvas", {
    shouldForwardProp: prop =>
        prop !== "fullscreen" && prop !== "scale" && prop !== "isMobile",
})<{ fullscreen: boolean; scale: number; isMobile: boolean }>(
    ({ fullscreen, scale, isMobile, theme }) => ({
        width: fullscreen && !isMobile ? "100%" : `${GAMEBOY_WIDTH * scale}px`,
        height:
            fullscreen && !isMobile ? "100%" : `${GAMEBOY_HEIGHT * scale}px`,
        border: fullscreen
            ? undefined
            : `1px solid ${theme.palette.text.secondary}`,
        imageRendering: "pixelated",
        justifySelf: "center",
        alignSelf: "center",
    }),
);

const renderFrame = (
    canvasContext: CanvasRenderingContext2D,
    buffer: number[],
): void => {
    const data = new Uint8ClampedArray(buffer);
    const imageData = new ImageData(data, GAMEBOY_WIDTH, GAMEBOY_HEIGHT);
    canvasContext.putImageData(imageData, 0, 0);
};

const initializeCanvas = (canvasContext: CanvasRenderingContext2D): void => {
    const initialBuffer = [] as number[];

    for (let i = 0; i < GAMEBOY_WIDTH * GAMEBOY_HEIGHT; i++) {
        const offset = i * 4;
        initialBuffer[offset] = 0;
        initialBuffer[offset + 1] = 0;
        initialBuffer[offset + 2] = 0;
        initialBuffer[offset + 3] = 0xff;
    }

    renderFrame(canvasContext, initialBuffer);
};

export const GameScreen = forwardRef<HTMLCanvasElement, GameScreenProps>(
    ({ playing, paused, fullscreen, ...remainingProps }, ref) => {
        const isMobile = useIsMobile();
        const canvasRef = ref as RefObject<HTMLCanvasElement>;

        const [scale, setScale] = useState(DEFAULT_SCALE);

        useEffect(() => {
            if (canvasRef.current) {
                const canvas = canvasRef.current;
                const canvasContext = canvas.getContext("2d");

                if (canvasContext) {
                    (window as any).canvasRender = (buffer: number[]): void => {
                        renderFrame(canvasContext, buffer);
                    };
                }
            }

            return () => {
                (window as any).canvasRender = (_: number[]) => {};
            };
        }, []);

        useEffect(() => {
            if (canvasRef.current) {
                const canvas = canvasRef.current;
                const canvasContext = canvas.getContext("2d");

                if (canvasContext && !paused && !playing) {
                    initializeCanvas(canvasContext);
                }
            }
        }, [playing, paused]);

        useEffect(() => {
            if (fullscreen) {
                setScale(window.innerWidth / GAMEBOY_WIDTH);
            }

            return () => {
                if (fullscreen) {
                    setScale(DEFAULT_SCALE);
                }
            };
        }, [fullscreen]);

        return (
            <Screen
                isMobile={isMobile}
                width={GAMEBOY_WIDTH}
                height={GAMEBOY_HEIGHT}
                fullscreen={fullscreen}
                scale={scale}
                ref={canvasRef}
                {...remainingProps}
            />
        );
    },
);

interface GameScreenProps
    extends React.CanvasHTMLAttributes<HTMLCanvasElement> {
    playing: boolean;
    paused: boolean;
    fullscreen: boolean;
}

export default GameScreen;
