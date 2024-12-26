import { styled } from "@mui/material";
import { RefObject, forwardRef, useEffect } from "react";

const GAMEBOY_WIDTH = 160;
const GAMEBOY_HEIGHT = 144;

const Screen = styled("canvas")`
    width: ${GAMEBOY_WIDTH * 2}px;
    height: ${GAMEBOY_HEIGHT * 2}px;
    border: ${({ theme }) => `1px solid ${theme.palette.text.secondary}`};
    image-rendering: pixelated;
`;

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

const GameScreen = forwardRef<HTMLCanvasElement, GameScreenProps>(
    ({ wasmInitialized, playing, paused, ...remainingProps }, ref) => {
        const canvasRef = ref as RefObject<HTMLCanvasElement>;

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

        return (
            <Screen
                width={GAMEBOY_WIDTH}
                height={GAMEBOY_HEIGHT}
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
}

export default GameScreen;
