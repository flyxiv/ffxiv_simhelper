import { SimLinkIcon } from "../components/image/SimUIIcon";
import "./Home.css";
import { AppHeader } from "../components/image/AppHeader";
import { Footer } from "../components/basic/Footer";
import { Box, Link, useMediaQuery, useTheme } from "@mui/material";
import { AppConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import {
  BEST_PARTNER_URL,
  GEAR_COMPARE_URL,
  DPS_ANALYSIS_URL,
  BEST_STATS_URL,
  BODY_WIDTH,
} from "../App";
import { AppLanguageTexts, TextDictionary } from "../const/languageTexts";

const dpsAnalysisButtonImagePath = "/images/icon_dps_analysis_C.svg";
const gearCompareSimButtonImagePath = "/images/icon_gear_compare_C.svg";
const statWeightsButtonImagePath = "/images/icon_best_stats_C.svg";
const bestPartnerButtonImagePath = "/images/icon_best_partner_C.svg";

export function Home() {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  const theme = useTheme();
  const isXs = useMediaQuery(theme.breakpoints.only('xs'));


  return (
    <Box
      sx={{
        backgroundColor: AppConfigurations.backgroundOne,
        width: "100%",
      }}
    >
      <Box display="flex" width="100%" overflow="auto" minHeight="100%">
        {BasicLeftMenu(LANGUAGE_TEXTS.HOME_PAGE_NAME, LANGUAGE_TEXTS)}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box className="HomeBody">
            {isXs ? OneColumnHomePage(LANGUAGE_TEXTS) : TwoColumnHomePage(LANGUAGE_TEXTS)}
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
        minWidth: "300px",
        height: "10vh",
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

function OneColumnHomePage(LANGUAGE_TEXTS: TextDictionary) {
  return (
    <>
      <Box
        width="100%"
        minWidth="400px"
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
    </>
  )
}

function TwoColumnHomePage(LANGUAGE_TEXTS: TextDictionary) {
  return (
    <>
      <Box
        width="40%"
        minWidth="400px"
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
        minWidth={"400px"}
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
    </>
  )
}