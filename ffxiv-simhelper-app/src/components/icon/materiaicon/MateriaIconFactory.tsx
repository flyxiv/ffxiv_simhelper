const materiaIcons = import.meta.glob(`/src/assets/images/equipment/materia/**/*.png`, { eager: true });

export function getMateriaIconPath(materiaKey: string) {
  let base_directory = `/src/assets/images/equipment/materia`;
  let materiaTokens = materiaKey.split("+");
  let materiaStat = materiaTokens[0].toLowerCase();
  let materiaValue = materiaTokens[1];
  let materiaIconPath = `${materiaStat}_${materiaValue}`;

  let materiaIconFullPath = `${base_directory}/${materiaIconPath}.png`;
  const materiaIconModule = materiaIcons[materiaIconFullPath] as { default: string } | undefined;

  if (!materiaIconModule) {
    return "";
  }

  return materiaIconModule.default;
}
