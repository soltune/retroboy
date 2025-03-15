import MuiCloseIcon from "@mui/icons-material/Close";
import {
    Box,
    Button,
    IconButton,
    Modal as MuiModal,
    styled,
    Typography,
} from "@mui/material";
import React from "react";

import { CssGrid, GapSize, Orientation, Position } from "./cssGrid";

import { useIsMobile } from "../hooks/useResponsiveBreakpoint";

const ModalContent = styled(Box, {
    shouldForwardProp: prop => prop !== "isMobile",
})<{ isMobile?: boolean }>(({ isMobile, theme }) => ({
    position: "absolute",
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
    paddingTop: "16px",
    paddingBottom: isMobile ? "80px" : "16px",
    paddingLeft: "32px",
    paddingRight: "32px",
    width: isMobile ? "100%" : "512px",
    height: isMobile ? "100%" : "auto",
    background: theme.palette.background.paper,
    boxShadow: "64px",
    "&:focus": {
        outline: "none",
    },
}));

const ModalGrid = styled(CssGrid)`
    height: 100%;
`;

const CloseIcon = styled(MuiCloseIcon)`
    font-size: 28px;
`;

export const ModalGridButton = styled(Button, {
    shouldForwardProp: prop => prop !== "isMobile",
})<{ isMobile?: boolean }>(({ isMobile }) => ({
    justifySelf: isMobile ? "stretch" : "end",
}));

export const Modal = ({
    heading,
    onClose,
    open,
    children,
}: ModalProps): JSX.Element => {
    const isMobile = useIsMobile();
    return (
        <MuiModal open={open} onClose={onClose}>
            <ModalContent isMobile={isMobile}>
                <ModalGrid gap={GapSize.large} template="auto 1fr">
                    <CssGrid
                        template="1fr auto"
                        orientation={Orientation.horizontal}
                        alignItems={Position.start}
                    >
                        <Typography variant="h5">{heading}</Typography>
                        <IconButton onClick={onClose} disableRipple>
                            <CloseIcon />
                        </IconButton>
                    </CssGrid>
                    <div>{children}</div>
                </ModalGrid>
            </ModalContent>
        </MuiModal>
    );
};

interface ModalProps {
    readonly open: boolean;
    readonly heading?: string;
    readonly onClose: () => void;
    readonly children: React.ReactNode;
}
