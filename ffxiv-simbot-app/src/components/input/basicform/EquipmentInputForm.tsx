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
  updateOnePlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { EquipmentMenuItem } from "../../../components/items/EquipmentMenuItem";
import {
  MainPlayerJobSelection,
  MainPlayerJobSelectionOnlyBuffJobs,
} from "../jobselection/MainPlayerJobSelection";
import { MainPlayerRaceSelection } from "../RaceSelection";
import { FoodMenuItem } from "../../../components/items/FoodMenuItem";
import { ALL_FOODS } from "../../../types/ffxivdatabase/Food";
import { EMPTY_MATERIA, Materia } from "../../../types/ffxivdatabase/Materia";
import { EquipmentSubStatTable } from "../../../components/container/EquipmentSubStatBox";
import { MateriaInputTable } from "./MateriaInputForm";
import { MenuItemStyle } from "../../../components/items/Styles";
import { ColorConfigurations } from "../../../Themes";
import { EquipmentInput } from "../../../types/EquipmentInput";
import { SimulationResultTimeTextBox } from "../SimulationResultTextBox";
import { Partner1Selection, Partner2Selection } from "../PartnerSelection";

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
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function
) {
  let totalState = totalEquipmentState.equipmentDatas[id];
  let key = `${slotName}-${id}-equipment`;
  let currentEquipmentId = totalState.itemSet[slotNameToSlotIndex(slotName)];

  let currentEquipment = PLAYER_EQUIPMENTS.get(currentEquipmentId);
  const updateEquipmentState = (e: SelectChangeEvent<number>) => {
    let newEquipmentId = e.target.value;

    if (typeof newEquipmentId === "string") {
      newEquipmentId = parseInt(newEquipmentId);
    }
    let newTotalData = { ...totalEquipmentState };
    let newData = newTotalData.equipmentDatas[id];
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
    updateOnePlayerPower(id, newTotalData, setTotalEquipmentState);
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
          sx={{
            '&.Mui-focused .MuiOutlinedInput-notchedOutline': {
              borderColor: 'transparent', // 포커스된 상태에서의 테두리 색 제거
            },
          }}
        >
          {equipmentsAvailableInSlot.map((equipment) => {
            return EquipmentMenuItem(
              id,
              equipment,
              totalState.mainPlayerJobAbbrev
            );
          })}
          <Divider />
          <EquipmentMenu value={-1} key={`${id}_${slotLabel}_empty`}>
            <Box display="flex" height="100%" alignItems="center" justifyContent="flex-end">
              <Typography variant="body2" color="white">
                Empty
              </Typography>
            </Box>
          </EquipmentMenu>
        </Select>
      </CustomFormControl>
      <MateriaBox>
        {MateriaInputTable(
          id,
          slotName,
          currentEquipment,
          totalEquipmentState,
          setTotalEquipmentState
        )}
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
    </>
  );
}

export function EquipmentSelectionMenu(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  onlyBuffJobs: boolean = false,
  hasTimeInput: boolean = false
) {
  let xs = 12;
  let mainCharacterJobAbbrev =
    totalEquipmentState.equipmentDatas[id].mainPlayerJobAbbrev;
  return (
    <EquipmentGridContainer container>
      <EquipmentGridItemBox key={`${id}_JobSelectionItemBox`}>
        <InputEquipmentBox item xs={xs} key={`Job_${id}`}>
          {onlyBuffJobs
            ? MainPlayerJobSelectionOnlyBuffJobs(
              id,
              totalEquipmentState,
              setTotalEquipmentState
            )
            : MainPlayerJobSelection(
              id,
              totalEquipmentState,
              setTotalEquipmentState
            )}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
      <EquipmentGridItemBox marginBottom={1} key={`${id}_RaceItemBox`}>
        <InputEquipmentBox item xs={xs} key="Race">
          {MainPlayerRaceSelection(
            id,
            totalEquipmentState,
            setTotalEquipmentState
          )}
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
          <EquipmentGridItemBox key={`${id}_equipment_${slotName}_itembox`}>
            <InputEquipmentBox
              item
              xs={xs}
              key={`${id}_${slotName}_equipmentselection`}
            >
              {EquipmentMenuOfOneSlot(
                id,
                slotName,
                equipmentsAvailableInSlot,
                totalEquipmentState,
                setTotalEquipmentState
              )}
            </InputEquipmentBox>
          </EquipmentGridItemBox>
        );
      })}
      <EquipmentGridItemBox key={`food_selectionbox_${id}`}>
        <InputEquipmentBox item xs={xs} key={`food_${id}`}>
          {FoodSelection(id, totalEquipmentState, setTotalEquipmentState)}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
      {PartnerSelectionMenu(id, totalEquipmentState, setTotalEquipmentState)}
      {hasTimeInput ? (
        <EquipmentGridItemBox>
          <InputEquipmentBox item xs={xs} key="time">
            {SimulationResultTimeTextBox(
              "Combat Time(Seconds)",
              totalEquipmentState,
              setTotalEquipmentState,
              true
            )}
          </InputEquipmentBox>
        </EquipmentGridItemBox>
      ) : (
        <Box></Box>
      )}
    </EquipmentGridContainer>
  );
}

function PartnerSelectionMenu(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function
) {
  let mainPlayerJobAbbrev =
    totalEquipmentState.equipmentDatas[id].mainPlayerJobAbbrev;

  if (mainPlayerJobAbbrev === "AST") {
    return (
      <>
        <EquipmentGridItemBox key={`partner_${id}`}>
          <InputEquipmentBox item xs={12} key={`partner_${id}`}>
            {Partner1Selection(
              id,
              totalEquipmentState,
              setTotalEquipmentState,
              "Melee Partner"
            )}
          </InputEquipmentBox>
        </EquipmentGridItemBox>

        <EquipmentGridItemBox key={`partner_${id}`}>
          <InputEquipmentBox item xs={12} key={`partner_${id}`}>
            {Partner2Selection(
              id,
              totalEquipmentState,
              setTotalEquipmentState,
              "Ranged Partner"
            )}
          </InputEquipmentBox>
        </EquipmentGridItemBox>
      </>
    );
  } else if (mainPlayerJobAbbrev === "DNC") {
    return (
      <EquipmentGridItemBox key={`partner_${id}`}>
        <InputEquipmentBox item xs={12} key={`partner_${id}`}>
          {Partner1Selection(
            id,
            totalEquipmentState,
            setTotalEquipmentState,
            "Dance Partner"
          )}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
    );
  } else {
    return <Box></Box>;
  }
}

function FoodSelection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function
) {
  let totalState = totalEquipmentState.equipmentDatas[id];

  let foodLabel = "food";
  if (totalState.foodId !== -1) {
    foodLabel = "";
  }

  const updateFoodState = (e: SelectChangeEvent<number>) => {
    let newFoodId = e.target.value;
    if (typeof newFoodId === "string") {
      newFoodId = parseInt(newFoodId);
    }
    let newTotalData = { ...totalEquipmentState };
    newTotalData.equipmentDatas[id].foodId = newFoodId;

    updateOnePlayerPower(id, newTotalData, setTotalEquipmentState);
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
            <Box display="flex" height="100%" alignItems="center" justifyContent="flex-end">
              <Typography variant="body2" color="white">
                Empty
              </Typography>
            </Box>
          </MenuItem>
        </Select>
      </CustomFormControl>
    </>
  );
}
