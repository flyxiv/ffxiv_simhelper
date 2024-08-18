import { IMAGES_DIRECTORY } from "../../../const/BaseDirectory";

export function getFoodIconPath(name: string) {
  let base_directory = `${IMAGES_DIRECTORY}/equipment/food`;
  let foodIconName = name.toLowerCase().replace(/ /g, "_");

  return `${base_directory}/${foodIconName}.png`;
}
