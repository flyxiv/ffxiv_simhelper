import { CharacterStates } from "src/types/CharacterStates";
import {
  styled,
  InputLabel,
  Grid,
  Select,
  SelectChangeEvent,
  MenuItem,
} from "@mui/material";
import { inputStyleJob, CustomFormControl } from "./BasicInputForm";
import { InputGridContainerStyle, InputGridItemStyle } from "../Styles";
import {
  Equipment,
  EQUIPMENT_DATABASE_BY_KEYS,
  getLeftEquipmentSlotsOfJob,
  toEquipmentKeyString,
} from "src/types/ffxivdatabase/Equipment";
import { ItemSet } from "src/types/ffxivdatabase/ItemSet";
import { EquipmentMenuItem } from "src/components/items/EquipmentMenuItem";
import { MainPlayerJobSelection } from "../jobselection/MainPlayerJobSelection";

const InputGridContainer = styled(Grid)`
  ${InputGridContainerStyle}
`;

const InputGridItemBox = styled(Grid)`
  ${InputGridItemStyle}
`;
const InputEquipmentBox = styled(Grid)`
  ${inputStyleJob}
`;

function EquipmentMenuOfOneSlot(
  id: number,
  slotName: string,
  jobAbbrev: string,
  equipmentsAvailableInSlot: Equipment[],
  itemSet: ItemSet,
  setItemSet: Function
) {
  let key = `${slotName}-${id}`;
  let currentEquipmentId = itemSet.get(slotName);
  if (currentEquipmentId === undefined) {
    currentEquipmentId = -1;
  }

  const updateEquipmentState = (e: SelectChangeEvent<number>) => {
    let newEquipmentId = e.target.value;
    if (typeof newEquipmentId === "string") {
      newEquipmentId = parseInt(newEquipmentId);
    }
    let newSet = new Map(itemSet);
    newSet.set(slotName, newEquipmentId);
    setItemSet(newSet);
  };

  let slotLabel = slotName;
  if (currentEquipmentId !== -1) {
    slotLabel = "";
  }

  return (
    <>
      <CustomFormControl fullWidth>
        <InputLabel id="JobSelect">{slotLabel}</InputLabel>
        <Select
          labelId={key}
          id={key}
          value={currentEquipmentId}
          key={key}
          label={slotName}
          onChange={updateEquipmentState}
        >
          {equipmentsAvailableInSlot.map((equipment) => {
            return EquipmentMenuItem(equipment, jobAbbrev);
          })}
          <MenuItem value={-1}>Empty</MenuItem>
        </Select>
      </CustomFormControl>
    </>
  );
}

export function EquipmentSelectionMenu(
  mainCharacterState: CharacterStates,
  itemSet: ItemSet,
  setItemSet: Function
) {
  let xs = 15;
  let mainCharacterJobAbbrev = mainCharacterState.jobAbbrev;
  return (
    <InputGridContainer container>
      <InputGridItemBox marginBottom={1}>
        <InputEquipmentBox item xs={xs} key="Job">
          {MainPlayerJobSelection(
            mainCharacterState.jobAbbrev,
            mainCharacterState.jobNameSetter,
            setItemSet
          )}
        </InputEquipmentBox>
      </InputGridItemBox>
      {getLeftEquipmentSlotsOfJob(mainCharacterJobAbbrev).map((slotName) => {
        let equipmentKeyString = toEquipmentKeyString(
          mainCharacterJobAbbrev,
          slotName
        );
        let equipmentsAvailableInSlot =
          EQUIPMENT_DATABASE_BY_KEYS.get(equipmentKeyString);
        if (equipmentsAvailableInSlot === undefined) {
          return;
        }
        return (
          <InputGridItemBox marginBottom={1}>
            <InputEquipmentBox item xs={xs} key="Job">
              {EquipmentMenuOfOneSlot(
                0,
                slotName,
                mainCharacterJobAbbrev,
                equipmentsAvailableInSlot,
                itemSet,
                setItemSet
              )}
            </InputEquipmentBox>
          </InputGridItemBox>
        );
      })}
    </InputGridContainer>
  );
}
