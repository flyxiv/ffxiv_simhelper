import { Box } from "@mui/material";
import "./JobIconFactory.css";

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
      return "healer/Whitemage";
    case "SCH":
      return "healer/Scholar";
    case "AST":
      return "healer/Astrologian";
    case "MNK":
      return "dps/Monk";
    case "DRG":
      return "dps/Dragoon";
    case "NIN":
      return "dps/Ninja";
    case "SAM":
      return "dps/Samurai";
    case "BRD":
      return "dps/Bard";
    case "MCH":
      return "dps/Machinist";
    case "DNC":
      return "dps/Dancer";
    case "SGE":
      return "healer/Sage";
  }
}

export function jobAbbrevToJobIconPath(jobAbbrev: String) {
  return (
    process.env.PUBLIC_URL +
    `/images/jobicons/${jobAbbrevToJobIconName(jobAbbrev)}.png`
  );
}

export const JobIconFactory = (jobAbbrev: String) => {
  const jobIcon = jobAbbrevToJobIconPath(jobAbbrev);

  return (
    <Box className="JobIconBox">
      <img
        src={jobIcon}
        alt="{jobIcon} icon"
        height="50px"
        width="50px"
        className="JobIcon"
      />
    </Box>
  );
};
