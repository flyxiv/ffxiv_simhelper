import totalFoodJson from "../../assets/data/food_data.json";
import { TextDictionary } from "../../const/languageTexts";

export const FOOD_MIN_ILVL = 710;
export const FOOD_MAX_ILVL = 740;

export const totalFood: Array<Food> = [];
totalFoodJson.forEach((element) => {
  totalFood.push({
    id: element.id,
    name: element.name,
    itemLevel: element.itemLevel,
    criticalStrike: element.criticalStrike,
    directHit: element.directHit,
    determination: element.determination,
    skillSpeed: element.skillSpeed,
    spellSpeed: element.spellSpeed,
    tenacity: element.tenacity,
    piety: element.piety,
  });
});

export const FOOD_DATABASE = readFoodData(FOOD_MIN_ILVL, FOOD_MAX_ILVL);
export const ALL_FOODS = Array.from(FOOD_DATABASE.values()).sort(
  (a, b) => b.itemLevel - a.itemLevel
);


export interface Food {
  id: number;
  name: string;
  itemLevel: number;

  criticalStrike: number;
  directHit: number;
  determination: number;
  skillSpeed: number;
  spellSpeed: number;
  tenacity: number;
  piety: number;
}

// Parses food.json file in the asset
export function readFoodData(minIlvl: number, maxIlvl: number) {
  let foodDataFiltered = totalFood.filter(
    (food: Food) => food.itemLevel >= minIlvl && food.itemLevel <= maxIlvl
  );
  let foodDatabase: Map<number, Food> = new Map();
  foodDataFiltered.forEach((food: Food) => {
    foodDatabase.set(food.id, food);
  });

  return foodDatabase;
}

export function foodStatDescriptionString(food: Food, LANGUAGE_TEXTS: TextDictionary) {
  const stats = [];

  if (food.criticalStrike > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.CRIT_STAT_NAME, value: food.criticalStrike });
  }
  if (food.directHit > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.DH_STAT_NAME, value: food.directHit });
  }
  if (food.determination > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.DET_STAT_NAME, value: food.determination });
  }
  if (food.skillSpeed > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.SKS_STAT_NAME, value: food.skillSpeed });
  }
  if (food.spellSpeed > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.SPS_STAT_NAME, value: food.spellSpeed });
  }
  if (food.tenacity > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.TEN_STAT_NAME, value: food.tenacity });
  }
  if (food.piety > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.PIE_STAT_NAME, value: food.piety });
  }

  let descriptionString = "";
  stats.forEach((stat, _) => {
    descriptionString += `${stat.statName} +10%(Max ${stat.value}) `;
  });

  return descriptionString.trim();
}
