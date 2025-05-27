import { RomInfo } from "./components/romSelector";
import { getRomPath } from "./config/paths";

// Store relative paths without base path
const romDefinitions = [
    {
        name: "CPU instruction tests",
        path: "cpu_instrs.gb",
    },
];

// Generate full paths at runtime
export const availableRoms: RomInfo[] = romDefinitions.map(rom => ({
    ...rom,
    path: getRomPath(rom.path),
}));
