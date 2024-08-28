import { Box, Divider, Drawer, Link, styled, Typography } from "@mui/material";
import { LeftMenuLogoStyle, LeftMenuNavigationBarStyle, LeftMenuNavigationItemStyle, LeftMenuTotalBarStyle } from "./Styles";
import { ColorConfigurations } from "../../Themes";
import { SingleEquipmentInputSaveState } from "../../types/SingleEquipmentInputSaveState";
import { SingleLoadoutBox } from "./LoadoutBox";

const QUICKSIM_LOADOUT_COUNT = 3;
export const MENU_WIDTH_VW = 15;

let LeftMenuLogo = styled(Box)`
    ${LeftMenuLogoStyle}
`;

let LeftMenuTotalBar = styled(Box)`
    ${LeftMenuTotalBarStyle}
`;

let LeftMenuNavigationBar = styled(Box)`
    ${LeftMenuNavigationBarStyle}
`;

let LeftMenuNavigationItem = styled(Box)`
    ${LeftMenuNavigationItemStyle}
`;

export function QuicksimLeftMenu(totalState: SingleEquipmentInputSaveState, setTotalState: Function) {
    let menu_width_string = `${MENU_WIDTH_VW}vw`;
    return (
        <Drawer sx={{
            width: menu_width_string,
            flexShrink: 0,
            '& .MuiDrawer-paper': {
                width: menu_width_string,
                backgroundColor: `${ColorConfigurations.backgroundOne}`
            },
        }} variant="permanent" anchor="left">
            <LeftMenuTotalBar>
                <LeftMenuLogo>
                    <Typography align="center"><b>FFXIV SIMULATION BOT</b> </Typography>
                </LeftMenuLogo>
                <LeftMenuNavigationBar paddingY={4} paddingLeft={2}>
                    <Typography variant="h3" align="left" sx={{ fontWeight: "bold" }}>Navigate</Typography>

                    <Divider sx={{ borderBottomWidth: 3, marginY: 2, bgcolor: `${ColorConfigurations.backgroundTwo}` }} />

                    {NavigationMenu("/", "Home")}
                    {NavigationMenu("/quicksim", "Quick Sim")}
                    {NavigationMenu("/quicksim", "Gear Compare")}
                    {NavigationMenu("/quicksim", "Best Partner")}
                    {NavigationMenu("/quicksim", "Stat Weights")}
                </LeftMenuNavigationBar>

                <Box paddingX={2}>
                    <Typography variant="h3" align="left" sx={{ fontWeight: "bold", paddingLeft: 2 }}>Saved Gearsets</Typography>
                    <Divider sx={{ borderBottomWidth: 3, marginY: 2, bgcolor: `${ColorConfigurations.backgroundTwo}` }} />
                </Box>

                {[...Array(QUICKSIM_LOADOUT_COUNT).keys()].map((loadoutId) => {
                    return (<Box paddingY={4} marginX={4}>
                        {SingleLoadoutBox(loadoutId + 1, "quicksim", totalState, setTotalState)}
                    </Box>)

                })}

            </LeftMenuTotalBar>

        </Drawer >
    )
}

export function BasicLeftMenu() {
    let menu_width_string = `${MENU_WIDTH_VW}vw`;
    return (
        <Drawer sx={{
            width: menu_width_string,
            flexShrink: 0,
            '& .MuiDrawer-paper': {
                width: menu_width_string,
                boxSizing: 'border-box',
                backgroundColor: `${ColorConfigurations.backgroundOne}`
            },
        }} variant="permanent" anchor="left">
            <LeftMenuTotalBar>
                <LeftMenuLogo>
                    <Typography align="center"><b>FFXIV SIMULATION BOT</b> </Typography>
                </LeftMenuLogo>
                <LeftMenuNavigationBar paddingY={4} paddingLeft={2}>
                    <Typography variant="h3" align="left" sx={{ fontWeight: "bold" }}>Navigate</Typography>

                    <Divider sx={{ borderBottomWidth: 3, marginY: 2, bgcolor: `${ColorConfigurations.backgroundTwo}` }} />

                    {NavigationMenu("/", "Home")}
                    {NavigationMenu("/quicksim", "Quick Sim")}
                    {NavigationMenu("/quicksim", "Gear Compare")}
                    {NavigationMenu("/quicksim", "Best Partner")}
                    {NavigationMenu("/quicksim", "Stat Weights")}
                </LeftMenuNavigationBar>
            </LeftMenuTotalBar>
        </Drawer >
    )
}

function NavigationMenu(link: string, text: string) {
    return (
        <LeftMenuNavigationItem sx={{ paddingY: 1 }}>
            <Link href={link} color="inherit" underline="hover"><Typography variant="body2" align="left">{text}</Typography></Link>
        </LeftMenuNavigationItem >
    )
}