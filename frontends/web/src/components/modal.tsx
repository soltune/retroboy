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
    padding: "16px 32px",
    width: isMobile ? "100%" : "512px",
    height: isMobile ? "100%" : "auto",
    background: theme.palette.background.paper,
    boxShadow: "64px",
}));

const ModalGrid = styled(CssGrid)`
    height: 100%;
`;

const CloseIcon = styled(MuiCloseIcon)`
    font-size: 28px;
`;

const CloseButton = styled(Button, {
    shouldForwardProp: prop => prop !== "isMobile",
})<{ isMobile?: boolean }>(({ isMobile }) => ({
    justifySelf: isMobile ? "stretch" : "end",
}));

const Modal = ({
    heading,
    onClose,
    open,
    children,
}: ModalProps): JSX.Element => {
    const isMobile = useIsMobile();
    return (
        <MuiModal open={open} onClose={onClose}>
            <ModalContent isMobile={isMobile}>
                <ModalGrid gap={GapSize.large} template="auto 1fr auto">
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
                    <CloseButton
                        variant="contained"
                        onClick={onClose}
                        isMobile={isMobile}
                    >
                        OK
                    </CloseButton>
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

export default Modal;
