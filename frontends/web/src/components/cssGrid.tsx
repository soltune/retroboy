import isPropValid from "@emotion/is-prop-valid";
import { styled } from "@mui/material/styles";

export enum GapSize {
    small = 4,
    medium = 8,
    mediumLarge = 12,
    large = 16,
    extraLarge = 32,
    giant = 64,
}

export enum Orientation {
    horizontal = "column",
    vertical = "row",
}

export enum Position {
    start = "start",
    center = "center",
    end = "end",
    stretch = "stretch",
}

const shouldForwardProp = (prop: string): boolean =>
    isPropValid(prop) && prop !== "orientation";

export const CssGrid = styled("div", { shouldForwardProp })<GridProps>(
    ({
        inline,
        orientation,
        template,
        gap,
        alignContent,
        alignItems,
        justifyContent,
        justifyItems,
    }) => ({
        display: inline ? "inline-grid" : "grid",
        gridTemplateRows:
            orientation !== Orientation.horizontal && template
                ? template
                : undefined,
        gridTemplateColumns:
            orientation === Orientation.horizontal && template
                ? template
                : undefined,
        gridAutoFlow: !template ? orientation : undefined,
        gap: gap ? `${gap}px` : undefined,
        alignContent,
        alignItems,
        justifyContent,
        justifyItems,
    }),
);

interface GridProps {
    inline?: boolean;
    gap?: GapSize;
    orientation?: Orientation;
    template?: string;
    alignContent?: Position;
    alignItems?: Position;
    justifyContent?: Position;
    justifyItems?: Position;
}
