import { Box, Divider, Drawer, Link, styled, Typography } from "@mui/material";
import {
  LeftMenuLogoStyle,
  LeftMenuNavigationBarStyle,
  LeftMenuNavigationItemStyle,
  LeftMenuTotalBarStyle,
} from "./Styles";
import { AppConfigurations } from "../../Themes";
import { LoadoutBox } from "./LoadoutBox";
import {
  BEST_PARTNER_URL,
  GEAR_COMPARE_URL,
  DPS_ANALYSIS_URL,
  BEST_STATS_URL,
} from "../../App";
import { EquipmentInput } from "../../types/EquipmentInput";
import { convertToLinkUrl } from "../../page/home";
import { AppLanguageTexts } from "../../const/languageTexts";
import { LanguageInputForm } from "../input/basicform/LanguageInputForm";

export const MENU_WIDTH_VW_XS = 35;
export const MENU_WIDTH_VW_SM = 30;
export const MENU_WIDTH_VW_MD = 25;
export const MENU_WIDTH_VW_LG = 20;
export const MENU_WIDTH_VW_XL = 15;

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
  currentSimulationPage: string,
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
          backgroundColor: `${AppConfigurations.backgroundOne}`,
          scrollbarWidth: "none",
        },
      }}
      variant="permanent"
      anchor="left"
    >
      <LeftMenuTotalBar>
        {DefaultLeftMenuComponents(currentSimulationPage)}
        {EquipmentLoadouts(
          loadoutCount,
          loadoutType,
          totalState,
          setTotalState
        )}
      </LeftMenuTotalBar>
    </Drawer>
  );
}

export function BasicLeftMenu(currentSimulationPage: string) {
  return (
    <Drawer
      sx={{
        width: MENU_WIDTH,
        flexShrink: 0,
        "& .MuiDrawer-paper": {
          width: MENU_WIDTH,
          boxSizing: "border-box",
          backgroundColor: `${AppConfigurations.backgroundOne}`,
          scrollbarWidth: "none",
        },
      }}
      variant="permanent"
      anchor="left"
    >
      <LeftMenuTotalBar>{DefaultLeftMenuComponents(currentSimulationPage)}</LeftMenuTotalBar>
    </Drawer>
  );
}

function NavigationMenu(link: string, text: string, iconPath: string, currentSimulationPage: string) {
  return (
    <LeftMenuNavigationItem sx={{ paddingY: 1 }}>
      <Link href={link} color="inherit" underline="hover">
        <Box display="flex" alignItems="center" sx={{ backgroundColor: currentSimulationPage === text ? AppConfigurations.backgroundFour : AppConfigurations.backgroundThree, width: "80%" }}>
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
const DPS_ANALYSIS_MENU_LOGO_PATH = "/images/icon_dps_analysis_w.svg";
const GEAR_COMPARE_MENU_LOGO_PATH = "/images/icon_gear_compare_w.svg";
const BEST_PARTNER_MENU_LOGO_PATH = "/images/icon_best_partner_w.svg";
const STAT_WEIGHTS_MENU_LOGO_PATH = "/images/icon_best_stats_w.svg";
const LOGO_PATH = "/images/left_menu_logo.svg";

function DefaultLeftMenuComponents(currentSimulationPage: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  return (
    <>
      <LeftMenuLogo>
        <Box display="flex" justifyContent={"center"}>
          <Box marginRight={0.5}>
            <img src={LOGO_PATH} width={20} height={20} alt="logo" />
          </Box>
          <Typography align="center">
            <b>FFXIV SIMHELPER</b>
          </Typography>
        </Box>
      </LeftMenuLogo>

      {LanguageInputForm()}

      <LeftMenuNavigationBar paddingY={4} paddingLeft={2}>
        <Typography variant="h3" align="left" sx={{ fontWeight: "bold" }}>
          {LANGUAGE_TEXTS.NAVIGATE_TEXT}
        </Typography>

        <Divider
          sx={{
            borderBottomWidth: 3,
            marginY: 2,
            bgcolor: `${AppConfigurations.backgroundTwo}`,
          }}
        />

        {NavigationMenu("/index.html", LANGUAGE_TEXTS.HOME_PAGE_NAME, HOME_MENU_LOGO_PATH, currentSimulationPage)}
        {NavigationMenu(
          convertToLinkUrl(DPS_ANALYSIS_URL),
          LANGUAGE_TEXTS.DPS_ANALYSIS_PAGE_NAME,
          DPS_ANALYSIS_MENU_LOGO_PATH,
          currentSimulationPage
        )}
        {NavigationMenu(
          convertToLinkUrl(GEAR_COMPARE_URL),
          LANGUAGE_TEXTS.GEAR_COMPARE_PAGE_NAME,
          GEAR_COMPARE_MENU_LOGO_PATH,
          currentSimulationPage
        )}
        {NavigationMenu(
          convertToLinkUrl(BEST_PARTNER_URL),
          LANGUAGE_TEXTS.BEST_PARTNER_PAGE_NAME,
          BEST_PARTNER_MENU_LOGO_PATH,
          currentSimulationPage

        )}
        {NavigationMenu(
          convertToLinkUrl(BEST_STATS_URL),
          LANGUAGE_TEXTS.BEST_STAT_PAGE_NAME,
          STAT_WEIGHTS_MENU_LOGO_PATH,
          currentSimulationPage
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
  let LANGUAGE_TEXTS = AppLanguageTexts();

  return (
    <>
      <Box paddingX={2}>
        <Typography
          variant="h3"
          align="left"
          sx={{ fontWeight: "bold", paddingLeft: 2 }}
        >
          {LANGUAGE_TEXTS.SAVED_GEARSETS_TEXT}
        </Typography>
        <Divider
          sx={{
            borderBottomWidth: 3,
            marginY: 2,
            bgcolor: `${AppConfigurations.backgroundTwo}`,
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
