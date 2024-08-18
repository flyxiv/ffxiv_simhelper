import { Box, Typography } from "@mui/material";
import { Food, foodStatDescriptionString } from "../../types/ffxivdatabase/Food";
import { getFoodIconPath } from "../icon/foodicon/FoodIconFactory";

export function FoodItem(food: Food) {
  return (
    <Box display="flex" justifyContent="left" alignContent={"center"}>
      <Box marginRight={1}>
        <img
          src={getFoodIconPath(food.name)}
          alt={getFoodIconPath(food.name)}
          width={50}
          height={50}
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
            {food.name}
          </Typography>
        </Box>
        <Box>
          <Typography
            variant="body2"
            alignContent={"center"}
            fontSize={10}
            color="white"
          >
            {foodStatDescriptionString(food)}
          </Typography>
        </Box>
      </Box>
    </Box>
  );
}
