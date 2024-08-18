import { Box } from "@mui/material";
import "./JobIconFactory.css";
import { IMAGES_DIRECTORY } from "../../../const/BaseDirectory";

function jobAbbrevToJobIconName(jobAbbrev: String) {
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
    case "PCT":
      return "dps/Pictomancer";
  }
}

export function jobAbbrevToJobIconPath(jobAbbrev: String) {
  return (
    `${IMAGES_DIRECTORY}/jobicons/${jobAbbrevToJobIconName(jobAbbrev)}.png`
  );
}

export const JobIconFactory = (jobAbbrev: String, sizePixel: number) => {
  const jobIcon = jobAbbrevToJobIconPath(jobAbbrev);
  const size = sizePixel.toString() + "px";

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
