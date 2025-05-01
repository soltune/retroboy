import Button, { ButtonProps } from "@mui/material/Button";
import { styled } from "@mui/material/styles";
import { useRef } from "react";

import { HiddenInput } from "./inputStyles";

const UploadButton = styled(Button)`
    width: 100%;
`;

export const openFileDialog = (
    fileInputRef: React.RefObject<HTMLInputElement>,
): void => {
    const fileInputDomObject = fileInputRef.current;
    if (fileInputDomObject) {
        fileInputDomObject.click();
    }
};

export const FileUploadButton = ({
    accept,
    onFileSelect,
    ...buttonProps
}: FileUploadButtonProps): JSX.Element => {
    const fileInputRef = useRef<HTMLInputElement | null>(null);
    return (
        <div>
            <HiddenInput
                type="file"
                accept={accept}
                ref={fileInputRef}
                onChange={event => {
                    onFileSelect(event.target.files);
                    if (fileInputRef.current) {
                        fileInputRef.current.value = "";
                    }
                }}
            />
            <UploadButton
                onClick={() => openFileDialog(fileInputRef)}
                type="button"
                {...buttonProps}
            />
        </div>
    );
};

export interface FileUploadButtonProps extends ButtonProps {
    accept: string;
    children: React.ReactNode;
    onFileSelect: (fileList: FileList | null) => void;
}
