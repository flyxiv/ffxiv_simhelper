import {
  Equipment,
  equipmentStatDescriptionString,
} from "src/types/ffxivdatabase/Equipment";
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
          width={30}
          height={30}
          style={{ verticalAlign: "middle" }}
        />
      </Box>
      <Box>
        <Box marginLeft={1}>
          <Typography
            variant="body2"
            alignContent={"center"}
            fontSize={12}
            color="white"
          >
            {equipment.name}
          </Typography>
        </Box>
        <Box>
          <Typography
            variant="body2"
            alignContent={"center"}
            fontSize={10}
            color="white"
          >
            {equipmentStatDescriptionString(equipment)}
          </Typography>
        </Box>
      </Box>
    </Box>
  );
}
