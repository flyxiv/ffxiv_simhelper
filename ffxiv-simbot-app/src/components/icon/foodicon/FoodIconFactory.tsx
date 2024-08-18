export function getFoodIconPath(name: string) {
  let base_directory = "../../.." + "/images/equipment/food";
  let foodIconName = name.toLowerCase().replace(/ /g, "_");

  return `${base_directory}/${foodIconName}.png`;
}
