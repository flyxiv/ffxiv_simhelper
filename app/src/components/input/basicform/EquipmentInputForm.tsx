import { EquipmentSimCharacterStates } from "src/types/CharacterStates";
import {
  styled,
  InputLabel,
  Grid,
  Select,
  SelectChangeEvent,
  MenuItem,
  Box,
  Typography,
  Divider,
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
import {
  slotNameToSlotIndex,
  updatePlayerPower,
} from "src/types/ffxivdatabase/ItemSet";
import { EquipmentMenuItem } from "src/components/items/EquipmentMenuItem";
import { MainPlayerJobSelection } from "../jobselection/MainPlayerJobSelection";
import { MainPlayerRaceSelection } from "../RaceSelection";
import { FoodMenuItem } from "src/components/items/FoodMenuItem";
import { ALL_FOODS } from "src/types/ffxivdatabase/Food";
import { EMPTY_MATERIA, Materia } from "src/types/ffxivdatabase/Materia";
import { EquipmentSubStatTable } from "src/components/container/EquipmentSubStatBox";
import { MateriaInputTable } from "./MateriaInputForm";
import { CharacterEquipmentsData } from "src/types/ffxivdatabase/PlayerPower";
import { MenuItemStyle } from "src/components/items/Styles";
import { ColorConfigurations } from "src/Themes";

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

const EquipmentMenu = styled(MenuItem)`
  ${MenuItemStyle}
`;

let PLAYER_EQUIPMENTS = new Map(EQUIPMENT_DATABASE_BY_ID);

function EquipmentMenuOfOneSlot(
  id: number,
  slotName: string,
  jobAbbrev: string,
  equipmentsAvailableInSlot: Equipment[],
  data: CharacterEquipmentsData,
  setData: Function
) {
  let key = `${slotName}-${id}-equipment`;
  let currentEquipmentId = data.itemSet[slotNameToSlotIndex(slotName)];

  let currentEquipment = PLAYER_EQUIPMENTS.get(currentEquipmentId);
  const updateEquipmentState = (e: SelectChangeEvent<number>) => {
    let newEquipmentId = e.target.value;

    if (typeof newEquipmentId === "string") {
      newEquipmentId = parseInt(newEquipmentId);
    }
    let newData = { ...data };
    let newSet = [...data.itemSet];
    newSet[slotNameToSlotIndex(slotName)] = newEquipmentId;
    newData.itemSet = newSet;

    let newGearSetMaterias = [...data.gearSetMaterias];

    let materiaSlotCount = 0;
    currentEquipment = PLAYER_EQUIPMENTS.get(newEquipmentId);
    if (currentEquipment !== undefined) {
      materiaSlotCount = currentEquipment.pentameldable
        ? 5
        : currentEquipment.materiaSlotCount;
      let defaultMaterias: Materia[] = [];
      for (let i = 0; i < materiaSlotCount; i++) {
        defaultMaterias.push(EMPTY_MATERIA);
      }
      newGearSetMaterias[slotNameToSlotIndex(slotName)] = defaultMaterias;
    } else {
      newGearSetMaterias[slotNameToSlotIndex(slotName)] = [];
    }

    newData.gearSetMaterias = newGearSetMaterias;
    updatePlayerPower(newData, setData);
  };

  let slotLabel = slotName;
  if (currentEquipmentId !== -1) {
    slotLabel = "";
  }

  return (
    <>
      <CustomFormControl fullWidth>
        <InputLabel id="SlotSelect" key={`${key}_label`}>
          {slotLabel}
        </InputLabel>
        <Select
          labelId={key}
          id={key}
          value={currentEquipmentId}
          key={key}
          label={slotName}
          onChange={updateEquipmentState}
          MenuProps={{
            PaperProps: {
              sx: {
                backgroundColor: ColorConfigurations.backgroundThree,
              },
            },
          }}
        >
          {equipmentsAvailableInSlot.map((equipment) => {
            return EquipmentMenuItem(equipment, jobAbbrev);
          })}
          <Divider />
          <EquipmentMenu value={-1} key={`${id}_${slotLabel}_empty`}>
            <Typography variant="body2" color="white">
              Empty
            </Typography>
          </EquipmentMenu>
        </Select>
      </CustomFormControl>
      <MateriaBox>
        {MateriaInputTable(slotName, currentEquipment, data, setData)}
      </MateriaBox>

      <EquipmentStatBox>
        {currentEquipment === undefined ? (
          <></>
        ) : (
          EquipmentSubStatTable(
            currentEquipment,
            data.gearSetMaterias[slotNameToSlotIndex(slotName)]
          )
        )}
      </EquipmentStatBox>
    </>
  );
}

export function EquipmentSelectionMenu(
  id: number,
  mainCharacterState: EquipmentSimCharacterStates,
  data: CharacterEquipmentsData,
  setData: Function
) {
  let xs = 12;
  let mainCharacterJobAbbrev = mainCharacterState.jobAbbrev;
  return (
    <EquipmentGridContainer container>
      <EquipmentGridItemBox marginBottom={1} key={`${id}_JobSelectionItemBox`}>
        <InputEquipmentBox item xs={xs} key="Job">
          {MainPlayerJobSelection(
            id,
            mainCharacterState.jobAbbrev,
            mainCharacterState.jobNameSetter,
            data,
            setData
          )}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
      <EquipmentGridItemBox marginBottom={1} key={`${id}_RaceItemBox`}>
        <InputEquipmentBox item xs={xs} key="Race">
          {MainPlayerRaceSelection(id, mainCharacterJobAbbrev, data, setData)}
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
          <EquipmentGridItemBox
            marginBottom={1}
            key={`${id}_equipment_${slotName}_itembox`}
          >
            <InputEquipmentBox
              item
              xs={xs}
              key={`${id}_${slotName}_equipmentselection`}
            >
              {EquipmentMenuOfOneSlot(
                id,
                slotName,
                mainCharacterJobAbbrev,
                equipmentsAvailableInSlot,
                data,
                setData
              )}
            </InputEquipmentBox>
          </EquipmentGridItemBox>
        );
      })}
      <EquipmentGridItemBox marginBottom={1} key={`food_selectionbox`}>
        <InputEquipmentBox item xs={xs} key="food">
          {FoodSelection(id, data, setData)}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
    </EquipmentGridContainer>
  );
}

function FoodSelection(
  id: number,
  data: CharacterEquipmentsData,
  setData: Function
) {
  let foodLabel = "Food";
  if (data.foodId !== -1) {
    foodLabel = "";
  }

  const updateFoodState = (e: SelectChangeEvent<number>) => {
    let newFoodId = e.target.value;
    if (typeof newFoodId === "string") {
      newFoodId = parseInt(newFoodId);
    }
    let newData: CharacterEquipmentsData = { ...data, foodId: newFoodId };
    updatePlayerPower(newData, setData);
  };

  let key = `food-${id}`;

  return (
    <>
      <CustomFormControl fullWidth>
        <InputLabel id="FoodSelect">{foodLabel}</InputLabel>
        <Select
          labelId={key}
          id={key}
          value={data.foodId}
          key={key}
          label="food"
          onChange={updateFoodState}
          MenuProps={{
            PaperProps: {
              sx: {
                backgroundColor: ColorConfigurations.backgroundThree,
              },
            },
          }}
        >
          {ALL_FOODS.map((food) => {
            return FoodMenuItem(food);
          })}
          <MenuItem value={-1}>
            <Typography variant="body2" color="white">
              Empty
            </Typography>
          </MenuItem>
        </Select>
      </CustomFormControl>
    </>
  );
}
