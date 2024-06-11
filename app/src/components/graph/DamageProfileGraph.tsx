import { QuickSimResponse } from "src/types/QuickSimResponse";
import {
  DamageChartData,
  DamageProfileBarChart,
} from "./DamageProfileBarChart";
import { SkillIdToIconPathFactory } from "../abilityicon/SkillIconFactory";
import { Box } from "@mui/material";

export const DamageProfileGraph = (response: QuickSimResponse) => {
  const mainPlayerId = response.mainPlayerId;
  const simulationDatas = response.simulationData;

  let rdpsProfileMap = new Map<String, number>();
  let pdpsProfileMap = new Map<String, number>();
  let totalSkills = new Set<String>();
  let damageProfileData: Array<DamageChartData> = [];
  let mainPlayerSimulationData = null;

  let i = 0;
  for (i = 0; i < simulationDatas.length; i++) {
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

  for (i = 0; i < mainPlayerSimulationData.damageProfileTable.length; i++) {
    let profile = mainPlayerSimulationData.damageProfileTable[i];
    let iconPath = SkillIdToIconPathFactory(profile.id);

    let currentSkillRdps = rdpsProfileMap.get(iconPath);
    let currentSkillPdps = pdpsProfileMap.get(iconPath);

    totalSkills.add(iconPath);

    if (currentSkillRdps === undefined) {
      rdpsProfileMap.set(iconPath, profile.rdpsContribution);
    } else {
      rdpsProfileMap.set(iconPath, currentSkillRdps + profile.rdpsContribution);
    }

    if (currentSkillPdps === undefined) {
      pdpsProfileMap.set(iconPath, profile.pdpsContribution);
    } else {
      pdpsProfileMap.set(iconPath, currentSkillPdps + profile.pdpsContribution);
    }
  }

  let totalSkillsArray = Array.from(totalSkills);

  for (i = 0; i < totalSkillsArray.length; i++) {
    let iconPath = totalSkillsArray[i];
    let rdps = rdpsProfileMap.get(iconPath);
    let pdps = pdpsProfileMap.get(iconPath);

    if (rdps === undefined) {
      rdps = 0;
    }
    if (pdps === undefined) {
      pdps = 0;
    }

    damageProfileData.push({
      icon: iconPath,
      rdps: rdps,
      pdps: pdps,
    });
  }

  damageProfileData.sort((a, b) => b.pdps - a.pdps);

  return (
    <Box width={500} height={80 + damageProfileData.length * 40}>
      <DamageProfileBarChart data={damageProfileData} />
    </Box>
  );
};
