export function getMateriaIconPath(materiaKey: string) {
  let base_directory = "../../../assets/images/equipment/materia";
  let materiaIconPath = materiaKey.toLowerCase().replace("+", "_");

  return `${base_directory}/${materiaIconPath}.png`;
}
