import { styled, Button } from "@mui/material";
import { ColorConfigurations } from "../Themes";

/// actions/{jobName}/{skillName}.png -> skillName
export function iconPathToName(iconPath: String) {
  let iconPathArray = iconPath.split("/");
  const regExp = new RegExp("_", "g");
  return iconPathArray[iconPathArray.length - 1]
    .split(".")[0]
    .replace(regExp, " ");
}

export const ButtonStyle = styled(Button)`
  font-size: 0.8rem;
  margin: 1rem;
  height: 8vh;
  background-color: ${ColorConfigurations.backgroundButton};
  color: black;
`;
