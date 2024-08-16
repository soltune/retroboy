import { Box, Button, Modal, Typography, styled } from "@mui/material";

const HelpContent = styled(Box)`
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    padding: 32px;
    width: 512px;
    background: ${({ theme }) => theme.palette.background.paper};
    border: 1px solid ${({ theme }) => theme.palette.text.primary};
    box-shadow: 64px;
`;

const HelpModal = ({ showHelpText, onClose }: HelpModalProps): JSX.Element => (
    <Modal open={showHelpText} onClose={onClose}>
        <HelpContent>
            <Typography variant="h6">Welcome to Retro Boy!</Typography>
            <p>
                Retro Boy is a simple Gameboy Emulator written in Rust and
                compiled to WebAssembly.
            </p>
            <p>
                To use, simply click "Load ROM" to load your game ROM. At the
                moment, only .gb files are supported. If you have a DMG boot ROM
                available, you can also load that (no other boot ROM types are
                supported as of now). For the boot ROM, only .bin files are
                supported.
            </p>
            <p>When you're done, click "Play".</p>
            <Button variant="contained" onClick={onClose}>
                Got it!
            </Button>
        </HelpContent>
    </Modal>
);

interface HelpModalProps {
    showHelpText: boolean;
    onClose: () => void;
}

export default HelpModal;
