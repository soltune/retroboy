import { styled } from "@mui/material";
import { useEffect, forwardRef, RefObject } from "react";

export const GAMEBOY_WIDTH = 160;
export const GAMEBOY_HEIGHT = 144;

const DEFAULT_SCALE = 2;

const Screen = styled("canvas", {
    shouldForwardProp: prop => prop !== "fullscreen" && prop !== "scale",
})<{ fullscreen: boolean; scale: number }>(({ fullscreen, scale, theme }) => ({
    width: `${GAMEBOY_WIDTH * scale}px`,
    height: `${GAMEBOY_HEIGHT * scale}px`,
    border: fullscreen
        ? undefined
        : `1px solid ${theme.palette.text.secondary}`,
    imageRendering: "pixelated",
    justifySelf: "center",
}));

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
    ({ playing, paused, scale, fullscreen, ...remainingProps }, ref) => {
        const canvasRef = ref as RefObject<HTMLCanvasElement>;

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

        return (
            <Screen
                width={GAMEBOY_WIDTH}
                height={GAMEBOY_HEIGHT}
                fullscreen={fullscreen}
                scale={scale || DEFAULT_SCALE}
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
    scale?: number;
}

export default GameScreen;
