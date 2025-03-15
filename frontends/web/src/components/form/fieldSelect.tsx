import MenuItem from "@mui/material/MenuItem";
import Select, { SelectProps } from "@mui/material/Select";
import { FieldProps, FieldRenderProps } from "react-final-form";

import { FieldWrapper } from "./fieldWrapper";
import { paddingChangeStyles } from "./sharedStyles";

export const FieldSelect = ({
    options,
    SelectProps,
    ...fieldProps
}: FieldSelectProps): JSX.Element => {
    return (
        <FieldWrapper {...fieldProps}>
            {({ input }: FieldWrapperRenderProps) => {
                const { onChange, ...remainingInputProps } = input;
                return (
                    <Select
                        {...remainingInputProps}
                        id={fieldProps.name}
                        disabled={fieldProps.disabled}
                        variant="outlined"
                        {...SelectProps}
                        sx={{
                            ...paddingChangeStyles,
                            ...(SelectProps?.sx || {}),
                        }}
                        onChange={(event, node) => {
                            if (SelectProps?.onChange) {
                                SelectProps.onChange(event, node);
                            }
                            onChange(event);
                        }}
                    >
                        {options.map(({ value, display, disabled = false }) => (
                            <MenuItem
                                value={value}
                                key={value}
                                disabled={disabled}
                            >
                                {display}
                            </MenuItem>
                        ))}
                    </Select>
                );
            }}
        </FieldWrapper>
    );
};

export interface SelectOption {
    value: string | number;
    display: string | React.ReactNode;
    disabled?: boolean;
}

type FieldWrapperRenderProps = FieldRenderProps<
    string | number | boolean,
    HTMLInputElement | HTMLTextAreaElement
>;
export interface FieldSelectProps
    extends FieldProps<
        string | number | boolean,
        FieldWrapperRenderProps,
        HTMLInputElement
    > {
    options: SelectOption[];
    SelectProps?: Partial<SelectProps>;
}
