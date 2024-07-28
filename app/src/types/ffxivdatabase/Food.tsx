export interface Food {
  id: number;
  name: string;
  ilvl: number;
  piety: number;
  determination: number;
  criticalStrike: number;
  directHit: number;
  skillSpeed: number;
  spellSpeed: number;
  tenacity: number;
}

export async function readFoodData(minIlvl: number) {
  let jsonFile = process.env.PUBLIC_URL + "data/food_data.json";
  const response = await fetch(jsonFile);

  let foodData: Array<Food> = await response.json();
  let foodDataFiltered = foodData.filter((food: Food) => food.ilvl >= minIlvl);
  return foodDataFiltered.sort((a, b) => {
    let ilvlDiff = a.ilvl - b.ilvl;

    if (ilvlDiff !== 0) {
      return ilvlDiff;
    } else {
      return a.name.localeCompare(b.name);
    }
  });
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
    descriptionString += `${stat.statName} +${stat.value} `;
  });

  return descriptionString.trim();
}
