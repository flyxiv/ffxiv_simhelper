import { Equipment } from "src/types/ffxivdatabase/Equipment";
import { ItemSet } from "src/types/ffxivdatabase/ItemSet";
import { Box, Typography } from "@mui/material";
import { getEquipmentIconDirectory } from "../icon/equipmenticon/EquipmentIconFactory";

export function EquipmentItem(equipment: Equipment, jobAbbrev: string) {
  return (
    <Box display="flex" justifyContent="left" alignContent={"center"}>
      <Box marginRight={1}>
        <img
          src={getEquipmentIconDirectory(
            equipment.slotName,
            jobAbbrev,
            equipment.name
          )}
          alt={getEquipmentIconDirectory(
            equipment.slotName,
            jobAbbrev,
            equipment.name
          )}
          width={50}
          height={50}
          style={{ verticalAlign: "middle" }}
        />
      </Box>
      <Box> </Box>
      <Box marginLeft={1}>
        <Typography variant="body2" alignContent={"center"} fontSize={12}>
          {equipment.name}
        </Typography>
      </Box>
    </Box>
  );
}
