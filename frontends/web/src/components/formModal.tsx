import { Form, FormRenderProps } from "react-final-form";

import { CssGrid, GapSize, Orientation, Position } from "./cssGrid";
import { Modal, ModalGridButton } from "./modal";

import { useIsMobile } from "../hooks/useResponsiveBreakpoint";

const FormModal = ({
    title,
    children,
    cancelButtonText,
    submitButtonText,
    initialValues,
    onSubmit,
    onCancel,
    onClose,
}: FormModalProps): JSX.Element => {
    const isMobile = useIsMobile();
    return (
        <Modal heading={title} open={true} onClose={onClose}>
            <Form
                initialValues={initialValues}
                onSubmit={values => onSubmit({ closeDialog: onClose, values })}
            >
                {({ handleSubmit }: FormRenderProps): JSX.Element => (
                    <form onSubmit={handleSubmit}>
                        <CssGrid
                            gap={GapSize.large}
                            orientation={Orientation.vertical}
                        >
                            {children}
                            <CssGrid
                                gap={GapSize.large}
                                orientation={
                                    isMobile
                                        ? Orientation.vertical
                                        : Orientation.horizontal
                                }
                                justifyContent={
                                    isMobile ? Position.stretch : Position.end
                                }
                            >
                                <ModalGridButton
                                    variant="contained"
                                    color="primary"
                                    type="submit"
                                    isMobile={isMobile}
                                >
                                    {submitButtonText || "Submit"}
                                </ModalGridButton>
                                <ModalGridButton
                                    variant="contained"
                                    color="secondary"
                                    onClick={() => {
                                        if (onCancel) {
                                            onCancel();
                                        } else {
                                            onClose();
                                        }
                                    }}
                                    isMobile={isMobile}
                                >
                                    {cancelButtonText || "Cancel"}
                                </ModalGridButton>
                            </CssGrid>
                        </CssGrid>
                    </form>
                )}
            </Form>
        </Modal>
    );
};

export interface SubmitActionProps {
    closeDialog: () => void;
    values: Record<string, unknown>;
}

export interface FormModalProps {
    title: string;
    submitButtonText?: string;
    cancelButtonText?: string;
    initialValues?: Record<string, unknown>;
    onSubmit: (props: SubmitActionProps) => Record<string, string> | undefined;
    onClose: () => void;
    onCancel?: () => void;
    children?: React.ReactNode;
}

export default FormModal;
