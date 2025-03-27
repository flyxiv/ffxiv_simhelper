import {
  styled,
  Grid,
  MenuItem,
  Box,
  Typography,
  Divider,
  TextField,
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
  EMPTY_EQUIPMENT_ID,
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
  MainPlayerGcdSelection,
  MainPlayerJobSelection,
  MainPlayerJobSelectionOnlyBuffJobs,
} from "../jobselection/MainPlayerJobSelection";
import { MainPlayerRaceSelection } from "../RaceSelection";
import { FoodMenuItem } from "../../../components/items/FoodMenuItem";
import { ALL_FOODS } from "../../../types/ffxivdatabase/Food";
import { EMPTY_MATERIA, Materia } from "../../../types/ffxivdatabase/Materia";
import { EquipmentSubStatTable } from "../../../components/container/EquipmentSubStatBox";
import { MateriaInputTable } from "./MateriaInputForm";
import { ITEM_BOTTOM_MENU_MIN_HEIGHT, ITEM_TOP_MENU_MIN_HEIGHT, MenuItemStyle } from "../../../components/items/Styles";
import { AppConfigurations } from "../../../Themes";
import {
  EquipmentInput,
  NO_POT_VAL,
  USE_POT_VAL,
} from "../../../types/EquipmentInput";
import { SimulationUpperInputTimeTextBox } from "../SimulationResultTextBox";
import { Partner1Selection, Partner2Selection } from "../PartnerSelection";
import { AST_EN_NAME, convertToSlotText, DNC_EN_NAME, FINGER1_SLOT_EN_TEXT, FINGER2_SLOT_EN_TEXT, TextDictionary, WEAPON_SLOT_EN_TEXT } from "../../../const/languageTexts";
import React from "react";

const topMenuInputStyle = {
  "& .MuiInputBase-input": {
    color: "white",
    backgroundColor: AppConfigurations.backgroundThree,
    textAlign: "right",
    height: ITEM_TOP_MENU_MIN_HEIGHT,
  },
  "& .MuiInputLabel-root": {
    color: AppConfigurations.primary,
  },
  "& .MuiOutlinedInput-root": {
    "& fieldset": {},
    "&:hover fieldset": {},
    notched: false,
    backgroundColor: AppConfigurations.backgroundThree
  },
  "& .MuiSelect-root": {
    backgroundColor: AppConfigurations.backgroundThree,
  },
  label: {
    transform: "translate(15px, 3vh)",
  },
}

const bottomMenuInputStyle = {
  "& .MuiInputBase-input": {
    color: "white",
    backgroundColor: AppConfigurations.backgroundThree,
    textAlign: "right",
    height: ITEM_BOTTOM_MENU_MIN_HEIGHT,
  },
  "& .MuiInputLabel-root": {
    color: AppConfigurations.primary,
  },
  "& .MuiOutlinedInput-root": {
    "& fieldset": {},
    "&:hover fieldset": {},
    notched: false,
    backgroundColor: AppConfigurations.backgroundThree
  },
  "& .MuiSelect-root": {
    backgroundColor: AppConfigurations.backgroundThree,
  },
  label: {
    transform: "translate(10px, 0px)",
  },
}


export const TopMenuInput = styled(TextField)(({ }) => topMenuInputStyle);
export const BottomMenuInput = styled(TextField)(({ }) => bottomMenuInputStyle);

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

