import { useState } from "react";
import { Box, styled } from "@mui/material";
import { BEST_PARTNER_INPUT_SAVE_NAME, BEST_PARTNER_URL, BODY_WIDTH } from "../App";
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { StatPowerSummary } from "../components/container/StatSummary";
import { EquipmentInput } from "../types/EquipmentInput";
import { defaultBestPartnerEquipmentInput } from "../const/DefaultSingleEquipmentInput";
import {
  LeftMenuWithLoadout,
} from "../components/container/LeftMenu";
import { ColorConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";
import { SelectionTitle } from "../components/basic/SelectionTitle";
import { BasicBottomMenu } from "../components/container/BottomMenu";
import { EquipmentBoardStyle, InputContainerStyle } from "./Styles";
import { BestPartnerRequestButton } from "../components/basic/BestPartnerRequestButton";

export enum JobRole {
  TANK,
  HEALER,
  MELEE,
  RANGED,
  CASTER,
}

let INPUT_CONTAINER_WIDTH = "40vw";
const BEST_PARTNER_LOADOUT_COUNT = 6;

let StatWeightsInputContainer = styled(Box)`
  ${InputContainerStyle(INPUT_CONTAINER_WIDTH)}
`;

let EquipmentBoard = styled(Box)`
  ${EquipmentBoardStyle}
`;

export function isNotValid(input: EquipmentInput) {
  if (input.equipmentDatas === null) {
    return true;
  }

  return false;
}

export function BestPartner() {
  let mostRecentInputState = localStorage.getItem(BEST_PARTNER_INPUT_SAVE_NAME);
  let mostRecentInput = null;
  console.log(mostRecentInputState);

  if (mostRecentInputState === null) {
    mostRecentInput = defaultBestPartnerEquipmentInput();
  } else {
    mostRecentInput = JSON.parse(mostRecentInputState) as EquipmentInput;
  }

  if (isNotValid(mostRecentInput)) {
    mostRecentInput = defaultBestPartnerEquipmentInput();
  }

  const [totalState, setTotalState] = useState(mostRecentInput);

  return (
    <>
      <Box
        display="flex"
        sx={{ backgroundColor: ColorConfigurations.backgroundOne }}
        width="100vw"
      >
        {LeftMenuWithLoadout(
          BEST_PARTNER_LOADOUT_COUNT,
          BEST_PARTNER_URL,
          totalState,
          setTotalState
        )}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box alignContent={"center"}>
            <StatWeightsInputContainer justifyContent={"center"}>
              {SelectionTitle("1. Input Your Info")}
              <EquipmentBoard>
                {EquipmentSelectionMenu(
                  0,
                  totalState,
                  setTotalState,
                  true,
                  true
                )}
              </EquipmentBoard>
            </StatWeightsInputContainer>

            <StatWeightsInputContainer marginTop={10}>
              {SelectionTitle("2. Specific Player Power")}
              <Box
                display="flex"
                justifyContent="center"
                paddingBottom={"20vh"}
              >
                {StatPowerSummary(totalState.equipmentDatas[0])}
              </Box>
            </StatWeightsInputContainer>

            {BasicBottomMenu(totalState, BestPartnerRequestButton)}
          </Box>
          {Footer()}
        </Box>
      </Box>
    </>
  );
}

export function jobAbbrevToRole(jobAbbrev: string) {
  switch (jobAbbrev) {
    case "PLD":
    case "WAR":
    case "DRK":
    case "GNB":
      return JobRole.TANK;
    case "WHM":
    case "SCH":
    case "AST":
    case "SGE":
      return JobRole.HEALER;
    case "MNK":
    case "DRG":
    case "NIN":
    case "SAM":
    case "RPR":
    case "VPR":
      return JobRole.MELEE;
    case "BRD":
    case "MCH":
    case "DNC":
      return JobRole.RANGED;
    default:
      return JobRole.CASTER;
  }
}
