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
import { CharacterEquipmentsData } from "../../../types/ffxivdatabase/PlayerPower";
import {
  slotNameToSlotIndex,
  updatePlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { MenuItemStyle } from "../../../components/items/Styles";
import { ColorConfigurations } from "../../../Themes";

const MateriaMenu = styled(MenuItem)`
  ${MenuItemStyle}
`;

export function MateriaInputTable(
  slotName: string,
  equipment: Equipment | undefined,
  data: CharacterEquipmentsData,
  setData: Function
) {
  let materiasInSlot = data.gearSetMaterias[slotNameToSlotIndex(slotName)];
  if (equipment === undefined) {
    return <></>;
  }

  return materiasInSlot.map((_, materiaSlot) => {
    return SingleMateriaMenu(
      equipment,
      materiasInSlot,
      materiaSlot,
      slotName,
      data,
      setData
    );
  });
}

function SingleMateriaMenu(
  equipment: Equipment,
  materias: Materia[] | undefined,
  materiaSlot: number,
  slotName: string,
  data: CharacterEquipmentsData,
  setData: Function
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
      data.gearSetMaterias[slotNameToSlotIndex(equipment.slotName)];

    updateMateriaList(e.target.value, equipment, materiasOfSlot, materiaSlot);
    let newGearSetMaterias = [...data.gearSetMaterias];
    let newData = { ...data };

    newGearSetMaterias[slotNameToSlotIndex(equipment.slotName)] =
      materiasOfSlot;
    newData.gearSetMaterias = newGearSetMaterias;
    updatePlayerPower(newData, setData);
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
