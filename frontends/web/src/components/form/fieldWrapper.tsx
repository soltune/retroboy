import { styled } from "@mui/material";
import FormControl from "@mui/material/FormControl";
import FormHelperText from "@mui/material/FormHelperText";
import * as React from "react";
import {
    Field,
    FieldMetaState,
    FieldProps,
    FieldRenderProps,
} from "react-final-form";

export const FieldWrapperLabel = styled("label")`
    font-weight: bold;
    cursor: pointer;
    user-select: none;
    margin-bottom: 2px;
`;

export const hasError = <T,>(meta: FieldMetaState<T>): boolean =>
    (meta.error || (!meta.dirtySinceLastSubmit && meta.submitError)) &&
    meta.touched;

export const FieldWrapper = <FieldValue, T extends HTMLElement>({
    children,
    format,
    initialValue,
    label,
    parse,
    name,
    type,
    validate,
    allowNull,
    formatOnBlur,
    className,
}: FieldWrapperProps<FieldValue, T>): JSX.Element => {
    return (
        <Field
            name={name}
            validate={validate}
            type={type}
            format={format}
            initialValue={initialValue}
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            parse={(value: any, name: string) => {
                const newValue =
                    !value && value !== false && value !== 0 ? null : value;
                return parse ? parse(newValue, name) : newValue;
            }}
            allowNull={allowNull}
            formatOnBlur={formatOnBlur}
        >
            {(renderProps: FieldRenderProps<FieldValue, T>) => {
                const { meta } = renderProps;
                const fieldHasError = hasError(meta);
                return (
                    <FormControl error={fieldHasError} className={className}>
                        {label && (
                            <FieldWrapperLabel htmlFor={name}>
                                {label}
                            </FieldWrapperLabel>
                        )}
                        {children(renderProps)}
                        {fieldHasError && (
                            <FormHelperText>
                                {meta.error || meta.submitError}
                            </FormHelperText>
                        )}
                    </FormControl>
                );
            }}
        </Field>
    );
};

interface FieldWrapperProps<FieldValue, T extends HTMLElement = HTMLElement>
    extends FieldProps<FieldValue, FieldRenderProps<FieldValue, T>, T> {
    children: (renderProps: FieldRenderProps<FieldValue, T>) => React.ReactNode;
    label?: React.ReactNode;
}
