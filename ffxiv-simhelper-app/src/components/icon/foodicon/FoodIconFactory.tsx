const foodIcons = import.meta.glob(`/src/assets/images/equipment/food/**/*.png`, { eager: true });

export function getFoodIconPath(name: string) {
  let foodIconName = name.toLowerCase().replace(/ /g, "_");
  let foodIconFullPath = `/src/assets/images/equipment/food/${foodIconName}.png`;

  const jobIconModule = foodIcons[foodIconFullPath] as { default: string } | undefined;

  if (!jobIconModule) {
    return "";
  }

  return (
    jobIconModule.default
  );
}
