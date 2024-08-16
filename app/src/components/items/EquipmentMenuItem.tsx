import { MenuItem, styled } from "@mui/material";
import { Equipment } from "src/types/ffxivdatabase/Equipment";
import { EquipmentItem } from "./EquipmentItem";
import { MenuItemStyle } from "./Styles";

const EquipmentMenu = styled(MenuItem)`
  ${MenuItemStyle}
`;

export function EquipmentMenuItem(equipment: Equipment, jobAbbrev: string) {
  return (
    <EquipmentMenu value={equipment.id} key={`${equipment.name}_select`}>
      {EquipmentItem(equipment, jobAbbrev)}
    </EquipmentMenu>
  );
}
