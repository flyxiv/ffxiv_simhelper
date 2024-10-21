import { Box } from "@mui/material";
import "./JobIconFactory.css";

const jobIcons = import.meta.glob(`/src/assets/images/jobicons/**/*.png`, { eager: true });

function jobAbbrevToJobIconPath(jobAbbrev: String) {
  switch (jobAbbrev) {
    case "PLD":
      return "tank/Paladin";
    case "WAR":
      return "tank/Warrior";
    case "DRK":
      return "tank/DarkKnight";
    case "GNB":
      return "tank/Gunbreaker";
    case "WHM":
      return "healer/WhiteMage";
    case "AST":
      return "healer/Astrologian";
    case "SCH":
      return "healer/Scholar";
    case "SGE":
      return "healer/Sage";
    case "MNK":
      return "dps/Monk";
    case "DRG":
      return "dps/Dragoon";
    case "NIN":
      return "dps/Ninja";
    case "SAM":
      return "dps/Samurai";
    case "RPR":
      return "dps/Reaper";
    case "VPR":
      return "dps/Viper";
    case "BRD":
      return "dps/Bard";
    case "MCH":
      return "dps/Machinist";
    case "DNC":
      return "dps/Dancer";
    case "SMN":
      return "dps/Summoner";
    case "RDM":
      return "dps/RedMage";
    case "BLM":
      return "dps/BlackMage";
    default:
      return "dps/Pictomancer";
  }
}

export function jobAbbrevToJobIcon(jobAbbrev: String) {
  const jobIconPath = jobAbbrevToJobIconPath(jobAbbrev);
  let jobIconFullPath = `/src/assets/images/jobicons/${jobIconPath}.png`;
  const jobIconModule = jobIcons[jobIconFullPath] as { default: string } | undefined;

  if (!jobIconModule) {
    return "";
  }

  return (
    jobIconModule.default
  );
}

export const JobIconFactory = (jobAbbrev: String, sizePixel: number) => {
  const size = sizePixel.toString() + "px";

  let jobIcon = jobAbbrevToJobIcon(jobAbbrev);

  return (
    <Box className="JobIconBox">
      <img
        src={jobIcon}
        alt="{jobIcon} icon"
        height={size}
        width={size}
        className="JobIcon"
      />
    </Box>
  );
};
