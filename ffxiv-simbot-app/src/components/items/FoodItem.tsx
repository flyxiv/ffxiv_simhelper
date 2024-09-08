import { Box, Typography } from "@mui/material";
import { Food, foodStatDescriptionString } from "../../types/ffxivdatabase/Food";
import { getFoodIconPath } from "../icon/foodicon/FoodIconFactory";
import { EQUIPMENT_FOOD_SIZE } from "./EquipmentItem";
import { AppConfigurations } from "../../Themes";

export function FoodItem(food: Food) {
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
      alignItems={"center"}
      height="5vh"
    >
      <Box marginRight={1}>
        <Box component={"img"}
          src={getFoodIconPath(food.name)}
          alt={getFoodIconPath(food.name)}
          sx={{ width: EQUIPMENT_FOOD_SIZE, height: EQUIPMENT_FOOD_SIZE }}
        />
      </Box>
      <Box display="flex" flexDirection="column" justifyContent={"center"} sx={{ height: imageSize }} marginLeft={1}>
        <Typography
          variant="body2"
          sx={{
            fontSize: AppConfigurations.body1FontSize
          }}
          color="white"
          align="left"
        >
          {`${food.name} (${food.itemLevel})`}
        </Typography>
        <Box>
          <Typography
            variant="body2"
            align="left"
            color="white"
            sx={{
              fontSize: AppConfigurations.body2FontSize
            }}
          >
            {foodStatDescriptionString(food)}
          </Typography>
        </Box>
      </Box>
    </Box >
  );
}
