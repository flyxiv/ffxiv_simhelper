import { JobSelectionWithState } from "./JobSelectionWithState";
import { CharacterStates, CharacterStats } from "src/types/CharacterStates";
import { styled, Box, Grid } from "@mui/material";
import { CustomInputForm, inputStyleJob } from "./InputForm";
import { InputGridContainerStyle, InputGridItemStyle } from "./Styles";
import { Partner1Selection } from "./PartnerSelection";
import { ItemSet } from "src/types/ffxivdatabase/ItemSet";
import {
  Equipment,
  readEquipmentData,
} from "src/types/ffxivdatabase/Equipment";
import { useState, useEffect } from "react";

const InputGridContainer = styled(Grid)`
  ${InputGridContainerStyle}
`;

const InputGridItem = styled(Grid)`
  ${InputGridItemStyle}
`;
const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;
const InputJobBox = styled(Grid)`
  ${inputStyleJob}
`;

export function ItemInputForm(
  mainCharacterState: CharacterStates,
  availablePartyIds: number[],
  startItems: ItemSet
) {
  const xs = 15;
  const equipmentDatabase: Map<string, Equipment[]> = readEquipmentData(660);
  const test = equipmentDatabase.get("weapon-PLD");
  if (test === undefined) {
    return <></>;
  }

  let directory = getEquipmentIconDirectory(test[1]);

  return (
    <InputGridContainer container>
      <img src={directory} alt={directory} />
      {Partner1Selection(mainCharacterState, availablePartyIds)}
    </InputGridContainer>
  );
}

function getEquipmentIconDirectory(equipment: Equipment) {
  let jobName = equipment.jobName;
  let slotName = equipment.slotName.toLowerCase();
  let equipmentName = equipment.name;
  let equipmentIconName = equipmentName.toLowerCase().replace(/ /g, "_");

  return (
    process.env.PUBLIC_URL +
    `/images/equipments/${jobName}/${slotName}/${equipmentIconName}.png`
  );
}
