import { styled } from "@mui/material";
import { RefObject, forwardRef, useEffect, useState } from "react";

const GAMEBOY_WIDTH = 160;
const GAMEBOY_HEIGHT = 144;

const DEFAULT_SCALE = 2;

const Screen = styled("canvas", {
    shouldForwardProp: prop => prop !== "mobileFullscreen" && prop !== "scale",
})<{ mobileFullscreen: boolean; scale: number }>(
    ({ mobileFullscreen, scale, theme }) => ({
        width: `${GAMEBOY_WIDTH * scale}px`,
        height: `${GAMEBOY_HEIGHT * scale}px`,
        border: mobileFullscreen
            ? undefined
            : `1px solid ${theme.palette.text.secondary}`,
        imageRendering: "pixelated",
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
    (
        {
            wasmInitialized,
            playing,
            paused,
            mobileFullscreen,
            ...remainingProps
        },
        ref,
    ) => {
        const canvasRef = ref as RefObject<HTMLCanvasElement>;

        const [scale, setScale] = useState(DEFAULT_SCALE);

        useEffect(() => {
            if (wasmInitialized && canvasRef.current) {
                const canvas = canvasRef.current;
                const canvasContext = canvas.getContext("2d");

                if (canvasContext) {
                    (window as any).canvasRender = (buffer: number[]): void => {
                        renderFrame(canvasContext, buffer);
                    };
                }
            }
        }, [wasmInitialized]);

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
            if (mobileFullscreen) {
                setScale(window.innerWidth / GAMEBOY_WIDTH);
            }

            return () => {
                if (mobileFullscreen) {
                    setScale(DEFAULT_SCALE);
                }
            };
        }, [mobileFullscreen]);

        return (
            <Screen
                width={GAMEBOY_WIDTH}
                height={GAMEBOY_HEIGHT}
                mobileFullscreen={mobileFullscreen}
                scale={scale}
                ref={ref}
                {...remainingProps}
            />
        );
    },
);

interface GameScreenProps
    extends React.CanvasHTMLAttributes<HTMLCanvasElement> {
    wasmInitialized: boolean;
    playing: boolean;
    paused: boolean;
    mobileFullscreen: boolean;
}

export default GameScreen;
