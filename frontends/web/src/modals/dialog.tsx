import { CssGrid, GapSize, Orientation, Position } from "../components/cssGrid";
import { Modal, ModalGridButton } from "../components/modal";
import { useIsMobile } from "../hooks/useResponsiveBreakpoint";

export const MessageDialog = ({
    onClose,
    message,
    heading,
}: MessageDialogProps): JSX.Element => {
    return (
        <Modal heading={heading} open={true} onClose={onClose}>
            <CssGrid orientation={Orientation.vertical} gap={GapSize.large}>
                <div>{message}</div>
                <ModalGridButton
                    variant="contained"
                    color="primary"
                    onClick={onClose}
                >
                    OK
                </ModalGridButton>
            </CssGrid>
        </Modal>
    );
};

export const YesNoDialog = ({
    onClose,
    onYes,
    onNo,
    message,
    heading,
}: YesNoDialogProps): JSX.Element => {
    const isMobile = useIsMobile();
    return (
        <Modal heading={heading} open={true} onClose={onClose}>
            <CssGrid orientation={Orientation.vertical} gap={GapSize.large}>
                <div>{message}</div>
                <CssGrid
                    orientation={
                        isMobile ? Orientation.vertical : Orientation.horizontal
                    }
                    gap={GapSize.large}
                    justifyContent={isMobile ? Position.stretch : Position.end}
                >
                    <ModalGridButton
                        variant="contained"
                        color="primary"
                        onClick={onYes}
                        isMobile={isMobile}
                    >
                        Yes
                    </ModalGridButton>
                    <ModalGridButton
                        variant="contained"
                        color="primary"
                        onClick={onNo}
                        isMobile={isMobile}
                    >
                        No
                    </ModalGridButton>
                </CssGrid>
            </CssGrid>
        </Modal>
    );
};

interface YesNoDialogProps {
    readonly onClose: () => void;
    readonly onYes: () => void;
    readonly onNo: () => void;
    readonly message: string;
    readonly heading: string;
}

interface MessageDialogProps {
    readonly onClose: () => void;
    readonly message: string;
    readonly heading: string;
}
