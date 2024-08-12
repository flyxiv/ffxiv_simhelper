import { MenuItem } from "@mui/material";
import { MateriaItem } from "./MateriaItem";
import { Materia } from "src/types/ffxivdatabase/Materia";

export function MateriaMenuItem(
  materiaKey: string,
  currentlyEquippedMateria: Materia
) {
  return (
    <MenuItem value={materiaKey}>
      {MateriaItem(materiaKey, currentlyEquippedMateria)}
    </MenuItem>
  );
}
