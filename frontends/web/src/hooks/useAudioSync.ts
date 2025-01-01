import { MutableRefObject, useCallback, useEffect, useRef } from "react";

import { stepUntilNextAudioBuffer } from "../core/retroboyCore";

const useAudioSync = (
    playing: boolean,
    resetGameCallback: () => void,
): [MutableRefObject<AudioContext | null>, () => void] => {
    const audioContextRef = useRef<AudioContext | null>(null);
    const scheduledResetRef = useRef<boolean>(false);

    const nextPlayTimeRef = useRef<number>(0);

    const resetGame = (): void => {
        resetGameCallback();
        scheduledResetRef.current = false;
    };

    const startReset = (): void => {
        if (playing) {
            scheduledResetRef.current = true;
        } else {
            resetGame();
        }
    };

    const step = useCallback(() => {
        if (scheduledResetRef.current) {
            resetGame();
        } else if (playing) {
            stepUntilNextAudioBuffer();
        }
    }, [playing]);

    useEffect(() => {
        if (playing && audioContextRef.current) {
            nextPlayTimeRef.current = audioContextRef.current.currentTime;
        }
    }, [playing]);

    const GAP_BEFORE_SAMPLE_PLAY = 15;

    useEffect(() => {
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
                    audioContext.sampleRate,
                );

                const leftChannel = audioBuffer.getChannelData(0);
                const rightChannel = audioBuffer.getChannelData(1);

                for (let i = 0; i < bufferLength; i++) {
                    leftChannel[i] = leftAudioSamples[i];
                    rightChannel[i] = rightAudioSamples[i];
                }

                const bufferSource = audioContext.createBufferSource();
                bufferSource.buffer = audioBuffer;

                const duration = bufferLength / audioContext.sampleRate;

                bufferSource.connect(audioContext.destination);

                bufferSource.start(nextPlayTimeRef.current);

                const waitTime =
                    (nextPlayTimeRef.current - audioContext.currentTime) * 1000;

                setTimeout(step, waitTime - GAP_BEFORE_SAMPLE_PLAY);

                nextPlayTimeRef.current = nextPlayTimeRef.current
                    ? nextPlayTimeRef.current + duration
                    : duration;
            }
        };
    }, [playing]);

    useEffect(() => {
        if (playing) {
            step();
        }
    }, [playing]);

    return [audioContextRef, startReset];
};

export default useAudioSync;
