import { CharacterStates } from "src/types/CharacterStates";
import { styled, Box, Grid } from "@mui/material";
import { inputStyleJob } from "./BasicInputForm";
import { InputGridContainerStyle, InputGridItemStyle } from "../Styles";
import { ItemSet } from "src/types/ffxivdatabase/ItemSet";
import { EQUIPMENT_DATABASE } from "src/types/ffxivdatabase/Equipment";
import { getEquipmentIconDirectory } from "src/components/icon/equipmenticon/EquipmentIconFactory";

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
  const testItem = EQUIPMENT_DATABASE.get("weapon-PLD");
  if (testItem === undefined) {
    return <div>Item not found</div>;
  }
  let firstItem = testItem[0];
  let directory = getEquipmentIconDirectory("mainhand", "PLD", firstItem.name);
  console.log(directory);

  return <img src={directory} alt={directory} />;
}
