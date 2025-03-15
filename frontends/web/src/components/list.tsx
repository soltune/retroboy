import styled from "@emotion/styled";
import { alpha } from "@mui/material";

import { CssGrid } from "./cssGrid";

export const ListGrid = styled(CssGrid, {
    shouldForwardProp: prop => prop !== "isMobile",
})<{ isMobile: boolean }>(({ isMobile }) => ({
    maxHeight: "50vh",
    width: "100%",
    overflowY: "scroll",
    marginBottom: "16px",
}));

export const ListItemGrid = styled(CssGrid, {
    shouldForwardProp: prop => prop !== "selected",
})<{ selected?: boolean }>(({ selected, theme }) => ({
    background: selected
        ? theme.palette.primary.main
        : theme.palette.background.paper,
    color: selected
        ? theme.palette.primary.contrastText
        : theme.palette.text.primary,
    border: `1px solid ${theme.palette.text.secondary}`,
    padding: "4px 8px",
    "&:hover": {
        background: selected
            ? undefined
            : alpha(theme.palette.primary.dark, 0.2),
        cursor: "pointer",
    },
}));
