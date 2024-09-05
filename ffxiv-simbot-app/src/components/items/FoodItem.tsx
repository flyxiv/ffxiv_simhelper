import { Box, Typography } from "@mui/material";
import { Food, foodStatDescriptionString } from "../../types/ffxivdatabase/Food";
import { getFoodIconPath } from "../icon/foodicon/FoodIconFactory";

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
      alignContent={"center"}
      height="5vh"
    >
      <Box marginRight={1}>
        <Box component={"img"}
          src={getFoodIconPath(food.name)}
          alt={getFoodIconPath(food.name)}
          sx={{ width: imageSize, height: imageSize }}
        />
      </Box>
      <Box display="flex" flexDirection="column" justifyContent={"center"} sx={{ height: imageSize }} marginLeft={1}>
        <Typography
          variant="body2"
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
          {food.name}
        </Typography>
        <Box>
          <Typography
            variant="body2"
            align="left"
            color="white"
            sx={{
              fontSize: {
                xs: 8,
                sm: 10,
                md: 10,
                lg: 12,
                xl: 12
              }
            }}
          >
            {foodStatDescriptionString(food)}
          </Typography>
        </Box>
      </Box>
    </Box >
  );
}
