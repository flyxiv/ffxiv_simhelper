import { IMAGES_DIRECTORY } from "../../../const/BaseDirectory";

export function getMateriaIconPath(materiaKey: string) {
  let base_directory = `${IMAGES_DIRECTORY}/equipment/materia`;
  let materiaTokens = materiaKey.split("+");
  let materiaStat = materiaTokens[0];
  let materiaValue = materiaTokens[1];
  let materiaIconPath = `${materiaStat}_${materiaValue}`;

  return `${base_directory}/${materiaIconPath}.png`;
}
