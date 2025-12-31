import { useState, useEffect, useCallback } from 'react';

export type TimerMode = 'countup' | 'countdown';

interface UseTimerProps {
  initialSeconds?: number;
  mode?: TimerMode;
  autoStart?: boolean;
}

interface TimerState {
  totalSeconds: number;
  minutes: number;
  seconds: number;
  milliseconds: number;
  isRunning: boolean;
}

export const useTimer = ({
  initialSeconds = 0,
  mode = 'countup',
  autoStart = false,
}: UseTimerProps = {}) => {
  const [totalSeconds, setTotalSeconds] = useState(initialSeconds);
  const [isRunning, setIsRunning] = useState(autoStart);

  useEffect(() => {
    if (!isRunning) return;

    const interval = setInterval(() => {
      setTotalSeconds((prev) => {
        if (mode === 'countdown') {
          return prev > 0 ? prev - 0.01 : 0;
        } else {
          return prev + 0.01;
        }
      });
    }, 10); // Update every 10ms for millisecond precision

    return () => clearInterval(interval);
  }, [isRunning, mode]);

  const minutes = Math.floor(totalSeconds / 60);
  const seconds = Math.floor(totalSeconds % 60);
  const milliseconds = Math.floor((totalSeconds % 1) * 100);

  const formatTime = useCallback(() => {
    return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}.${String(milliseconds).padStart(2, '0')}`;
  }, [minutes, seconds, milliseconds]);

  const start = useCallback(() => setIsRunning(true), []);
  const pause = useCallback(() => setIsRunning(false), []);
  const reset = useCallback(() => {
    setTotalSeconds(initialSeconds);
    setIsRunning(false);
  }, [initialSeconds]);
  const resume = useCallback(() => setIsRunning(true), []);

  return {
    totalSeconds,
    minutes,
    seconds,
    milliseconds,
    isRunning,
    formatTime,
    start,
    pause,
    reset,
    resume,
    setTotalSeconds,
  };
};
