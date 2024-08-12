import { CharacterStates } from "src/types/CharacterStates";
import {
  styled,
  InputLabel,
  Grid,
  Select,
  SelectChangeEvent,
  MenuItem,
  Box,
} from "@mui/material";
import { CustomFormControl } from "./BasicInputForm";
import {
  EquipmentGridContainerStyle,
  EquipmentGridItemStyle,
  EquipmentStatBoxStyle,
  EquipmentStyle,
  MateriaInputBoxStyle,
} from "../Styles";
import {
  Equipment,
  EQUIPMENT_DATABASE_BY_ID,
  EQUIPMENT_DATABASE_BY_KEYS,
  getEquipmentSlotsOfJob,
  toEquipmentKeyString,
} from "src/types/ffxivdatabase/Equipment";
import { ItemSet } from "src/types/ffxivdatabase/ItemSet";
import { EquipmentMenuItem } from "src/components/items/EquipmentMenuItem";
import { MainPlayerJobSelection } from "../jobselection/MainPlayerJobSelection";
import { MainPlayerRaceSelection } from "../RaceSelection";
import { FoodMenuItem } from "src/components/items/FoodMenuItem";
import { ALL_FOODS } from "src/types/ffxivdatabase/Food";
import { EMPTY_MATERIA, Materia } from "src/types/ffxivdatabase/Materia";
import { EquipmentSubStatTable } from "src/components/container/EquipmentSubStatBox";
import { MateriaInputTable } from "./MateriaInputForm";

const EquipmentGridContainer = styled(Grid)`
  ${EquipmentGridContainerStyle}
`;

const EquipmentGridItemBox = styled(Grid)`
  ${EquipmentGridItemStyle}
`;
const InputEquipmentBox = styled(Grid)`
  ${EquipmentStyle}
`;

const MateriaBox = styled(Box)`
  ${MateriaInputBoxStyle}
`;

const EquipmentStatBox = styled(Box)`
  ${EquipmentStatBoxStyle}
`;

let PLAYER_EQUIPMENTS = new Map(EQUIPMENT_DATABASE_BY_ID);

function EquipmentMenuOfOneSlot(
  id: number,
  slotName: string,
  jobAbbrev: string,
  equipmentsAvailableInSlot: Equipment[],
  itemSet: ItemSet,
  setItemSet: Function,
  gearSetMaterias: Map<string, Materia[]>,
  setGearSetMaterias: Function
) {
  let key = `${slotName}-${id}`;
  let currentEquipmentId = itemSet.get(slotName);
  if (currentEquipmentId === undefined) {
    currentEquipmentId = -1;
  }

  let currentEquipment = PLAYER_EQUIPMENTS.get(currentEquipmentId);
  const updateEquipmentState = (e: SelectChangeEvent<number>) => {
    let newEquipmentId = e.target.value;
    let materiaSlotCount = 0;

    if (typeof newEquipmentId === "string") {
      newEquipmentId = parseInt(newEquipmentId);
    }
    let newSet = new Map(itemSet);
    newSet.set(slotName, newEquipmentId);
    setItemSet(newSet);
    currentEquipment = PLAYER_EQUIPMENTS.get(newEquipmentId);
    if (currentEquipment !== undefined) {
      materiaSlotCount = currentEquipment.pentameldable
        ? 5
        : currentEquipment.materiaSlotCount;
    }

    let defaultMaterias: Materia[] = [];
    for (let i = 0; i < materiaSlotCount; i++) {
      defaultMaterias.push(EMPTY_MATERIA);
    }
    let newGearSetMaterias = new Map(gearSetMaterias);
    newGearSetMaterias.set(slotName, defaultMaterias);
    setGearSetMaterias(newGearSetMaterias);
  };

  let slotLabel = slotName;
  if (currentEquipmentId !== -1) {
    slotLabel = "";
  }

  return (
    <>
      <CustomFormControl fullWidth>
        <InputLabel id="SlotSelect">{slotLabel}</InputLabel>
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
      <MateriaBox>
        {MateriaInputTable(
          slotName,
          currentEquipment,
          gearSetMaterias,
          setGearSetMaterias
        )}
      </MateriaBox>

      <EquipmentStatBox>
        {currentEquipment === undefined ? (
          <></>
        ) : (
          EquipmentSubStatTable(currentEquipment, gearSetMaterias.get(slotName))
        )}
      </EquipmentStatBox>
    </>
  );
}
export function EquipmentSelectionMenu(
  mainCharacterState: CharacterStates,
  itemSet: ItemSet,
  setItemSet: Function,
  race: string,
  setRace: Function,
  foodId: number,
  setFoodId: Function,
  gearSetMaterias: Map<string, Materia[]>,
  setGearSetMaterias: Function
) {
  let xs = 12;
  let mainCharacterJobAbbrev = mainCharacterState.jobAbbrev;
  return (
    <EquipmentGridContainer container>
      <EquipmentGridItemBox marginBottom={1}>
        <InputEquipmentBox item xs={xs} key="Job">
          {MainPlayerJobSelection(
            mainCharacterState.jobAbbrev,
            mainCharacterState.jobNameSetter,
            setItemSet
          )}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
      <EquipmentGridItemBox marginBottom={1}>
        <InputEquipmentBox item xs={xs} key="Race">
          {MainPlayerRaceSelection(race, setRace, mainCharacterJobAbbrev)}
        </InputEquipmentBox>
      </EquipmentGridItemBox>

      {getEquipmentSlotsOfJob(mainCharacterJobAbbrev).map((slotName) => {
        let equipmentKeyString = toEquipmentKeyString(
          mainCharacterJobAbbrev,
          slotName
        );
        let equipmentsAvailableInSlot =
          EQUIPMENT_DATABASE_BY_KEYS.get(equipmentKeyString);
        if (equipmentsAvailableInSlot === undefined) {
          console.log(equipmentKeyString);
          return;
        }
        return (
          <EquipmentGridItemBox marginBottom={1}>
            <InputEquipmentBox item xs={xs} key={slotName}>
              {EquipmentMenuOfOneSlot(
                0,
                slotName,
                mainCharacterJobAbbrev,
                equipmentsAvailableInSlot,
                itemSet,
                setItemSet,
                gearSetMaterias,
                setGearSetMaterias
              )}
            </InputEquipmentBox>
          </EquipmentGridItemBox>
        );
      })}
      <EquipmentGridItemBox marginBottom={1}>
        <InputEquipmentBox item xs={xs} key="food">
          {FoodSelection(foodId, setFoodId)}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
    </EquipmentGridContainer>
  );
}

function FoodSelection(foodId: number, setFoodId: Function) {
  let foodLabel = "Food";
  if (foodId !== -1) {
    foodLabel = "";
  }

  const updateFoodState = (e: SelectChangeEvent<number>) => {
    let newFoodId = e.target.value;
    if (typeof newFoodId === "string") {
      newFoodId = parseInt(newFoodId);
    }

    setFoodId(newFoodId);
  };

  return (
    <>
      <CustomFormControl fullWidth>
        <InputLabel id="FoodSelect">{foodLabel}</InputLabel>
        <Select
          labelId="food"
          id="food"
          value={foodId}
          key="food"
          label="food"
          onChange={updateFoodState}
        >
          {ALL_FOODS.map((food) => {
            return FoodMenuItem(food);
          })}
          <MenuItem value={-1}>Empty</MenuItem>
        </Select>
      </CustomFormControl>
    </>
  );
}
