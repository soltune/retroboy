import { createContext, useContext, useEffect, useState } from "react";

export enum ResponsiveBreakpoint {
    lg = "lg",
    md = "md",
    sm = "sm",
    xs = "xs",
}

const breakpoints = {
    lg: {
        name: ResponsiveBreakpoint.lg,
        min: 1200,
    },
    md: {
        name: ResponsiveBreakpoint.md,
        min: 992,
    },
    sm: {
        name: ResponsiveBreakpoint.sm,
        min: 768,
    },
    xs: {
        name: ResponsiveBreakpoint.xs,
    },
};

const getBreakpointFromWindowSize = (
    windowSize: number,
): ResponsiveBreakpoint => {
    return windowSize >= breakpoints.lg.min
        ? ResponsiveBreakpoint.lg
        : windowSize >= breakpoints.md.min
          ? ResponsiveBreakpoint.md
          : windowSize >= breakpoints.sm.min
            ? ResponsiveBreakpoint.sm
            : ResponsiveBreakpoint.xs;
};

const initialBreakpoint = getBreakpointFromWindowSize(
    typeof window !== "undefined" ? window.innerWidth : breakpoints.md.min,
);
const ResponsiveBreakpointContext = createContext(initialBreakpoint);

export const ResponsiveBreakpointProvider = ({
    children,
}: ResponsiveBreakpointProviderProps): JSX.Element => {
    const [width, setWidth] = useState(null as number | null);

    const handleWindowResize = () => {
        setWidth(window.innerWidth);
    };

    useEffect(() => {
        window.addEventListener("resize", handleWindowResize);
        return () => window.removeEventListener("resize", handleWindowResize);
    }, []);

    useEffect(() => {
        handleWindowResize();
    }, []);

    const breakpoint =
        width !== null ? getBreakpointFromWindowSize(width) : null;
    return (
        <>
            {breakpoint && (
                <ResponsiveBreakpointContext.Provider value={breakpoint}>
                    {children}
                </ResponsiveBreakpointContext.Provider>
            )}
        </>
    );
};

export const useResponsiveBreakpoint = (): ResponsiveBreakpoint =>
    useContext(ResponsiveBreakpointContext);

export const useIsMobile = (): boolean => {
    const breakpoint = useResponsiveBreakpoint();
    return breakpoint === ResponsiveBreakpoint.xs;
};

interface ResponsiveBreakpointProviderProps {
    children: React.ReactNode;
}
