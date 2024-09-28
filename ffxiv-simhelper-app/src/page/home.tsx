import { SimLinkIcon } from "../components/image/SimUIIcon";
import "./Home.css";
import { AppHeader } from "../components/image/AppHeader";
import { Footer } from "../components/basic/Footer";
import { Box, Link } from "@mui/material";
import { AppConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import {
  BEST_PARTNER_URL,
  GEAR_COMPARE_URL,
  QUICKSIM_URL,
  STAT_WEIGHTS_URL,
} from "../App";
import {
  BEST_PARTNER_DESCRIPTION_TEXT,
  BEST_PARTNER_PAGE_NAME,
  GEAR_COMPARE_DESCRIPTION_TEXT,
  GEAR_COMPARE_PAGE_NAME,
  HOME_PAGE_NAME,
  QUICK_SIM_DESCRIPTION_TEXT,
  QUICKSIM_PAGE_NAME,
  STAT_WEIGHTS_DESCRIPTION_TEXT,
  STAT_WEIGHTS_PAGE_NAME,
} from "../const/languageTexts";

const quickSimButtonImagePath = "/images/icon_quick_sim_C.svg";
const gearCompareSimButtonImagePath = "/images/icon_gear_compare_C.svg";
const statWeightsButtonImagePath = "/images/icon_stat_weights_C.svg";
const bestPartnerButtonImagePath = "/images/icon_best_partner_C.svg";

export function Home() {
  return (
    <Box
      sx={{
        backgroundColor: AppConfigurations.backgroundOne,
        width: "100vw",
        height: "100vh",
      }}
    >
      <Box display="flex" width="100vw">
        {BasicLeftMenu(HOME_PAGE_NAME)}
        <Box width="100%">
          {AppHeader()}
          <Box className="HomeBody" height="70vh">
            <Box
              width="40%"
              display="flex"
              alignItems={"center"}
              flexDirection={"column"}
            >
              {LogoBox(
                QUICKSIM_URL,
                quickSimButtonImagePath,
                QUICKSIM_PAGE_NAME,
                QUICKSIM_PAGE_NAME,
                QUICK_SIM_DESCRIPTION_TEXT,
                AppConfigurations.primary
              )}

              {LogoBox(
                BEST_PARTNER_URL,
                bestPartnerButtonImagePath,
                BEST_PARTNER_PAGE_NAME,
                BEST_PARTNER_PAGE_NAME,
                BEST_PARTNER_DESCRIPTION_TEXT,
                AppConfigurations.secondary
              )}
            </Box>
            <Box
              width="40%"
              display="flex"
              alignItems={"center"}
              flexDirection={"column"}
            >
              {LogoBox(
                GEAR_COMPARE_URL,
                gearCompareSimButtonImagePath,
                GEAR_COMPARE_PAGE_NAME,
                GEAR_COMPARE_PAGE_NAME,
                GEAR_COMPARE_DESCRIPTION_TEXT,
                AppConfigurations.secondary
              )}

              {LogoBox(
                STAT_WEIGHTS_URL,
                statWeightsButtonImagePath,
                STAT_WEIGHTS_PAGE_NAME,
                STAT_WEIGHTS_PAGE_NAME,
                STAT_WEIGHTS_DESCRIPTION_TEXT,
                AppConfigurations.primary
              )}
            </Box>
          </Box>
          {Footer()}
        </Box>
      </Box>
    </Box>
  );
}

function LogoBox(
  linkUrl: string,
  buttonImagePath: string,
  altText: string,
  title: string,
  description: string,
  color: string
) {
  return (

    <Box
      component={Link}
      href={convertToLinkUrl(linkUrl)}
      sx={{
        border: `4px solid ${color}`,
        backgroundColor: AppConfigurations.backgroundTwo,
        width: "85%",
        height: "15vh",
      }}
      display="flex"
      alignItems={"center"}
      padding={2}
      marginBottom={4}
      borderRadius={10}
      underline="none"
    >

      {SimLinkIcon(buttonImagePath, altText, title, description, color)}
    </Box >
  );
}

export function convertToLinkUrl(urlName: string): string {
  return AppConfigurations.electron ? `/index.html#/${urlName}` : `/${urlName}`;
}
