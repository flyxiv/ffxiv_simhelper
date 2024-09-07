import { Divider, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { JobMenuItem } from "../../items/JobMenuItem";
import {
  slotIndexToSlotName,
  updateAllPlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { ColorConfigurations } from "../../../Themes";
import { EquipmentInput, SingleEquipmentInputSaveState } from "../../../types/EquipmentInput";
import { EMPTY_EQUIPMENT_ID, EQUIPMENT_DATABASE_BY_KEYS, toEquipmentKeyString } from "../../../types/ffxivdatabase/Equipment";
import { getRoleByIdAndMainCharacterJob } from "./PartyMemberJobSelection";

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
              backgroundColor: ColorConfigurations.backgroundThree,
            },
          },
        }}
      >
        {JobMenuItem("PLD", ALIGN)}
        {JobMenuItem("WAR", ALIGN)}
        {JobMenuItem("DRK", ALIGN)}
        {JobMenuItem("GNB", ALIGN)}
        <Divider />
        {JobMenuItem("WHM", ALIGN)}
        {JobMenuItem("AST", ALIGN)}
        {JobMenuItem("SCH", ALIGN)}
        {JobMenuItem("SGE", ALIGN)}
        <Divider />
        {JobMenuItem("DRG", ALIGN)}
        {JobMenuItem("MNK", ALIGN)}
        {JobMenuItem("NIN", ALIGN)}
        {JobMenuItem("SAM", ALIGN)}
        {JobMenuItem("RPR", ALIGN)}
        {JobMenuItem("VPR", ALIGN)}
        <Divider />
        {JobMenuItem("BRD", ALIGN)}
        {JobMenuItem("MCH", ALIGN)}
        {JobMenuItem("DNC", ALIGN)}
        <Divider />
        {JobMenuItem("SMN", ALIGN)}
        {JobMenuItem("BLM", ALIGN)}
        {JobMenuItem("RDM", ALIGN)}
        {JobMenuItem("PCT", ALIGN)}
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
              backgroundColor: ColorConfigurations.backgroundThree,
            },
          },
        }}

        sx={{
          height: '100%',
          display: 'flex',
        }}
      >
        {JobMenuItem("AST", ALIGN)}
        {JobMenuItem("SCH", ALIGN)}
        <Divider />
        {JobMenuItem("DRG", ALIGN)}
        {JobMenuItem("MNK", ALIGN)}
        {JobMenuItem("NIN", ALIGN)}
        {JobMenuItem("RPR", ALIGN)}
        <Divider />
        {JobMenuItem("BRD", ALIGN)}
        {JobMenuItem("DNC", ALIGN)}
        <Divider />
        {JobMenuItem("SMN", ALIGN)}
        {JobMenuItem("RDM", ALIGN)}
        {JobMenuItem("PCT", ALIGN)}
      </Select>
    </CustomFormControl>
  );
}
