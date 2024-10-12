import { MenuItem } from "@mui/material";
import { FoodItem } from "./FoodItem";
import { Food } from "../../types/ffxivdatabase/Food";
import { TextDictionary } from "../../const/languageTexts";

export function FoodMenuItem(food: Food, LANGUAGE_TEXTS: TextDictionary) {
  return (
    <MenuItem value={food.id} key={`${food.name}_menuitem`}>
      {FoodItem(food, LANGUAGE_TEXTS)}
    </MenuItem>
  );
}
