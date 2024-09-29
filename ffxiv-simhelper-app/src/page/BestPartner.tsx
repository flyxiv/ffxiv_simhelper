import { useState } from "react";
import { Box, styled } from "@mui/material";
import {
  BEST_PARTNER_INPUT_SAVE_NAME,
  BODY_WIDTH,
} from "../App";
import { BestPartnerInputMenu } from "../components/input/basicform/EquipmentInputForm";
import { EquipmentInput } from "../types/EquipmentInput";
import { defaultBestPartnerEquipmentInput } from "../const/DefaultSingleEquipmentInput";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";
import { SelectionTitle } from "../components/basic/SelectionTitle";
import { EquipmentBoardStyle, InputContainerStyle } from "./Styles";
import { BestPartnerRequestButton } from "../components/basic/BestPartnerRequestButton";
import {
  AST_EN_NAME,
  BEST_PARTNER_INPUT_INFO_TEXT,
  BEST_PARTNER_PAGE_NAME,
  BRD_EN_NAME,
  DNC_EN_NAME,
  DRG_EN_NAME,
  DRK_EN_NAME,
  GNB_EN_NAME,
  MCH_EN_NAME,
  MNK_EN_NAME,
  NIN_EN_NAME,
  PLD_EN_NAME,
  RPR_EN_NAME,
  SAM_EN_NAME,
  SCH_EN_NAME,
  SGE_EN_NAME,
  VPR_EN_NAME,
  WAR_EN_NAME,
  WHM_EN_NAME,
} from "../const/languageTexts";

export enum JobRole {
  TANK,
  HEALER,
  MELEE,
  RANGED,
  CASTER,
}

let INPUT_CONTAINER_WIDTH = "70%";

let StatWeightsInputContainer = styled(Box)`
  ${InputContainerStyle(INPUT_CONTAINER_WIDTH)}
`;

let EquipmentBoard = styled(Box)`
  ${EquipmentBoardStyle}
`;

export function isNotValid(input: EquipmentInput) {
  if (input.equipmentDatas === null || input.equipmentDatas === undefined) {
    return true;
  }

  for (let i = 0; i < input.equipmentDatas.length; i++) {
    if (input.equipmentDatas[i].partyMemberIlvl === undefined) {
      return true;
    }

    if (input.equipmentDatas[i].usePot === undefined) {
      return true;
    }
  }

  return false;
}

export function BestPartner() {
  let mostRecentInputState = localStorage.getItem(BEST_PARTNER_INPUT_SAVE_NAME);
  let mostRecentInput = null;

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
        sx={{ backgroundColor: AppConfigurations.backgroundOne }}
        width="100vw"
      >
        {BasicLeftMenu(
          BEST_PARTNER_PAGE_NAME,
        )}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box alignContent={"center"} minHeight={"60vh"}>
            <StatWeightsInputContainer justifyContent={"center"}>
              {SelectionTitle(BEST_PARTNER_INPUT_INFO_TEXT)}
              <EquipmentBoard>
                {BestPartnerInputMenu(
                  0,
                  totalState,
                  setTotalState,
                )}
              </EquipmentBoard>
            </StatWeightsInputContainer>

            <Box display="flex" justifyContent={"center"} paddingTop={2}>
              {BestPartnerRequestButton(totalState)}
            </Box>
          </Box>
          {Footer()}
        </Box>
      </Box>
    </>
  );
}

export function jobAbbrevToRole(jobAbbrev: string) {
  switch (jobAbbrev) {
    case PLD_EN_NAME:
    case WAR_EN_NAME:
    case DRK_EN_NAME:
    case GNB_EN_NAME:
      return JobRole.TANK;
    case WHM_EN_NAME:
    case SCH_EN_NAME:
    case AST_EN_NAME:
    case SGE_EN_NAME:
      return JobRole.HEALER;
    case MNK_EN_NAME:
    case DRG_EN_NAME:
    case NIN_EN_NAME:
    case SAM_EN_NAME:
    case RPR_EN_NAME:
    case VPR_EN_NAME:
      return JobRole.MELEE;
    case BRD_EN_NAME:
    case MCH_EN_NAME:
    case DNC_EN_NAME:
      return JobRole.RANGED;
    default:
      return JobRole.CASTER;
  }
}
