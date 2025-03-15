import { styled } from "@mui/material/styles";

export const paddingChangeStyles = {
    "& .MuiInputBase-input": {
        padding: "8px",
    },
    "& .MuiInputBase-input[type=number]": {
        textAlign: "right",
    },
};

export const FieldWrapperLabel = styled("label")`
    font-weight: bold;
    cursor: pointer;
    user-select: none;
    margin-bottom: 2px;
`;