function EquipmentMenuOfOneSlot(
  id: number,
  slotName: string,
  equipmentsAvailableInSlot: Equipment[],
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  LANGUAGE_TEXTS: TextDictionary,
) {
  let totalState = totalEquipmentState.equipmentDatas[id];
  let key = `${slotName}-${id}-equipment`;
  let slotEquipmentId = totalState.itemSet[slotNameToSlotIndex(slotName)];
  let currentEquipmentId =
    slotEquipmentId === undefined ? EMPTY_EQUIPMENT_ID : slotEquipmentId;

  if (
    slotName === FINGER1_SLOT_EN_TEXT &&
    totalState.itemSet[slotNameToSlotIndex(FINGER2_SLOT_EN_TEXT)] !== -1
  ) {
    let ring2 = totalState.itemSet[slotNameToSlotIndex(FINGER2_SLOT_EN_TEXT)];
    let ring2Equipment = EQUIPMENT_DATABASE_BY_ID.get(ring2);

    if (
      ring2Equipment !== undefined &&
      (!ring2Equipment.name.includes("Archeo") && !ring2Equipment.name.includes("Ceremonial"))
    ) {
      equipmentsAvailableInSlot = equipmentsAvailableInSlot.filter(
        (equipment) =>
          equipment.id !==
          totalState.itemSet[slotNameToSlotIndex(FINGER2_SLOT_EN_TEXT)]
      );
    }
  }
  if (
    slotName === FINGER2_SLOT_EN_TEXT &&
    totalState.itemSet[slotNameToSlotIndex(FINGER1_SLOT_EN_TEXT)] !== -1
  ) {
    let ring1 = totalState.itemSet[slotNameToSlotIndex(FINGER1_SLOT_EN_TEXT)];
    let ring1Equipment = EQUIPMENT_DATABASE_BY_ID.get(ring1);
    if (
      ring1Equipment !== undefined &&
      (!ring1Equipment.name.includes("Archeo") && !ring1Equipment.name.includes("Ceremonial"))
    ) {
      equipmentsAvailableInSlot = equipmentsAvailableInSlot.filter(
        (equipment) =>
          equipment.id !==
          totalState.itemSet[slotNameToSlotIndex(FINGER1_SLOT_EN_TEXT)]
      );
    }
  }

  let currentEquipment = EQUIPMENT_DATABASE_BY_ID.get(currentEquipmentId);
  const updateEquipmentState = (value: string) => {
    let newEquipmentId = parseInt(value);
    let newTotalData = { ...totalEquipmentState };
    let newData = newTotalData.equipmentDatas[id];
    let newSet = [...totalState.itemSet];
    newSet[slotNameToSlotIndex(slotName)] = newEquipmentId;
    newData.itemSet = newSet;

    let newGearSetMaterias = [...totalState.gearSetMaterias];

    let materiaSlotCount = 0;
    currentEquipment = EQUIPMENT_DATABASE_BY_ID.get(newEquipmentId);
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

  let slotLabel = convertToSlotText(slotName, LANGUAGE_TEXTS);
  if (currentEquipmentId !== -1) {
    slotLabel = "";
  }

  return (
    <>
      <CustomFormControl fullWidth>
        <TopMenuInput
          select
          id={key}
          value={totalState.itemSet[slotNameToSlotIndex(slotName)]}
          key={key}
          label={slotLabel}
          onChange={(e: React.ChangeEvent<HTMLInputElement>): void => updateEquipmentState(e.target.value)}
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
          {equipmentsAvailableInSlot.map((equipment) => {
            return EquipmentMenuItem(
              id,
              equipment,
              totalState.mainPlayerJobAbbrev,
              LANGUAGE_TEXTS
            );
          })}
          <Divider />
          {slotName !== WEAPON_SLOT_EN_TEXT ? (
            <EquipmentMenu value={-1} key={`${id}_${slotLabel}_empty`}>
              <Box
                display="flex"
                height="100%"
                alignItems="center"
                justifyContent="flex-end"
              >
                <Typography
                  variant="body2"
                  color="white"
                  sx={{ fontSize: AppConfigurations.body1FontSize }}
                >
                  {LANGUAGE_TEXTS.EMPTY_TEXT}
                </Typography>
              </Box>
            </EquipmentMenu>
          ) : (
            <Box></Box>
          )}
        </TopMenuInput>
      </CustomFormControl>
      <MateriaBox>
        {MateriaInputTable(
          id,
          slotName,
          currentEquipment,
          totalEquipmentState,
          setTotalEquipmentState,
          LANGUAGE_TEXTS
        )}
      </MateriaBox>

      <EquipmentStatBox>
        {currentEquipment === undefined ? (
          <></>
        ) : (
          EquipmentSubStatTable(
            currentEquipment,
            totalState.gearSetMaterias[slotNameToSlotIndex(slotName)],
            LANGUAGE_TEXTS
          )
        )}
      </EquipmentStatBox>
    </>
  );
}

const LONG_ITEM_WIDTH = "96%";
const SHORT_ITEM_WIDTH = "46%";

const EQUIPMENT_ITEM_WIDTH = (itemCount: number) => {
  return itemCount > 1
    ? {
      xs: LONG_ITEM_WIDTH,
      sm: LONG_ITEM_WIDTH,
      md: LONG_ITEM_WIDTH,
      lg: LONG_ITEM_WIDTH,
      xl: LONG_ITEM_WIDTH,
    }
    : {
      xs: LONG_ITEM_WIDTH,
      sm: LONG_ITEM_WIDTH,
      md: LONG_ITEM_WIDTH,
      lg: SHORT_ITEM_WIDTH,
      xl: SHORT_ITEM_WIDTH,
    };
};

export function EquipmentSelectionMenu(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  LANGUAGE_TEXTS: TextDictionary,
  hasTimeInput: boolean = true,
  isDouble: boolean = false
) {
  let xs = 12;
  let mainCharacterJobAbbrev =
    totalEquipmentState.equipmentDatas[id].mainPlayerJobAbbrev;
  let inputCount = isDouble ? 2 : 1;

  return (
    <EquipmentGridContainer container>
      <EquipmentGridItemBox
        key={`${id}_JobSelectionItemBox`}
        sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
      >
        <InputEquipmentBox item xs={xs} key={`Job_${id}`}>
          {MainPlayerJobSelection(
            id,
            totalEquipmentState,
            setTotalEquipmentState,
            LANGUAGE_TEXTS
          )}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
      <EquipmentGridItemBox
        marginBottom={1}
        key={`${id}_RaceItemBox`}
        sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
      >
        <InputEquipmentBox item xs={xs} key="Race">
          {MainPlayerRaceSelection(
            id,
            totalEquipmentState,
            setTotalEquipmentState,
            LANGUAGE_TEXTS
          )}
        </InputEquipmentBox>
      </EquipmentGridItemBox>

      {
        getEquipmentSlotsOfJob(mainCharacterJobAbbrev).map((slotName) => {
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
              key={`${id}_equipment_${slotName}_itembox`}
              sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
            >
              <InputEquipmentBox
                item
                xs={xs}
                key={`${id}_${slotName} _equipmentselection`}
              >
                {EquipmentMenuOfOneSlot(
                  id,
                  slotName,
                  equipmentsAvailableInSlot,
                  totalEquipmentState,
                  setTotalEquipmentState,
                  LANGUAGE_TEXTS
                )}
              </InputEquipmentBox>
            </EquipmentGridItemBox>
          );
        })
      }
      <EquipmentGridItemBox
        key={`food_selectionbox_${id} `}
        sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
      >
        <InputEquipmentBox item xs={xs} key={`food_${id} `}>
          {FoodSelection(id, totalEquipmentState, setTotalEquipmentState, LANGUAGE_TEXTS)}
        </InputEquipmentBox>
      </EquipmentGridItemBox>

      <EquipmentGridItemBox
        key={`pot_selectionbox_${id} `}
        sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
      >
        <InputEquipmentBox item xs={xs} key={`pot_${id} `}>
          {PotSelection(id, totalEquipmentState, setTotalEquipmentState, LANGUAGE_TEXTS.POT_LABEL_TEXT, LANGUAGE_TEXTS.USE_POT_TEXT, LANGUAGE_TEXTS.NO_POT_TEXT)}
        </InputEquipmentBox>
      </EquipmentGridItemBox>

      {PartnerSelectionMenu(id, totalEquipmentState, setTotalEquipmentState, inputCount, LANGUAGE_TEXTS)}

      {
        hasTimeInput ? (
          <EquipmentGridItemBox
            key={`${id} _timeinput`}
            sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
          >
            <InputEquipmentBox item xs={xs}>
              {SimulationUpperInputTimeTextBox(
                LANGUAGE_TEXTS.TIME_INPUT_LABEL_TEXT,
                totalEquipmentState,
                setTotalEquipmentState,
              )}
            </InputEquipmentBox>
          </EquipmentGridItemBox>
        ) : (
          <Box></Box>
        )
      }
    </EquipmentGridContainer >
  );
}


