import totalFoodJson from "../../assets/data/food_data.json";

const totalFood: Array<Food> = [];

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

export const FOOD_DATABASE = readFoodData(710);
export const ALL_FOODS = Array.from(FOOD_DATABASE.values());

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

export function readFoodData(minIlvl: number) {
  let foodDataFiltered = totalFood.filter(
    (food: Food) => food.itemLevel >= minIlvl
  );
  let foodDatabase: Map<number, Food> = new Map();
  foodDataFiltered.forEach((food: Food) => {
    foodDatabase.set(food.id, food);
  });

  return foodDatabase;
}

export function foodStatDescriptionString(food: Food) {
  const stats = [];

  if (food.criticalStrike > 0) {
    stats.push({ statName: "CRT", value: food.criticalStrike });
  }
  if (food.directHit > 0) {
    stats.push({ statName: "DH", value: food.directHit });
  }
  if (food.determination > 0) {
    stats.push({ statName: "DET", value: food.determination });
  }
  if (food.skillSpeed > 0) {
    stats.push({ statName: "SKS", value: food.skillSpeed });
  }
  if (food.spellSpeed > 0) {
    stats.push({ statName: "SPS", value: food.spellSpeed });
  }
  if (food.tenacity > 0) {
    stats.push({ statName: "TEN", value: food.tenacity });
  }
  if (food.piety > 0) {
    stats.push({ statName: "PIE", value: food.piety });
  }

  let descriptionString = "";
  stats.forEach((stat, _) => {
    descriptionString += `${stat.statName} +10%(Max ${stat.value}) `;
  });

  return descriptionString.trim();
}
