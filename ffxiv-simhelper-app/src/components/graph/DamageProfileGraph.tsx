import { DpsAnalysisResponse } from "../../types/DpsAnalysisResponse";
import { DamageChartTitle, SkillDamageProfile } from "./DamageProfileBarChart";
import { skillIdToIcon } from "../icon/abilityicon/SkillIconFactory";
import { iconPathToName } from "../Util";
import { DamageChartData } from "./GraphData";
import { Box } from "@mui/material";
import { TextDictionary } from "../../const/languageTexts";

export const DamageProfileGraph = (response: DpsAnalysisResponse, LANGUAGE_TEXTS: TextDictionary) => {
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

    let iconPath = skillIdToIcon(profile.id);
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
    <Box width={{ xs: "95%", sm: "90%", md: "85%", lg: "80%", xl: "75%" }}>
      {DamageChartTitle(LANGUAGE_TEXTS.SKILL_TITLE_TEXT, LANGUAGE_TEXTS.DAMAGE_PERCENTAGE_TEXT, LANGUAGE_TEXTS.TOTAL_DAMAGE_TEXT, LANGUAGE_TEXTS.CAST_TEXT)}
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
