import { MenuItem, styled } from "@mui/material";
import { Equipment } from "../../types/ffxivdatabase/Equipment";
import { EquipmentItem } from "./EquipmentItem";
import { MenuItemStyle } from "./Styles";

const EquipmentMenu = styled(MenuItem)`
  ${MenuItemStyle}
`;

export function EquipmentMenuItem(id: number, equipment: Equipment, jobAbbrev: string) {
  return (
    <EquipmentMenu value={equipment.id} key={`${id}_${equipment.name}_select`}>
      {EquipmentItem(equipment, jobAbbrev)}
    </EquipmentMenu>
  );
}
