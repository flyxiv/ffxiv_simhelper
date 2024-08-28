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
} from "../../../types/ffxivdatabase/Equipment";
import {
  slotNameToSlotIndex,
  updatePlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { EquipmentMenuItem } from "../../../components/items/EquipmentMenuItem";
import { MainPlayerJobSelection } from "../jobselection/MainPlayerJobSelection";
import { MainPlayerRaceSelection } from "../RaceSelection";
import { FoodMenuItem } from "../../../components/items/FoodMenuItem";
import { ALL_FOODS } from "../../../types/ffxivdatabase/Food";
import { EMPTY_MATERIA, Materia } from "../../../types/ffxivdatabase/Materia";
import { EquipmentSubStatTable } from "../../../components/container/EquipmentSubStatBox";
import { MateriaInputTable } from "./MateriaInputForm";
import { MenuItemStyle } from "../../../components/items/Styles";
import { ColorConfigurations } from "../../../Themes";
import { SingleEquipmentInputSaveState } from "../../../types/SingleEquipmentInputSaveState";

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
  equipmentsAvailableInSlot: Equipment[],
  totalState: SingleEquipmentInputSaveState,
  setTotalState: Function
) {
  let key = `${slotName}-${id}-equipment`;
  let currentEquipmentId = totalState.itemSet[slotNameToSlotIndex(slotName)];

  let currentEquipment = PLAYER_EQUIPMENTS.get(currentEquipmentId);
  const updateEquipmentState = (e: SelectChangeEvent<number>) => {
    let newEquipmentId = e.target.value;

    if (typeof newEquipmentId === "string") {
      newEquipmentId = parseInt(newEquipmentId);
    }
    let newData = { ...totalState };
    let newSet = [...totalState.itemSet];
    newSet[slotNameToSlotIndex(slotName)] = newEquipmentId;
    newData.itemSet = newSet;

    let newGearSetMaterias = [...totalState.gearSetMaterias];

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
    updatePlayerPower(newData, setTotalState);
  };

  let slotLabel = slotName;
  if (currentEquipmentId !== -1) {
    slotLabel = "";
  }

  return (
    <Box height="10vh">
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
            return EquipmentMenuItem(equipment, totalState.mainPlayerJobAbbrev);
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
        {MateriaInputTable(slotName, currentEquipment, totalState, setTotalState)}
      </MateriaBox>

      <EquipmentStatBox>
        {currentEquipment === undefined ? (
          <></>
        ) : (
          EquipmentSubStatTable(
            currentEquipment,
            totalState.gearSetMaterias[slotNameToSlotIndex(slotName)]
          )
        )}
      </EquipmentStatBox>
    </Box>
  );
}

export function EquipmentSelectionMenu(
  id: number,
  totalState: SingleEquipmentInputSaveState,
  setTotalState: Function
) {
  let xs = 12;
  let mainCharacterJobAbbrev = totalState.mainPlayerJobAbbrev;
  return (
    <EquipmentGridContainer container>
      <EquipmentGridItemBox marginBottom={1} key={`${id}_JobSelectionItemBox`}>
        <InputEquipmentBox item xs={xs} key="Job">
          {MainPlayerJobSelection(
            id,
            totalState,
            setTotalState
          )}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
      <EquipmentGridItemBox marginBottom={1} key={`${id}_RaceItemBox`}>
        <InputEquipmentBox item xs={xs} key="Race">
          {MainPlayerRaceSelection(id, totalState, setTotalState)}
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
                equipmentsAvailableInSlot,
                totalState,
                setTotalState
              )}
            </InputEquipmentBox>
          </EquipmentGridItemBox>
        );
      })}
      <EquipmentGridItemBox marginBottom={1} key={`food_selectionbox`}>
        <InputEquipmentBox item xs={xs} key="food">
          {FoodSelection(id, totalState, setTotalState)}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
    </EquipmentGridContainer>
  );
}

function FoodSelection(
  id: number,
  totalState: SingleEquipmentInputSaveState,
  setTotalState: Function
) {
  let foodLabel = "food";
  if (totalState.foodId !== -1) {
    foodLabel = "";
  }

  const updateFoodState = (e: SelectChangeEvent<number>) => {
    let newFoodId = e.target.value;
    if (typeof newFoodId === "string") {
      newFoodId = parseInt(newFoodId);
    }
    let newData = { ...totalState, foodId: newFoodId };
    updatePlayerPower(newData, setTotalState);
  };

  let key = `food-${id}`;

  return (
    <>
      <CustomFormControl fullWidth>
        <InputLabel id="FoodSelect">{foodLabel}</InputLabel>
        <Select
          labelId={key}
          id={key}
          value={totalState.foodId}
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
