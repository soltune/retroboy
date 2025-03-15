import { InputProps } from "@mui/material/Input";
import OutlinedInput, { OutlinedInputProps } from "@mui/material/OutlinedInput";
import { FieldProps, FieldRenderProps } from "react-final-form";

import { FieldWrapper } from "./fieldWrapper";
import { paddingChangeStyles } from "./sharedStyles";

export const FieldInput = ({
    InputProps,
    ...fieldProps
}: FieldInputProps): JSX.Element => (
    <FieldWrapper formatOnBlur {...fieldProps}>
        {({ input }: FieldWrapperRenderProps) => {
            return (
                <OutlinedInput
                    {...input}
                    id={fieldProps.name}
                    disabled={fieldProps.disabled}
                    {...InputProps}
                    autoComplete="off"
                    sx={{
                        ...paddingChangeStyles,
                        ...(InputProps?.sx || {}),
                    }}
                />
            );
        }}
    </FieldWrapper>
);

type FieldWrapperRenderProps = FieldRenderProps<
    string,
    HTMLInputElement | HTMLTextAreaElement
>;
export interface FieldInputProps
    extends FieldProps<string, FieldWrapperRenderProps, HTMLInputElement> {
    InputProps?: Partial<OutlinedInputProps> | Partial<InputProps>;
}
