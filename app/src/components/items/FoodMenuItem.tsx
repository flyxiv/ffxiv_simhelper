import { MenuItem } from "@mui/material";
import { FoodItem } from "./FoodItem";
import { Food } from "src/types/ffxivdatabase/Food";

export function FoodMenuItem(food: Food) {
  return <MenuItem value={food.id}>{FoodItem(food)}</MenuItem>;
}
