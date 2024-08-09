import { MenuItem } from "@mui/material";
import { Equipment } from "src/types/ffxivdatabase/Equipment";
import { EquipmentItem } from "./EquipmentItem";

export function EquipmentMenuItem(equipment: Equipment, jobAbbrev: string) {
  return (
    <MenuItem value={equipment.id}>
      {EquipmentItem(equipment, jobAbbrev)}
    </MenuItem>
  );
}
