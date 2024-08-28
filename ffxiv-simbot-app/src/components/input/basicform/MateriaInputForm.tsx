import { Equipment } from "src/types/ffxivdatabase/Equipment";
import { Materia } from "src/types/ffxivdatabase/Materia";
import { getPossibleMateriasForEquipmentSlot } from "../../../types/ffxivdatabase/Materia";
import {
  SelectChangeEvent,
  InputLabel,
  Select,
  MenuItem,
  styled,
  Divider,
} from "@mui/material";
import { CustomFormControl } from "./BasicInputForm";
import {
  toMateriaKey,
  updateMateriaList,
  EMPTY_MATERIA,
} from "../../../types/ffxivdatabase/Materia";
import { MateriaMenuItem } from "../../../components/items/MateriaMenuItem";
import {
  slotNameToSlotIndex,
  updatePlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { MenuItemStyle } from "../../../components/items/Styles";
import { ColorConfigurations } from "../../../Themes";
import { SingleEquipmentInputSaveState } from "../../../types/SingleEquipmentInputSaveState";

const MateriaMenu = styled(MenuItem)`
  ${MenuItemStyle}
`;

export function MateriaInputTable(
  slotName: string,
  equipment: Equipment | undefined,
  totalState: SingleEquipmentInputSaveState,
  setTotalState: Function
) {
  let materiasInSlot = totalState.gearSetMaterias[slotNameToSlotIndex(slotName)];
  if (equipment === undefined) {
    return <></>;
  }

  return materiasInSlot.map((_, materiaSlot) => {
    return SingleMateriaMenu(
      equipment,
      materiasInSlot,
      materiaSlot,
      slotName,
      totalState,
      setTotalState
    );
  });
}

function SingleMateriaMenu(
  equipment: Equipment,
  materias: Materia[] | undefined,
  materiaSlot: number,
  slotName: string,
  totalState: SingleEquipmentInputSaveState,
  setTotalState: Function
) {
  if (materias === undefined) {
    return <></>;
  }
  let possibleMaterias = getPossibleMateriasForEquipmentSlot(
    equipment,
    materiaSlot
  );
  let key = `${equipment.slotName}-${equipment.id}-materia-${materiaSlot}`;

  let updateMateria = (e: SelectChangeEvent<string>) => {
    let materiasOfSlot =
      totalState.gearSetMaterias[slotNameToSlotIndex(slotName)];

    updateMateriaList(e.target.value, equipment, materiasOfSlot, materiaSlot);
    let newGearSetMaterias = [...totalState.gearSetMaterias];
    let newData = { ...totalState };

    newGearSetMaterias[slotNameToSlotIndex(slotName)] =
      materiasOfSlot;
    newData.gearSetMaterias = newGearSetMaterias;
    updatePlayerPower(newData, setTotalState);
  };

  let materiaValue = toMateriaKey(materias[materiaSlot]);

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="MateriaSelect">
        {materiaValue === toMateriaKey(EMPTY_MATERIA)
          ? `Mat ${materiaSlot + 1}`
          : ""}
      </InputLabel>
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
              backgroundColor: ColorConfigurations.backgroundThree,
            },
          },
        }}
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
        <MateriaMenu value={"empty"}> </MateriaMenu>
      </Select>
    </CustomFormControl>
  );
}
