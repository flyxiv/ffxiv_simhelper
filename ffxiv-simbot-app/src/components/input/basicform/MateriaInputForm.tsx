import { Equipment } from "../../../types/ffxivdatabase/Equipment";
import { Materia } from "../../../types/ffxivdatabase/Materia";
import { getPossibleMateriasForEquipmentSlot } from "../../../types/ffxivdatabase/Materia";
import {
  SelectChangeEvent,
  Select,
  MenuItem,
  styled,
  Divider,
  Box,
  Typography,
} from "@mui/material";
import { CustomFormControl } from "./BasicInputForm";
import {
  toMateriaKey,
  updateMateriaList,
} from "../../../types/ffxivdatabase/Materia";
import { MateriaMenuItem } from "../../../components/items/MateriaMenuItem";
import {
  slotNameToSlotIndex,
  updateOnePlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { MenuItemStyle } from "../../../components/items/Styles";
import { AppConfigurations } from "../../../Themes";
import { EquipmentInput } from "../../../types/EquipmentInput";
import { EMPTY_TEXT } from "../../../const/languageTexts";

const MateriaMenu = styled(MenuItem)`
  ${MenuItemStyle}
`;

const MATERIA_MENU_HEIGHT = "6vh";

export function MateriaInputTable(
  id: number,
  slotName: string,
  equipment: Equipment | undefined,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function
) {
  let totalState = totalEquipmentState.equipmentDatas[id];
  let materiasInSlot =
    totalState.gearSetMaterias[slotNameToSlotIndex(slotName)];
  if (equipment === undefined) {
    return <></>;
  }

  return (
    <Box display="flex" height={MATERIA_MENU_HEIGHT} width="100%">
      {materiasInSlot.map((_, materiaSlot) => {
        return SingleMateriaMenu(
          id,
          equipment,
          materiasInSlot,
          materiaSlot,
          slotName,
          totalEquipmentState,
          setTotalEquipmentState
        );
      })}
    </Box>
  );
}

function SingleMateriaMenu(
  id: number,
  equipment: Equipment,
  materias: Materia[] | undefined,
  materiaSlot: number,
  slotName: string,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function
) {
  if (materias === undefined) {
    return <></>;
  }
  let possibleMaterias = getPossibleMateriasForEquipmentSlot(
    equipment,
    materiaSlot
  );
  let key = `${id}-${equipment.slotName}-${equipment.id}-materia-${materiaSlot}`;

  let updateMateria = (e: SelectChangeEvent<string>) => {
    let newTotalEquipmentState = { ...totalEquipmentState };
    let newData = newTotalEquipmentState.equipmentDatas[id];

    let materiasOfSlot = newData.gearSetMaterias[slotNameToSlotIndex(slotName)];

    updateMateriaList(e.target.value, equipment, materiasOfSlot, materiaSlot);
    let newGearSetMaterias = [...newData.gearSetMaterias];

    newGearSetMaterias[slotNameToSlotIndex(slotName)] = materiasOfSlot;
    newData.gearSetMaterias = newGearSetMaterias;
    updateOnePlayerPower(id, newTotalEquipmentState, setTotalEquipmentState);
  };

  let materiaValue = toMateriaKey(materias[materiaSlot]);

  return (
    <CustomFormControl fullWidth>
      <Select
        labelId={key}
        id={key}
        value={materiaValue}
        key={`${key}`}
        label={key}
        onChange={updateMateria}
        MenuProps={{
          PaperProps: {
            sx: {
              backgroundColor: AppConfigurations.backgroundThree,
            },
          },
        }}
        sx={{ height: "100%" }}
      >
        {possibleMaterias.map((materiaKey) => {
          return MateriaMenuItem(
            slotName,
            materiaSlot,
            materiaKey,
            materias[materiaSlot]
          );
        })}
        <Divider />
        <MateriaMenu value={"empty"}>
          <Box display="flex" justifyContent={"center"} alignItems={"center"} height="100%">
            <Typography align="center">{EMPTY_TEXT}</Typography>
          </Box>
        </MateriaMenu>
      </Select>
    </CustomFormControl>
  );
}
