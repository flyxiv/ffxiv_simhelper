import { MenuItem } from "@mui/material";
import { FoodItem } from "./FoodItem";
import { Food } from "../../types/ffxivdatabase/Food";

export function FoodMenuItem(food: Food) {
  return (
    <MenuItem value={food.id} key={`${food.name}_menuitem`}>
      {FoodItem(food)}
    </MenuItem>
  );
}
