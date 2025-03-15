import Alert, { AlertColor } from "@mui/material/Alert";
import Snackbar from "@mui/material/Snackbar";
import { useState } from "react";

import { useTopLevelRenderer } from "./useTopLevelRenderer";

const TOAST_DURATION = 5000;
const TOAST_COMPONENT_KEY = "toast";

export const Toast = ({ text, severity, onClose }: ToastProps): JSX.Element => {
    const [open, setOpen] = useState(true);
    const closeToast = () => setOpen(false);
    return (
        <Snackbar
            open={open}
            anchorOrigin={{ vertical: "top", horizontal: "right" }}
            autoHideDuration={TOAST_DURATION}
            onClose={closeToast}
            TransitionProps={{ onExited: () => onClose() }}
        >
            <Alert onClose={closeToast} severity={severity as AlertColor}>
                {text}
            </Alert>
        </Snackbar>
    );
};

export const useToastRenderer = (): ToastRenderer => {
    const { displayTopLevelComponent, removeTopLevelComponent } =
        useTopLevelRenderer();

    const removeToast = () => removeTopLevelComponent(TOAST_COMPONENT_KEY);

    const displayToast = (text: string, severity: string): void => {
        displayTopLevelComponent(
            TOAST_COMPONENT_KEY,
            <Toast text={text} severity={severity} onClose={removeToast} />,
        );
    };

    const buildToast = (severity: string) => (text: string) =>
        displayToast(text, severity);

    return {
        success: buildToast("success"),
        warning: buildToast("warning"),
        error: buildToast("error"),
    };
};

export interface ToastRenderer {
    success: (text: string) => void;
    warning: (text: string) => void;
    error: (text: string) => void;
}

interface ToastProps {
    text: string;
    severity: string;
    onClose: () => void;
}
