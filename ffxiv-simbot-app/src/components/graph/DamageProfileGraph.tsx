import { QuickSimResponse } from "../../types/QuickSimResponse";
import { DamageChartTitle, SkillDamageProfile } from "./DamageProfileBarChart";
import { SkillIdToIconPathFactory } from "../icon/abilityicon/SkillIconFactory";
import { iconPathToName } from "../Util";
import { DamageChartData } from "./GraphData";
import { Box } from "@mui/material";

export const DamageProfileGraph = (response: QuickSimResponse) => {
  const mainPlayerId = response.mainPlayerId;
  const simulationDatas = response.simulationData;

  let damageProfileData: Array<DamageChartData> = [];
  let mainPlayerSimulationData = null;
  let totalDps = 0;

  for (let i = 0; i < simulationDatas.length; i++) {
    if (simulationDatas[i].playerId === mainPlayerId) {
      mainPlayerSimulationData = simulationDatas[i];
      break;
    }
  }

  if (mainPlayerSimulationData === null) {
    return (
      <div>
        <p>
          Simulation Result is not available: main player simulation data is
          null.
        </p>
      </div>
    );
  }

  for (let i = 0; i < mainPlayerSimulationData.damageProfileTable.length; i++) {
    let profile = mainPlayerSimulationData.damageProfileTable[i];
    if (profile.entity === "Status") {
      continue;
    }

    let iconPath = SkillIdToIconPathFactory(profile.id);
    let skillName = iconPathToName(iconPath);
    totalDps += profile.pdpsContribution;

    damageProfileData.push({
      name: skillName,
      pdps: profile.pdpsContribution,
      rdps: profile.rdpsContribution,
      icon: iconPath,
      castCount: profile.castCount,
    });
  }

  damageProfileData.sort((a, b) => b.pdps - a.pdps);
  let highestDamageOfSingleSkill = damageProfileData[0].pdps;

  return (
    <Box>
      {DamageChartTitle}
      {damageProfileData.map((data) => {
        return SkillDamageProfile(
          data,
          totalDps,
          highestDamageOfSingleSkill,
          response.combatTimeMillisecond
        );
      })}
    </Box>
  );
};