export function BestPartnerInputMenu(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  LANGUAGE_TEXTS: TextDictionary,
  isDouble: boolean = false
) {
  let xs = 12;
  let inputCount = isDouble ? 2 : 1;

  return (
    <EquipmentGridContainer container>
      <EquipmentGridItemBox
        key={`${id} _JobSelectionItemBox`}
        sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
      >
        <InputEquipmentBox item xs={xs} key={`Job_${id} `}>
          {
            MainPlayerJobSelectionOnlyBuffJobs(
              id,
              totalEquipmentState,
              setTotalEquipmentState,
              LANGUAGE_TEXTS
            )
          }
        </InputEquipmentBox>
      </EquipmentGridItemBox>
      <EquipmentGridItemBox
        key={`${id}_timeinput`}
        sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
      >
        <InputEquipmentBox item xs={xs}>
          {MainPlayerGcdSelection(
            0,
            totalEquipmentState,
            setTotalEquipmentState,
            LANGUAGE_TEXTS.SPEED_LABEL_TEXT
          )}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
      <EquipmentGridItemBox
        key={`${id} _timeinput`}
        sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
      >
        <InputEquipmentBox item xs={xs}>
          {SimulationUpperInputTimeTextBox(
            LANGUAGE_TEXTS.TIME_INPUT_LABEL_TEXT,
            totalEquipmentState,
            setTotalEquipmentState,
          )}
        </InputEquipmentBox>
      </EquipmentGridItemBox>
    </EquipmentGridContainer>
  );
}

