import { Box, Divider, Drawer, Link, styled, Typography } from "@mui/material";
import { LeftMenuLogoStyle, LeftMenuNavigationBarStyle, LeftMenuNavigationItemStyle, LeftMenuTotalBarStyle } from "./Styles";
import { ColorConfigurations } from "../../Themes";
import { LoadoutBox } from "./LoadoutBox";
import { BEST_PARTNER_PAGE_NAME, BEST_PARTNER_URL, GEAR_COMPARE_PAGE_NAME, GEAR_COMPARE_URL, HOME_PAGE_NAME, QUICKSIM_PAGE_NAME, QUICKSIM_URL, STAT_WEIGHTS_PAGE_NAME, STAT_WEIGHTS_URL } from "../../App";
import { EquipmentInput } from "src/types/EquipmentInput";

export const MENU_WIDTH_VW = 15;
const MENU_WIDTH_VW_STRING = `${MENU_WIDTH_VW}vw`;

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

export function LeftMenuWithLoadout(loadoutCount: number, loadoutType: string, totalState: EquipmentInput, setTotalState: Function) {
    return (
        <Drawer sx={{
            width: MENU_WIDTH_VW_STRING,
            flexShrink: 0,
            '& .MuiDrawer-paper': {
                width: MENU_WIDTH_VW_STRING,
                backgroundColor: `${ColorConfigurations.backgroundOne}`
            },
        }} variant="permanent" anchor="left">
            <LeftMenuTotalBar>
                {DefaultLeftMenuComponents()}
                {EquipmentLoadouts(loadoutCount, loadoutType, totalState, setTotalState)}
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
                {DefaultLeftMenuComponents()}
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

function DefaultLeftMenuComponents() {
    return (
        <>
            <LeftMenuLogo>
                <Typography align="center"><b>FFXIV SIMULATION BOT</b> </Typography>
            </LeftMenuLogo>
            <LeftMenuNavigationBar paddingY={4} paddingLeft={2}>
                <Typography variant="h3" align="left" sx={{ fontWeight: "bold" }}>Navigate</Typography>

                <Divider sx={{ borderBottomWidth: 3, marginY: 2, bgcolor: `${ColorConfigurations.backgroundTwo}` }} />

                {NavigationMenu("/", HOME_PAGE_NAME)}
                {NavigationMenu(`/${QUICKSIM_URL}`, QUICKSIM_PAGE_NAME)}
                {NavigationMenu(`/${GEAR_COMPARE_URL}`, GEAR_COMPARE_PAGE_NAME)}
                {NavigationMenu(`/${BEST_PARTNER_URL}`, BEST_PARTNER_PAGE_NAME)}
                {NavigationMenu(`/${STAT_WEIGHTS_URL}`, STAT_WEIGHTS_PAGE_NAME)}
            </LeftMenuNavigationBar>
        </>
    )
}


function EquipmentLoadouts(loadoutCount: number, loadoutType: string, totalState: EquipmentInput, setTotalState: Function) {
    return (
        <>
            <Box paddingX={2}>
                <Typography variant="h3" align="left" sx={{ fontWeight: "bold", paddingLeft: 2 }}>Saved Gearsets</Typography>
                <Divider sx={{ borderBottomWidth: 3, marginY: 2, bgcolor: `${ColorConfigurations.backgroundTwo}` }} />
            </Box>
            {[...Array(loadoutCount).keys()].map((loadoutId) => {
                return (<Box paddingY={4} marginX={4}>
                    {LoadoutBox(loadoutId + 1, loadoutType, totalState, setTotalState, getNumberOfEquipmentSets(loadoutType))}
                </Box>)

            })}
        </>
    );
}

function getNumberOfEquipmentSets(loadoutType: string) {
    if (loadoutType === GEAR_COMPARE_URL) {
        return 2;
    } else {
        return 1;
    }
}       
