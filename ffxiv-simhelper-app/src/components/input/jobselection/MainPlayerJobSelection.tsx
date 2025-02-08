import { Box, Divider, MenuItem, Select, SelectChangeEvent, Typography } from "@mui/material";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { JobMenuItem } from "../../items/JobMenuItem";
import {
  updateAllPlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { AppConfigurations } from "../../../Themes";
import {
  EquipmentInput,
  SingleEquipmentInputSaveState,
} from "../../../types/EquipmentInput";
import { getRoleByIdAndMainCharacterJob } from "./PartyMemberJobSelection";
import { calculateHasteBuff, DEFAULT_GCD } from "../../../types/ffxivdatabase/StatCalculator";
import { mapJobAbbrevToJobBisEquipments } from "../../../const/StatValue";
import { AST_EN_NAME, BLM_EN_NAME, BRD_EN_NAME, DNC_EN_NAME, DRG_EN_NAME, DRK_EN_NAME, GNB_EN_NAME, MCH_EN_NAME, MNK_EN_NAME, NIN_EN_NAME, PCT_EN_NAME, PLD_EN_NAME, RDM_EN_NAME, RPR_EN_NAME, SAM_EN_NAME, SCH_EN_NAME, SGE_EN_NAME, SMN_EN_NAME, TextDictionary, VPR_EN_NAME, WAR_EN_NAME, WHM_EN_NAME } from "../../../const/languageTexts";
import { TopMenuInput } from "../basicform/EquipmentInputForm";
import { ITEM_TOP_MENU_MIN_HEIGHT } from "../../items/Styles";

let ALIGN = "left";


function resetInvalidPartnersForNewJob(data: SingleEquipmentInputSaveState) {
  let partyMemberJobAbbrevs = data.partyMemberJobAbbrevs;
  let mainPlayerJobAbbrev = data.mainPlayerJobAbbrev;

  let newPartyMemberJobAbbrevs = [];
  let usedIds: Array<number> = [];

  for (let partyMemberId = 1; partyMemberId <= 8; partyMemberId++) {
    if (partyMemberId > partyMemberJobAbbrevs.length) {
      break;
    }

    newPartyMemberJobAbbrevs.push("Empty");

    let possibleJobsForSlot = getRoleByIdAndMainCharacterJob(
      partyMemberId,
      mainPlayerJobAbbrev,
      partyMemberJobAbbrevs
    );

    for (
      let partyMemberIdSearch = 1;
      partyMemberIdSearch <= partyMemberJobAbbrevs.length;
      partyMemberIdSearch++
    ) {
      if (
        possibleJobsForSlot.includes(
          partyMemberJobAbbrevs[partyMemberIdSearch - 1]
        ) &&
        !usedIds.includes(partyMemberIdSearch)
      ) {
        newPartyMemberJobAbbrevs[partyMemberId - 1] =
          partyMemberJobAbbrevs[partyMemberIdSearch - 1];
        usedIds.push(partyMemberIdSearch);
        break;
      }
    }
  }

  let newPartyMemberIds = [];
  for (
    let partyMemberId = 1;
    partyMemberId <= newPartyMemberJobAbbrevs.length;
    partyMemberId++
  ) {
    if (newPartyMemberJobAbbrevs[partyMemberId - 1] !== "Empty") {
      newPartyMemberIds.push(partyMemberId);
    }
  }

  data.partyMemberJobAbbrevs = newPartyMemberJobAbbrevs;
  data.partyMemberIds = newPartyMemberIds;
}

export function MainPlayerJobSelection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function,
  LANGUAGE_TEXTS: TextDictionary
) {
  const handleJobChange = (event: SelectChangeEvent<string>) => {
    let newJobAbbrev = event.target.value;

    let newTotalState = {...totalEquipmentState}; 
    let bisForNewJob = mapJobAbbrevToJobBisEquipments(newJobAbbrev);

    if (bisForNewJob === undefined) {
      return;
    }

    newTotalState.equipmentDatas.forEach((data) => {
      data.mainPlayerJobAbbrev = newJobAbbrev;
      data.itemSet = [...bisForNewJob.itemSet];
      data.gearSetMaterias = JSON.parse(JSON.stringify(bisForNewJob.gearSetMaterias));
      data.foodId = bisForNewJob.foodId;
      resetInvalidPartnersForNewJob(data);
    })


    updateAllPlayerPower(newTotalState, setTotalState);
  };

  let key = `Job-${id}`;

  return (
    <CustomFormControl fullWidth>
      <Select
        labelId={key}
        id={key}
        value={totalEquipmentState.equipmentDatas[id].mainPlayerJobAbbrev}
        label={key}
        onChange={handleJobChange}
        MenuProps={{
          PaperProps: {
            sx: {
              backgroundColor: AppConfigurations.backgroundThree,
            },
          },
        }}
      >
        {JobMenuItem(PLD_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(WAR_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(DRK_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(GNB_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        <Divider />
        {JobMenuItem(WHM_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(AST_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(SCH_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(SGE_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        <Divider />
        {JobMenuItem(DRG_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(MNK_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(NIN_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(SAM_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(RPR_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(VPR_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        <Divider />
        {JobMenuItem(BRD_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(MCH_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(DNC_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        <Divider />
        {JobMenuItem(SMN_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(BLM_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(RDM_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(PCT_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
      </Select>
    </CustomFormControl>
  );
}

export function MainPlayerJobSelectionOnlyBuffJobs(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function,
  LANGUAGE_TEXTS: TextDictionary
) {
  const handleJobChange = (event: SelectChangeEvent<string>) => {
    let newJobAbbrev = event.target.value;
    let newTotalEquipmentState = { ...totalEquipmentState };

    newTotalEquipmentState.equipmentDatas[id].mainPlayerJobAbbrev = newJobAbbrev;
    newTotalEquipmentState.equipmentDatas[id].power.speedMultiplier = 1;
    newTotalEquipmentState.equipmentDatas[id].power.gcd = Math.floor(DEFAULT_GCD * calculateHasteBuff(newJobAbbrev) / 100) / 100

    setTotalState(newTotalEquipmentState);
  };

  let key = `Job-${id}`;

  return (
    <CustomFormControl fullWidth>
      <Select
        labelId={key}
        id={key}
        value={totalEquipmentState.equipmentDatas[id].mainPlayerJobAbbrev}
        label={key}
        onChange={handleJobChange}
        MenuProps={{
          PaperProps: {
            sx: {
              backgroundColor: AppConfigurations.backgroundThree,
            },
          },
        }}
      >
        {JobMenuItem(AST_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(SCH_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        <Divider />
        {JobMenuItem(DRG_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(MNK_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(NIN_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(RPR_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        <Divider />
        {JobMenuItem(BRD_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(DNC_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        <Divider />
        {JobMenuItem(SMN_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(RDM_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
        {JobMenuItem(PCT_EN_NAME, ALIGN, LANGUAGE_TEXTS)}
      </Select>
    </CustomFormControl>
  );
}


const GCD_OPTION_COUNT = 30;

function getGcdOptions(jobAbbrev: string) {
  let maxGcd = Math.floor(DEFAULT_GCD * calculateHasteBuff(jobAbbrev) / 100)
  let gcdOptions = [];

  for (let i = 0; i < GCD_OPTION_COUNT; i++) {
    let gcd = (maxGcd - i);
    gcdOptions.push(gcd);
  }

  return gcdOptions;
}


export function MainPlayerGcdSelection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function,
  speedLabelText: string,
) {
  let key = `Job-${id}`;
  let jobAbbrev = totalEquipmentState.equipmentDatas[id].mainPlayerJobAbbrev;
  let gcdOptions = getGcdOptions(jobAbbrev);

  return (
    <CustomFormControl fullWidth>
      <TopMenuInput
        select
        id={key}
        value={(totalEquipmentState.equipmentDatas[id].power.gcd).toFixed(2)}
        key={key}
        label={speedLabelText}
        onChange={(e: React.ChangeEvent<HTMLInputElement>): void => {
          {
            let newGcd = parseFloat(e.target.value);
            let newTotalState = { ...totalEquipmentState };
            let maxGcd = Math.floor(DEFAULT_GCD * (calculateHasteBuff(jobAbbrev) / 100)) / 100;
            let newSpeedMultiplier = Math.floor(maxGcd / newGcd * 1000) / 1000;

            newTotalState.equipmentDatas[id].power.gcd = newGcd;
            newTotalState.equipmentDatas[id].power.speedMultiplier = newSpeedMultiplier;
            setTotalState(newTotalState);
          }
        }}
        SelectProps={{
          MenuProps: {
            PaperProps: {
              sx: {
                backgroundColor: AppConfigurations.backgroundThree,
              },
            },
          },
        }}
      >
        {
          gcdOptions.map((gcd) => {
            return (
              <MenuItem value={(gcd / 100).toFixed(2)}>
                <Box display="flex" sx={{ height: ITEM_TOP_MENU_MIN_HEIGHT }} alignItems={"center"} justifyContent={"flex-end"}>
                  <Typography sx={{ fontSize: AppConfigurations.body2FontSize, color: "white" }}>
                    {`${(gcd / 100).toFixed(2)}`}
                  </Typography>
                </Box>
              </MenuItem>
            )
          })
        }
      </TopMenuInput>
    </CustomFormControl >
  );
}
