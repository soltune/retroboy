import { CssGrid, Orientation, GapSize, Position } from "./cssGrid";
import { FileUploadButton, FileUploadButtonProps } from "./fileUploadButton";

import { useIsMobile } from "../hooks/useResponsiveBreakpoint";

export const buildFileBufferObject = (
    uploadedFile: File,
): Promise<FileBufferObject> =>
    new Promise(resolve => {
        uploadedFile.arrayBuffer().then(buffer => {
            resolve({
                filename: uploadedFile.name,
                data: new Uint8Array(buffer),
            });
        });
    });

const getFieldFileUploadLabel = (value: FileBufferObject | null): string => {
    return value ? value.filename : "No file chosen";
};

export const BufferFileUpload = ({
    onFileSelect,
    uploadedFile,
    label,
    ...remainingProps
}: BufferFileUploadProps): JSX.Element => {
    const isMobile = useIsMobile();
    return (
        <CssGrid
            orientation={
                isMobile ? Orientation.vertical : Orientation.horizontal
            }
            gap={GapSize.medium}
            alignItems={Position.center}
            justifyContent={isMobile ? Position.stretch : Position.start}
        >
            <FileUploadButton
                variant="contained"
                {...remainingProps}
                onFileSelect={async (fileList: FileList | null) => {
                    if (!fileList) return;

                    const file = fileList[0];

                    if (file) {
                        try {
                            const bufferObject =
                                await buildFileBufferObject(file);
                            onFileSelect(bufferObject);
                        } catch (err) {
                            console.error(
                                "An error occurred while processing the file",
                                err,
                            );
                        }
                    }
                }}
            >
                <CssGrid
                    orientation={Orientation.horizontal}
                    gap={GapSize.medium}
                    alignItems={Position.center}
                >
                    {label || "Choose File"}
                </CssGrid>
            </FileUploadButton>
            <div>{getFieldFileUploadLabel(uploadedFile)}</div>
        </CssGrid>
    );
};

export interface FileBufferObject {
    filename: string;
    data: Uint8Array;
}

interface BufferFileUploadProps
    extends Omit<FileUploadButtonProps, "onFileSelect" | "children"> {
    onFileSelect: (file: FileBufferObject | null) => void;
    uploadedFile: FileBufferObject | null;
    label?: string;
}
