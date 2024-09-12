import { MenuItem, styled } from "@mui/material";
import { MateriaItem } from "./MateriaItem";
import { Materia } from "../../types/ffxivdatabase/Materia";
import { MenuItemStyle } from "./Styles";

const MateriaMenu = styled(MenuItem)`
  ${MenuItemStyle}
`;

export function MateriaMenuItem(
  slotName: string,
  materiaSlot: number,
  materiaKey: string,
  currentlyEquippedMateria: Materia
) {
  return (
    <MateriaMenu
      value={materiaKey}
      key={`${slotName}_${materiaSlot}_${materiaKey}`}
      sx={{ align: "center" }}
    >
      {MateriaItem(materiaKey, currentlyEquippedMateria)}
    </MateriaMenu>
  );
}
