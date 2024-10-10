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
  DPS_ANALYSIS_URL,
  BEST_STATS_URL,
} from "../App";
import { AppLanguageTexts } from "../const/languageTexts";

const dpsAnalysisButtonImagePath = "/images/icon_dps_analysis_C.svg";
const gearCompareSimButtonImagePath = "/images/icon_gear_compare_C.svg";
const statWeightsButtonImagePath = "/images/icon_best_stats_C.svg";
const bestPartnerButtonImagePath = "/images/icon_best_partner_C.svg";

export function Home() {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  return (
    <Box
      sx={{
        backgroundColor: AppConfigurations.backgroundOne,
        width: "100vw",
        height: "100vh",
      }}
    >
      <Box display="flex" width="100vw">
        {BasicLeftMenu(LANGUAGE_TEXTS.HOME_PAGE_NAME)}
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
                DPS_ANALYSIS_URL,
                dpsAnalysisButtonImagePath,
                LANGUAGE_TEXTS.DPS_ANALYSIS_PAGE_NAME,
                LANGUAGE_TEXTS.DPS_ANALYSIS_PAGE_NAME,
                LANGUAGE_TEXTS.DPS_ANALYSIS_DESCRIPTION_TEXT,
                AppConfigurations.primary
              )}

              {LogoBox(
                BEST_PARTNER_URL,
                bestPartnerButtonImagePath,
                LANGUAGE_TEXTS.BEST_PARTNER_PAGE_NAME,
                LANGUAGE_TEXTS.BEST_PARTNER_PAGE_NAME,
                LANGUAGE_TEXTS.BEST_PARTNER_DESCRIPTION_TEXT,
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
                LANGUAGE_TEXTS.GEAR_COMPARE_PAGE_NAME,
                LANGUAGE_TEXTS.GEAR_COMPARE_PAGE_NAME,
                LANGUAGE_TEXTS.GEAR_COMPARE_DESCRIPTION_TEXT,
                AppConfigurations.secondary
              )}

              {LogoBox(
                BEST_STATS_URL,
                statWeightsButtonImagePath,
                LANGUAGE_TEXTS.BEST_STAT_PAGE_NAME,
                LANGUAGE_TEXTS.BEST_STAT_PAGE_NAME,
                LANGUAGE_TEXTS.BEST_STATS_DESCRIPTION_TEXT,
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
