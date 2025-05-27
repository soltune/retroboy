import {
    MenuItem,
    Select,
    SelectChangeEvent,
    FormControl,
    InputLabel,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import React from "react";

const StyledFormControl = styled(FormControl)(({ theme }) => ({
    minWidth: 200,
    marginBottom: theme.spacing(2),
}));

export interface RomInfo {
    name: string;
    path: string;
}

interface RomSelectorProps {
    selectedRomName: string | null;
    onRomSelect: (romInfo: RomInfo | null) => void;
    roms: RomInfo[];
    disabled?: boolean;
}

export const RomSelector: React.FC<RomSelectorProps> = ({
    selectedRomName,
    onRomSelect,
    roms,
    disabled = false,
}) => {
    const handleChange = (event: SelectChangeEvent<string>) => {
        const selectedName = event.target.value;
        if (selectedName === "") {
            onRomSelect(null);
        } else {
            const rom = roms.find(r => r.name === selectedName);
            if (rom) {
                onRomSelect(rom);
            }
        }
    };

    return (
        <StyledFormControl variant="outlined" fullWidth>
            <InputLabel id="rom-selector-label">Select ROM</InputLabel>
            <Select
                labelId="rom-selector-label"
                id="rom-selector"
                value={selectedRomName || ""}
                onChange={handleChange}
                label="Select ROM"
                variant="outlined"
                disabled={disabled}
            >
                <MenuItem value="">
                    <em>None</em>
                </MenuItem>
                {roms.map(rom => (
                    <MenuItem key={rom.name} value={rom.name}>
                        {rom.name}
                    </MenuItem>
                ))}
            </Select>
        </StyledFormControl>
    );
};
