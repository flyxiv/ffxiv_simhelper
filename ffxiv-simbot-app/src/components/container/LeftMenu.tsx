import { Box, Divider, Drawer, Link, styled, Typography } from "@mui/material";
import { LeftMenuLogoStyle, LeftMenuNavigationBarStyle, LeftMenuNavigationItemStyle, LeftMenuTotalBarStyle } from "./Styles";
import { ColorConfigurations } from "../../Themes";
import { SingleEquipmentInputSaveState } from "../../types/SingleEquipmentInputSaveState";
import { SingleLoadoutBox } from "./LoadoutBox";

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
    return (
        <Drawer sx={{
            width: "15vw",
            flexShrink: 0,
            '& .MuiDrawer-paper': {
                width: "15vw",
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

                <Box paddingY={4}>
                    <Box paddingX={2}>

                        <Typography variant="h3" align="left" sx={{ fontWeight: "bold", paddingLeft: 2 }}>Saved Gearsets</Typography>
                        <Divider sx={{ borderBottomWidth: 3, marginY: 2, bgcolor: `${ColorConfigurations.backgroundTwo}` }} />
                        {SingleLoadoutBox(1, "quicksim", totalState, setTotalState)}
                    </Box>
                </Box>

            </LeftMenuTotalBar>

        </Drawer >
    )
}

export function BasicLeftMenu() {
    return (
        <Drawer sx={{
            width: "15vw",
            flexShrink: 0,
            '& .MuiDrawer-paper': {
                width: "15vw",
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