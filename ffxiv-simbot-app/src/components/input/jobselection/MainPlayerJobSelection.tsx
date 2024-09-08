import { Divider, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { JobMenuItem } from "../../items/JobMenuItem";
import {
  slotIndexToSlotName,
  updateAllPlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { AppConfigurations } from "../../../Themes";
import { EquipmentInput, SingleEquipmentInputSaveState } from "../../../types/EquipmentInput";
import { EMPTY_EQUIPMENT_ID, EQUIPMENT_DATABASE_BY_KEYS, toEquipmentKeyString } from "../../../types/ffxivdatabase/Equipment";
import { getRoleByIdAndMainCharacterJob } from "./PartyMemberJobSelection";
import { AST_EN_NAME, BLM_EN_NAME, BRD_EN_NAME, DNC_EN_NAME, DRG_EN_NAME, DRK_EN_NAME, GNB_EN_NAME, MCH_EN_NAME, MNK_EN_NAME, NIN_EN_NAME, PCT_EN_NAME, PLD_EN_NAME, RDM_EN_NAME, RPR_EN_NAME, SAM_EN_NAME, SCH_EN_NAME, SGE_EN_NAME, SMN_EN_NAME, VPR_EN_NAME, WAR_EN_NAME, WHM_EN_NAME } from "../../../const/languageTexts";

let ALIGN = "left";

function resetOnlyUnequippableEquipment(data: SingleEquipmentInputSaveState) {
  for (let slotIndex = 0; slotIndex < data.itemSet.length; slotIndex++) {
    let newPossibleEquipments = EQUIPMENT_DATABASE_BY_KEYS.get(toEquipmentKeyString(data.mainPlayerJobAbbrev, slotIndexToSlotName(slotIndex)));
    if (newPossibleEquipments === undefined) {
      data.itemSet[slotIndex] = -1;
      data.gearSetMaterias[slotIndex] = [];
    } else {
      let newPossibleEquipmentsId = newPossibleEquipments.map((equipment) => equipment.id);

      // if the switched job can still equip the currently selected equipment, leave the state as it is
      if (!newPossibleEquipmentsId.includes(data.itemSet[slotIndex])) {
        data.itemSet[slotIndex] = EMPTY_EQUIPMENT_ID;
        data.gearSetMaterias[slotIndex] = [];
      }
    }
  }
}

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

    let possibleJobsForSlot = getRoleByIdAndMainCharacterJob(partyMemberId, mainPlayerJobAbbrev, partyMemberJobAbbrevs);

    for (let partyMemberIdSearch = 1; partyMemberIdSearch <= partyMemberJobAbbrevs.length; partyMemberIdSearch++) {
      if (possibleJobsForSlot.includes(partyMemberJobAbbrevs[partyMemberIdSearch - 1]) && !usedIds.includes(partyMemberIdSearch)) {
        newPartyMemberJobAbbrevs[partyMemberId - 1] = partyMemberJobAbbrevs[partyMemberIdSearch - 1];
        usedIds.push(partyMemberIdSearch);
        break;
      }
    }
  }

  let newPartyMemberIds = [];
  for (let partyMemberId = 1; partyMemberId <= newPartyMemberJobAbbrevs.length; partyMemberId++) {
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
) {
  const handleJobChange = (event: SelectChangeEvent<string>) => {
    let newJobAbbrev = event.target.value;
    let newTotalState = { ...totalEquipmentState };

    newTotalState.equipmentDatas.forEach((data: SingleEquipmentInputSaveState) => {
      data.mainPlayerJobAbbrev = newJobAbbrev;
      resetOnlyUnequippableEquipment(data);
      resetInvalidPartnersForNewJob(data);
    });

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
        {JobMenuItem(PLD_EN_NAME, ALIGN)}
        {JobMenuItem(WAR_EN_NAME, ALIGN)}
        {JobMenuItem(DRK_EN_NAME, ALIGN)}
        {JobMenuItem(GNB_EN_NAME, ALIGN)}
        <Divider />
        {JobMenuItem(WHM_EN_NAME, ALIGN)}
        {JobMenuItem(AST_EN_NAME, ALIGN)}
        {JobMenuItem(SCH_EN_NAME, ALIGN)}
        {JobMenuItem(SGE_EN_NAME, ALIGN)}
        <Divider />
        {JobMenuItem(DRG_EN_NAME, ALIGN)}
        {JobMenuItem(MNK_EN_NAME, ALIGN)}
        {JobMenuItem(NIN_EN_NAME, ALIGN)}
        {JobMenuItem(SAM_EN_NAME, ALIGN)}
        {JobMenuItem(RPR_EN_NAME, ALIGN)}
        {JobMenuItem(VPR_EN_NAME, ALIGN)}
        <Divider />
        {JobMenuItem(BRD_EN_NAME, ALIGN)}
        {JobMenuItem(MCH_EN_NAME, ALIGN)}
        {JobMenuItem(DNC_EN_NAME, ALIGN)}
        <Divider />
        {JobMenuItem(SMN_EN_NAME, ALIGN)}
        {JobMenuItem(BLM_EN_NAME, ALIGN)}
        {JobMenuItem(RDM_EN_NAME, ALIGN)}
        {JobMenuItem(PCT_EN_NAME, ALIGN)}
      </Select>
    </CustomFormControl>
  );
}



export function MainPlayerJobSelectionOnlyBuffJobs(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function,
) {
  const handleJobChange = (event: SelectChangeEvent<string>) => {
    let newJobAbbrev = event.target.value;
    let newTotalState = { ...totalEquipmentState };

    newTotalState.equipmentDatas.forEach((data: SingleEquipmentInputSaveState) => {
      resetOnlyUnequippableEquipment(data);
      data.mainPlayerJobAbbrev = newJobAbbrev;
    });

    updateAllPlayerPower(newTotalState, setTotalState);
  };

  let key = `Job-${id}`;

  return (
    <CustomFormControl fullWidth sx={{ height: '100%' }}>
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

        sx={{
          height: '100%',
          display: 'flex',
        }}
      >
        {JobMenuItem(AST_EN_NAME, ALIGN)}
        {JobMenuItem(SCH_EN_NAME, ALIGN)}
        <Divider />
        {JobMenuItem(DRG_EN_NAME, ALIGN)}
        {JobMenuItem(MNK_EN_NAME, ALIGN)}
        {JobMenuItem(NIN_EN_NAME, ALIGN)}
        {JobMenuItem(RPR_EN_NAME, ALIGN)}
        <Divider />
        {JobMenuItem(BRD_EN_NAME, ALIGN)}
        {JobMenuItem(DNC_EN_NAME, ALIGN)}
        <Divider />
        {JobMenuItem(SMN_EN_NAME, ALIGN)}
        {JobMenuItem(RDM_EN_NAME, ALIGN)}
        {JobMenuItem(PCT_EN_NAME, ALIGN)}
      </Select>
    </CustomFormControl>
  );
}
