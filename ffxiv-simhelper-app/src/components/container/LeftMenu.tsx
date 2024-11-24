import { Box, Button, Dialog, DialogActions, DialogContent, DialogContentText, Divider, Drawer, IconButton, Link, styled, Typography } from "@mui/material";
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
	PARTY_COMPOSITION_URL,
} from "../../App";
import { EquipmentInput } from "../../types/EquipmentInput";
import { convertToLinkUrl } from "../../page/home";
import { LanguageInputForm } from "../input/basicform/LanguageInputForm";
import { TextDictionary } from "../../const/languageTexts";
import { useEffect, useState } from "react";
import MenuIcon from "@mui/icons-material/Menu";

export const MENU_WIDTH_VW_XS = 80;
export const MENU_WIDTH_VW_SM = 60;
export const MENU_WIDTH_VW_MD = 40;
export const MENU_WIDTH_VW_LG = 20;
export const MENU_WIDTH_VW_XL = 20;

export const MENU_WIDTH = {
	xs: `${MENU_WIDTH_VW_XS}%`,
	sm: `${MENU_WIDTH_VW_SM}%`,
	md: `${MENU_WIDTH_VW_MD}%`,
	lg: `${MENU_WIDTH_VW_LG}%`,
	xl: `${MENU_WIDTH_VW_XL}%`,
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
	setTotalState: Function,
	LANGUAGE_TEXTS: TextDictionary
) {
	const [isDrawerOpen, setIsDrawerOpen] = useState(false);

	const toggleDrawer = () => {
		setIsDrawerOpen(!isDrawerOpen);
	};

	return (
		<>
			<Box
				position="fixed"
				top={16}
				left={16}
				zIndex={2}
			>
				<IconButton onClick={toggleDrawer} sx={{
					backgroundColor: "white",
					'&:hover': {
						backgroundColor: "gray"
					}
				}}>
					<MenuIcon />
				</IconButton>
			</Box>
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
				open={isDrawerOpen}
				onClose={toggleDrawer}
				anchor="left"
			>
				<LeftMenuTotalBar>
					{DefaultLeftMenuComponents(currentSimulationPage, LANGUAGE_TEXTS)}
					{EquipmentLoadouts(
						loadoutCount,
						loadoutType,
						totalState,
						setTotalState,
						LANGUAGE_TEXTS
					)}
				</LeftMenuTotalBar>
			</Drawer>
		</>
	);
}

export function BasicLeftMenu(currentSimulationPage: string, LANGUAGE_TEXTS: TextDictionary) {
	const [isDrawerOpen, setIsDrawerOpen] = useState(false);

	const toggleDrawer = () => {
		setIsDrawerOpen(!isDrawerOpen);
	};
	return (
		<>
			<Box
				position="fixed"
				top={16}
				left={16}
				zIndex={2}
			>
				<IconButton onClick={toggleDrawer} sx={{
					backgroundColor: "white",
					'&:hover': {
						backgroundColor: 'gray'
					}
				}}>
					<MenuIcon />
				</IconButton>
			</Box>
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
				open={isDrawerOpen}
				onClose={toggleDrawer}
				anchor="left"
			>
				<LeftMenuTotalBar minHeight="100vh">{DefaultLeftMenuComponents(currentSimulationPage, LANGUAGE_TEXTS)}</LeftMenuTotalBar>
			</Drawer>
		</>
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

import HOME_MENU_LOGO_PATH from "/src/assets/images/icon_home_w.svg";
import DPS_ANALYSIS_MENU_LOGO_PATH from "/src/assets/images/icon_dps_analysis_w.svg";
import GEAR_COMPARE_MENU_LOGO_PATH from "/src/assets/images/icon_gear_compare_w.svg";
import BEST_PARTNER_MENU_LOGO_PATH from "/src/assets/images/icon_best_partner_w.svg";
import STAT_WEIGHTS_MENU_LOGO_PATH from "/src/assets/images/icon_best_stats_w.svg";
import LEFT_MENU_LOGO_PATH from "/src/assets/images/left_menu_logo.svg";
import PARTY_COMPOSITION_MENU_LOGO_PATH from "/src/assets/images/icon_party_composition_w.svg";

function DefaultLeftMenuComponents(currentSimulationPage: string, LANGUAGE_TEXTS: TextDictionary) {
	return (
		<>
			<LeftMenuLogo>
				<Box display="flex" justifyContent={"center"} alignItems={"center"}>
					<Box marginRight={0.5}>
						<img src={LEFT_MENU_LOGO_PATH} width={20} height={20} alt="logo" />
					</Box>
					<Typography align="center">
						<b>FFXIV SIMHELPER</b>
					</Typography>

				</Box>
			</LeftMenuLogo>

			<Box width="95%" display="flex" justifyContent={"flex-start"} marginY={2} marginLeft={2}>
				{LanguageInputForm()}
				{ResetAllDataButton(LANGUAGE_TEXTS)}
			</Box>

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

				{NavigationMenu(AppConfigurations.isApp ? "/index.html" : "/", LANGUAGE_TEXTS.HOME_PAGE_NAME, HOME_MENU_LOGO_PATH, currentSimulationPage)}
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
				{NavigationMenu(
					convertToLinkUrl(PARTY_COMPOSITION_URL),
					LANGUAGE_TEXTS.PARTY_COMPOSITION_PAGE_NAME,
					PARTY_COMPOSITION_MENU_LOGO_PATH,
					currentSimulationPage
				)}
			</LeftMenuNavigationBar>
		</>
	);
}


function ResetAllDataButton(
  LANGUAGE_TEXTS: TextDictionary
) {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [buttonClickConfirmed, setButtonClickConfirmed] = useState(false);

  const handleDialogClose = () => {
    setDialogOpen(false); // 다이얼로그 닫기
  };

  const handleConfirm = () => {
    setDialogOpen(false);
	localStorage.clear();

    setButtonClickConfirmed(true); // 다이얼로그 확인 클릭 시 버튼 클릭 확정
	window.location.reload();
  };


  useEffect(() => {
    if (buttonClickConfirmed) {
      setButtonClickConfirmed(false); // 상태 초기화
    }
  }, [buttonClickConfirmed]);

  return (
    <>
      <Button
        sx={{
          backgroundColor: AppConfigurations.primary,
          color: "black",
          borderRadius: 2,
		  marginX: 2,
        }}
        onClick={() => {
          setDialogOpen(true);
        }}
      >
        <Typography
          sx={{ fontWeight: "bold", fontSize: AppConfigurations.body2FontSize }}
        >
          {LANGUAGE_TEXTS.RESET_TEXT}
        </Typography>
      </Button>
      <Dialog
        open={dialogOpen}
        onClose={handleDialogClose}
        aria-labelledby="confirm-dialog-title"
        aria-describedby="confirm-dialog-description"
      >
        <DialogContent>
          <DialogContentText id="confirm-dialog-description">
            {LANGUAGE_TEXTS.RESET_DESCRIPTION_TEXT}
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleConfirm} color="primary" autoFocus>
            {LANGUAGE_TEXTS.CONFIRM_TEXT}
          </Button>
          <Button onClick={handleDialogClose} color="primary">
            {LANGUAGE_TEXTS.CANCEL_TEXT}
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
}


function EquipmentLoadouts(
	loadoutCount: number,
	loadoutType: string,
	totalState: EquipmentInput,
	setTotalState: Function,
	LANGUAGE_TEXTS: TextDictionary
) {
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
							getNumberOfEquipmentSets(loadoutType),
							LANGUAGE_TEXTS
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