function PartnerSelectionMenu(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  inputCount: number,
  LANGUAGE_TEXTS: TextDictionary
) {
  let mainPlayerJobAbbrev =
    totalEquipmentState.equipmentDatas[id].mainPlayerJobAbbrev;

  if (mainPlayerJobAbbrev === AST_EN_NAME) {
    return (
      <>
        <EquipmentGridItemBox
          key={`partner1_${id} _grid`}
          sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
        >
          <InputEquipmentBox item xs={12} key={`partner1_${id} `}>
            {Partner1Selection(
              id,
              totalEquipmentState,
              setTotalEquipmentState,
              LANGUAGE_TEXTS.AST_MELEE_PARTNER_TEXT,
              LANGUAGE_TEXTS.PARTY_MEMBER_LABEL_TEXT,
              LANGUAGE_TEXTS.EMPTY_TEXT
            )}
          </InputEquipmentBox>
        </EquipmentGridItemBox>

        <EquipmentGridItemBox
          key={`partner2_${id} _grid`}
          sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
        >
          <InputEquipmentBox item xs={12} key={`partner2_${id} `}>
            {Partner2Selection(
              id,
              totalEquipmentState,
              setTotalEquipmentState,
              LANGUAGE_TEXTS.AST_RANGED_PARTNER_TEXT,
              LANGUAGE_TEXTS.PARTY_MEMBER_LABEL_TEXT,
              LANGUAGE_TEXTS.EMPTY_TEXT
            )}
          </InputEquipmentBox>
        </EquipmentGridItemBox>
      </>
    );
  } else if (mainPlayerJobAbbrev === DNC_EN_NAME) {
    return (
      <EquipmentGridItemBox
        key={`partner1_${id} _grid`}
        sx={{ width: EQUIPMENT_ITEM_WIDTH(inputCount) }}
      >
        <InputEquipmentBox item xs={12} key={`partner1_${id} `}>
          {Partner1Selection(
            id,
            totalEquipmentState,
            setTotalEquipmentState,
            LANGUAGE_TEXTS.DNC_PARTNER_TEXT,
            LANGUAGE_TEXTS.PARTY_MEMBER_LABEL_TEXT,
            LANGUAGE_TEXTS.EMPTY_TEXT
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
  setTotalEquipmentState: Function,
  LANGUAGE_TEXTS: TextDictionary,
) {
  let totalState = totalEquipmentState.equipmentDatas[id];

  let foodLabel = LANGUAGE_TEXTS.FOOD_SLOT_TEXT;
  if (totalState.foodId !== -1) {
    foodLabel = "";
  }

  const updateFoodState = (value: string) => {
    let newFoodId = parseInt(value);
    let newTotalData = { ...totalEquipmentState };
    newTotalData.equipmentDatas[id].foodId = newFoodId;

    updateOnePlayerPower(id, newTotalData, setTotalEquipmentState);
  };

  let key = `food - ${id} `;

  return (
    <>
      <CustomFormControl fullWidth>
        <TopMenuInput
          select
          id={key}
          value={totalState.foodId}
          key={key}
          label={foodLabel}
          onChange={(e: React.ChangeEvent<HTMLInputElement>): void => updateFoodState(e.target.value)}
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
          {ALL_FOODS.map((food) => {
            return FoodMenuItem(food, LANGUAGE_TEXTS);
          })}
          <MenuItem value={-1}>
            <Box
              display="flex"
              height="100%"
              alignItems="center"
              justifyContent="flex-end"
            >
              <Typography variant="body2" color="white" sx={{ fontSize: AppConfigurations.body1FontSize }}>
                {LANGUAGE_TEXTS.EMPTY_TEXT}
              </Typography>
            </Box>
          </MenuItem>
        </TopMenuInput>
      </CustomFormControl >
    </>
  );
}

function PotSelection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  potLabelText: string,
  usePotText: string,
  noPotText: string
) {
  let totalState = totalEquipmentState.equipmentDatas[id];
  let label = potLabelText;

  const updateUsePot = (value: string) => {
    let newState = { ...totalEquipmentState };
    let usePot = parseInt(value);

    newState.equipmentDatas[id].usePot = usePot;

    setTotalEquipmentState(newState);
  };

  let key = `pot - ${id} `;

  return (
    <>
      <CustomFormControl fullWidth>
        <TopMenuInput
          select
          id={key}
          value={totalState.usePot}
          key={key}
          label={label}
          onChange={(e: React.ChangeEvent<HTMLInputElement>): void => updateUsePot(e.target.value)}
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
          <MenuItem value={USE_POT_VAL}>
            <Box
              display="flex"
              alignItems="center"
              height="100%"
              justifyContent="flex-end"
            >
              <Typography variant="body2" color="white" sx={{ fontSize: AppConfigurations.body1FontSize }}>
                {usePotText}
              </Typography>
            </Box>
          </MenuItem>
          <MenuItem value={NO_POT_VAL}>
            <Box
              display="flex"
              alignItems="center"
              height="100%"
              justifyContent="flex-end"
            >
              <Typography variant="body2" color="white" sx={{ fontSize: AppConfigurations.body1FontSize }}>
                {noPotText}
              </Typography>
            </Box>
          </MenuItem>
        </TopMenuInput>
      </CustomFormControl>
    </>
  );
}
