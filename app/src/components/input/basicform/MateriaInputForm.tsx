import { Equipment } from "src/types/ffxivdatabase/Equipment";
import { Materia } from "src/types/ffxivdatabase/Materia";
import { getPossibleMateriasForEquipmentSlot } from "src/types/ffxivdatabase/Materia";
import { SelectChangeEvent, InputLabel, Select, MenuItem } from "@mui/material";
import { CustomFormControl } from "./BasicInputForm";
import {
  toMateriaKey,
  updateMateriaList,
  EMPTY_MATERIA,
} from "src/types/ffxivdatabase/Materia";
import { MateriaMenuItem } from "src/components/items/MateriaMenuItem";

export function MateriaInputTable(
  slotName: string,
  equipment: Equipment | undefined,
  gearSetMaterias: Map<string, Materia[]>,
  setGearSetMaterias: Function
) {
  let materiasInSlot = gearSetMaterias.get(slotName);
  if (materiasInSlot === undefined) {
    return <></>;
  }
  if (equipment === undefined) {
    return <></>;
  }

  return materiasInSlot.map((materia, materiaSlot) => {
    return SingleMateriaMenu(
      equipment,
      materiasInSlot,
      gearSetMaterias,
      setGearSetMaterias,
      materiaSlot
    );
  });
}

function SingleMateriaMenu(
  equipment: Equipment,
  materias: Materia[] | undefined,
  gearSetMaterias: Map<string, Materia[]>,
  setGearSetMaterias: Function,
  materiaSlot: number
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
    let materiasOfSlot = gearSetMaterias.get(equipment.slotName);
    if (materiasOfSlot === undefined) {
      return;
    }

    updateMateriaList(e.target.value, equipment, materiasOfSlot, materiaSlot);
    let newGearSetMaterias = new Map(gearSetMaterias);

    newGearSetMaterias.set(equipment.slotName, materiasOfSlot);
    setGearSetMaterias(newGearSetMaterias);
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
        key={key}
        label={key}
        onChange={updateMateria}
      >
        {possibleMaterias.map((materiaKey) => {
          return MateriaMenuItem(materiaKey, materias[materiaSlot]);
        })}
        <MenuItem value={"empty"}>Empty</MenuItem>
      </Select>
    </CustomFormControl>
  );
}
