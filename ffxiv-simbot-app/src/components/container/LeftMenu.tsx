import { Box, Divider, Drawer, Link, styled, Typography, useMediaQuery, useTheme } from "@mui/material";
import {
  LeftMenuLogoStyle,
  LeftMenuNavigationBarStyle,
  LeftMenuNavigationItemStyle,
  LeftMenuTotalBarStyle,
} from "./Styles";
import { ColorConfigurations } from "../../Themes";
import { LoadoutBox } from "./LoadoutBox";
import {
  BEST_PARTNER_PAGE_NAME,
  BEST_PARTNER_URL,
  GEAR_COMPARE_PAGE_NAME,
  GEAR_COMPARE_URL,
  HOME_PAGE_NAME,
  QUICKSIM_PAGE_NAME,
  QUICKSIM_URL,
  STAT_WEIGHTS_PAGE_NAME,
  STAT_WEIGHTS_URL,
} from "../../App";
import { EquipmentInput } from "../../types/EquipmentInput";

export const MENU_WIDTH_VW_XS = 50;
export const MENU_WIDTH_VW_SM = 45;
export const MENU_WIDTH_VW_MD = 40;
export const MENU_WIDTH_VW_LG = 30;
export const MENU_WIDTH_VW_XL = 20;

export const MENU_WIDTH = {
  xs: `${MENU_WIDTH_VW_XS}vw`,
  sm: `${MENU_WIDTH_VW_SM}vw`,
  md: `${MENU_WIDTH_VW_MD}vw`,
  lg: `${MENU_WIDTH_VW_LG}vw`,
  xl: `${MENU_WIDTH_VW_XL}vw`,
};


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

export function LeftMenuWithLoadout(
  loadoutCount: number,
  loadoutType: string,
  totalState: EquipmentInput,
  setTotalState: Function
) {
  return (
    <Drawer
      sx={{
        width: MENU_WIDTH,
        flexShrink: 0,
        "& .MuiDrawer-paper": {
          width: MENU_WIDTH,
          backgroundColor: `${ColorConfigurations.backgroundOne}`,
        },
      }}
      variant="permanent"
      anchor="left"
    >
      <LeftMenuTotalBar>
        {DefaultLeftMenuComponents()}
        {EquipmentLoadouts(
          loadoutCount,
          loadoutType,
          totalState,
          setTotalState
        )}
      </LeftMenuTotalBar>
    </Drawer >
  );
}

export function BasicLeftMenu() {
  return (
    <Drawer
      sx={{
        width: MENU_WIDTH,
        flexShrink: 0,
        "& .MuiDrawer-paper": {
          width: MENU_WIDTH,
          boxSizing: "border-box",
          backgroundColor: `${ColorConfigurations.backgroundOne}`,
        },
      }}
      variant="permanent"
      anchor="left"
    >
      <LeftMenuTotalBar>{DefaultLeftMenuComponents()}</LeftMenuTotalBar>
    </Drawer>
  );
}

function NavigationMenu(link: string, text: string, iconPath: string) {
  return (
    <LeftMenuNavigationItem sx={{ paddingY: 1 }}>
      <Link href={link} color="inherit" underline="hover">
        <Box display="flex" alignItems="center">
          <Box marginRight={1}>
            <img src={iconPath} alt={text} height={20} width={20} />
          </Box>
          <Typography variant="body2" align="left">
            {text}
          </Typography>
        </Box>
      </Link>
    </LeftMenuNavigationItem>
  );
}

const HOME_MENU_LOGO_PATH = "/images/icon_home_w.svg";
const QUICKSIM_MENU_LOGO_PATH = "/images/icon_quick_sim_w.svg";
const GEAR_COMPARE_MENU_LOGO_PATH = "/images/icon_gear_compare_w.svg";
const BEST_PARTNER_MENU_LOGO_PATH = "/images/icon_best_partner_w.svg";
const STAT_WEIGHTS_MENU_LOGO_PATH = "/images/icon_stat_weights_w.svg";
const LOGO_PATH = "/images/icon_main_top_kupo_w.svg";

function DefaultLeftMenuComponents() {
  return (
    <>
      <LeftMenuLogo>
        <Box display="flex" justifyContent={"center"}>
          <Box marginRight={0.5}>
            <img src={LOGO_PATH} width={20} height={20} alt="logo" />
          </Box>
          <Typography align="center">
            <b>FFXIV SIMULATION BOT</b>{" "}
          </Typography>
        </Box>
      </LeftMenuLogo>
      <LeftMenuNavigationBar paddingY={4} paddingLeft={2}>
        <Typography variant="h3" align="left" sx={{ fontWeight: "bold" }}>
          Navigate
        </Typography>

        <Divider
          sx={{
            borderBottomWidth: 3,
            marginY: 2,
            bgcolor: `${ColorConfigurations.backgroundTwo}`,
          }}
        />

        {NavigationMenu("/", HOME_PAGE_NAME, HOME_MENU_LOGO_PATH)}
        {NavigationMenu(
          `/${QUICKSIM_URL}`,
          QUICKSIM_PAGE_NAME,
          QUICKSIM_MENU_LOGO_PATH
        )}
        {NavigationMenu(
          `/${GEAR_COMPARE_URL}`,
          GEAR_COMPARE_PAGE_NAME,
          GEAR_COMPARE_MENU_LOGO_PATH
        )}
        {NavigationMenu(
          `/${BEST_PARTNER_URL}`,
          BEST_PARTNER_PAGE_NAME,
          BEST_PARTNER_MENU_LOGO_PATH
        )}
        {NavigationMenu(
          `/${STAT_WEIGHTS_URL}`,
          STAT_WEIGHTS_PAGE_NAME,
          STAT_WEIGHTS_MENU_LOGO_PATH
        )}
      </LeftMenuNavigationBar>
    </>
  );
}

function EquipmentLoadouts(
  loadoutCount: number,
  loadoutType: string,
  totalState: EquipmentInput,
  setTotalState: Function
) {
  return (
    <>
      <Box paddingX={2}>
        <Typography
          variant="h3"
          align="left"
          sx={{ fontWeight: "bold", paddingLeft: 2 }}
        >
          Saved Gearsets
        </Typography>
        <Divider
          sx={{
            borderBottomWidth: 3,
            marginY: 2,
            bgcolor: `${ColorConfigurations.backgroundTwo}`,
          }}
        />
      </Box>
      {[...Array(loadoutCount).keys()].map((loadoutId) => {
        return (
          <Box paddingY={4} marginX={4}>
            {LoadoutBox(
              loadoutId + 1,
              loadoutType,
              totalState,
              setTotalState,
              getNumberOfEquipmentSets(loadoutType)
            )}
          </Box>
        );
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
