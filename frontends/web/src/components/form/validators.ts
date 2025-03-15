import { FieldValidator } from "final-form";

export const composeValidators =
    <FieldValue>(...validators: FieldValidator<FieldValue>[]) =>
    // eslint-disable-next-line @typescript-eslint/ban-types
    (value: FieldValue, allValues: object): string | undefined =>
        validators.reduce(
            (error, validator) => error || validator(value, allValues),
            undefined as string | undefined,
        );

export const required = (value: unknown): string | undefined =>
    value !== undefined && value !== null ? undefined : "Required";

export const maxLength =
    (maxLength: number) =>
    (value: string): string | undefined => {
        return !value || value.length <= maxLength
            ? undefined
            : `Length cannot be greater than ${maxLength}`;
    };
