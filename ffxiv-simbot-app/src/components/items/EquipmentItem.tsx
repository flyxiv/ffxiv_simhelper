import {
  Equipment,
  equipmentStatDescriptionString,
} from "../../types/ffxivdatabase/Equipment";
import { Box, Typography } from "@mui/material";
import { getEquipmentIconDirectory } from "../icon/equipmenticon/EquipmentIconFactory";
import { AppConfigurations } from "../../Themes";

export const EQUIPMENT_FOOD_SIZE = "4vh";

export function EquipmentItem(
  equipment: Equipment,
  jobAbbrev: string,
  idCount: number
) {
  return (
    <Box display="flex" alignContent={"center"} height="5vh">
      <Box marginRight={0.3}>
        <Box
          component={"img"}
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
          sx={{
            width: EQUIPMENT_FOOD_SIZE,
            height: EQUIPMENT_FOOD_SIZE,
          }}
          style={{ verticalAlign: "middle" }}
        />
      </Box>

      <Box
        display="flex"
        flexDirection="column"
        justifyContent={"center"}
        sx={{ height: EQUIPMENT_FOOD_SIZE }}
        marginLeft={1}
      >
        <Typography
          variant="body2"
          justifyContent={"center"}
          sx={{
            fontSize: AppConfigurations.body1FontSize,
          }}
          color="white"
          align="left"
        >
          {`${equipment.name} (${equipment.itemLevel})`}
        </Typography>
        <Typography
          variant="body2"
          sx={{
            fontSize: AppConfigurations.body2FontSize,
          }}
          color="white"
          align="left"
        >
          {equipmentStatDescriptionString(equipment)}
        </Typography>
      </Box>
    </Box>
  );
}
