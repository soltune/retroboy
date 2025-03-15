import MoreVertIcon from "@mui/icons-material/MoreVert";
import Button, { ButtonProps } from "@mui/material/Button";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import { styled } from "@mui/material/styles";
import { useState } from "react";

const PaddedButton = styled(Button)`
    padding: 5px 12px;
    min-width: 16px;
`;

export const MenuButton = ({
    menuItems,
    ...buttonProps
}: MenuButtonProps): JSX.Element => {
    const [anchorEl, setAnchorEl] = useState(null as HTMLElement | null);

    const handleClick = (event: React.MouseEvent<HTMLButtonElement>): void => {
        setAnchorEl(event.currentTarget);
    };

    const handleClose = (): void => {
        setAnchorEl(null);
    };

    const asMenuItemElement = (
        { action, display, key }: MenuItemInfo,
        index: number,
    ): JSX.Element => (
        <MenuItem
            key={key ?? index}
            onClick={() => {
                action();
                handleClose();
            }}
        >
            {display}
        </MenuItem>
    );

    return (
        <>
            <PaddedButton onClick={handleClick} type="button" {...buttonProps}>
                <MoreVertIcon fontSize="small" />
            </PaddedButton>
            <Menu
                anchorEl={anchorEl}
                keepMounted
                open={!!anchorEl}
                onClose={handleClose}
                anchorOrigin={{ vertical: "bottom", horizontal: "left" }}
            >
                {menuItems.map(asMenuItemElement)}
            </Menu>
        </>
    );
};

export interface MenuItemInfo {
    display: React.ReactNode;
    action: () => void;
    key?: string;
}

interface MenuButtonProps extends ButtonProps {
    menuItems: MenuItemInfo[];
}
