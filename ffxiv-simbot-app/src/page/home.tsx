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

const quickSimButtonImagePath = "/images/icon_quick_sim_C.svg";
const QUICK_SIM_TITLE = "Quick Sim";
const QUICK_SIM_DESCRIPTION =
  "Quickly get detailed statistics of your party's DPS expectation.";

const gearCompareSimButtonImagePath = "/images/icon_gear_compare_C.svg";
const GEAR_COMPARE_TITLE = "Gear Compare";
const GEAR_COMPARE_DESCRIPTION =
  "Compare which of the two gearsets is better for your damage.";

const statWeightsButtonImagePath = "/images/icon_stat_weights_C.svg";
const STAT_WEIGHTS_TITLE = "Stat Weights";
const STAT_WEIGHTS_DESCRIPTION =
  "Calculate which main/sub stats are more valuable to you.";

const bestPartnerButtonImagePath = "/images/icon_best_partner_C.svg";
const BEST_PARTNER_TITLE = "Best Partner";
const BEST_PARTNER_DESCRIPTION =
  "Find out the teammates that will contribute the most RDPS for you(buff jobs only).";

export function Home() {
  return (
    <Box sx={{ backgroundColor: AppConfigurations.backgroundOne, width: "100vw", height: "100vh" }}>
      <Box display="flex" width="100vw">
        {BasicLeftMenu()}
        <Box width="100%">
          {AppHeader()}
          <Box className="HomeBody">
            <Box
              width="40%"
              display="flex"
              alignItems={"center"}
              flexDirection={"column"}
            >
              {LogoBox(
                QUICKSIM_URL,
                quickSimButtonImagePath,
                QUICK_SIM_TITLE,
                QUICK_SIM_TITLE,
                QUICK_SIM_DESCRIPTION,
                AppConfigurations.primary
              )}

              {LogoBox(
                BEST_PARTNER_URL,
                bestPartnerButtonImagePath,
                BEST_PARTNER_TITLE,
                BEST_PARTNER_TITLE,
                BEST_PARTNER_DESCRIPTION,
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
                GEAR_COMPARE_TITLE,
                GEAR_COMPARE_TITLE,
                GEAR_COMPARE_DESCRIPTION,
                AppConfigurations.secondary
              )}

              {LogoBox(
                STAT_WEIGHTS_URL,
                statWeightsButtonImagePath,
                STAT_WEIGHTS_TITLE,
                STAT_WEIGHTS_TITLE,
                STAT_WEIGHTS_DESCRIPTION,
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
      sx={{
        border: `4px solid ${color}`,
        backgroundColor: AppConfigurations.backgroundTwo,
        width: "80%",
      }}
      alignItems={"center"}
      padding={2}
      marginBottom={4}
    >
      <Link href={`/${linkUrl}`} underline="none">
        {SimLinkIcon(buttonImagePath, altText, title, description, color)}
      </Link>
    </Box>
  );
}
