export function getFoodIconPath(name: string) {
  let base_directory = process.env.PUBLIC_URL + "/images/equipment/food";
  let foodIconName = name.toLowerCase().replace(/ /g, "_");

  return `${base_directory}/${foodIconName}.png`;
}
