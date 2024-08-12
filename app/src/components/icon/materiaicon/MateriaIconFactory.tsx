export function getMateriaIconPath(materiaKey: string) {
  let base_directory = process.env.PUBLIC_URL + "/images/equipment/materia";
  let materiaIconPath = materiaKey.toLowerCase().replace("+", "_");

  return `${base_directory}/${materiaIconPath}.png`;
}
