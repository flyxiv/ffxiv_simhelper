import { IMAGES_DIRECTORY } from "../../../const/BaseDirectory";

export function getMateriaIconPath(materiaKey: string) {
  let base_directory = `${IMAGES_DIRECTORY}/equipment/materia`;
  let materiaIconPath = materiaKey.toLowerCase().replace("+", "_");

  return `${base_directory}/${materiaIconPath}.png`;
}
