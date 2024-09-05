import {
  Equipment,
  equipmentStatDescriptionString,
} from "../../types/ffxivdatabase/Equipment";
import { Box, Typography } from "@mui/material";
import { getEquipmentIconDirectory } from "../icon/equipmenticon/EquipmentIconFactory";

export function EquipmentItem(equipment: Equipment, jobAbbrev: string) {
  let imageSize = {
    xs: 30,
    sm: 30,
    md: 40,
    lg: 50,
    xl: 65,
  }
  return (
    <Box
      display="flex"
      alignContent={"center"}
      height="5vh"
    >
      <Box marginRight={1}>
        <Box component={"img"}
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
            width: imageSize,
            height: imageSize
          }}
          style={{ verticalAlign: "middle" }}
        />
      </Box>
      <Box display="flex" flexDirection="column" justifyContent={"center"} sx={{ height: imageSize }} marginLeft={1}>
        <Typography
          variant="body2"
          justifyContent={"center"}
          sx={{
            fontSize: {
              xs: 10,
              sm: 12,
              md: 14,
              lg: 16,
              xl: 18
            }
          }}
          color="white"
          align="left"
        >
          {equipment.name}
        </Typography>
        <Typography variant="body2" sx={{
          fontSize: {
            xs: 8,
            sm: 10,
            md: 10,
            lg: 12,
            xl: 12
          }
        }} color="white" align="left">
          {equipmentStatDescriptionString(equipment)}
        </Typography>
      </Box>
    </Box >
  );
}
