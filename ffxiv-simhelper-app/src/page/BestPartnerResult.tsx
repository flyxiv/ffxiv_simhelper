import { Box, styled } from "@mui/material";
import {
  ResultBoardBoxStyle,
  ResultBoardTopBoxStyle,
} from "../components/container/Styles";
import { PlayerInfo } from "../components/container/PlayerInfo";
import { SimulationTitle } from "../components/basic/SimulationTitle";
import { BEST_PARTNER_RESPONSE_SAVE_NAME } from "../App";
import { AppConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppHeader } from "../components/image/AppHeader";
import {
  BestPartnerResponseTable,
  BestPartnerSingleBurst,
} from "../types/BestPartnerResponse";
import { Footer } from "../components/basic/Footer";
import { SimulationDataByRole } from "../types/ffxivdatabase/PartyCompositionMaker";
import { ContributionByRoleTable } from "../components/graph/ContributionByRoleTable";
import { BEST_PARTNER_ITERATION_COUNT } from "../components/basic/BestPartnerRequestButton";
import { AppLanguageTexts } from "../const/languageTexts";

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

const ResultTopBoardBox = styled(Box)`
  ${ResultBoardTopBoxStyle}
`;

export const TABLE_WIDTH = "80%";

export function BestPartnerResult() {
  let response = localStorage.getItem(BEST_PARTNER_RESPONSE_SAVE_NAME);

  if (response == null) {
    return (
      <div>
        <p>Simulation Result is not available: simulation result is null.</p>
      </div>
    );
  }

  let responseJson = JSON.parse(response) as BestPartnerResponseTable;
  let mainPlayerJob = responseJson.mainPlayerJobAbbrev;
  let contributionTable = responseJson.partnerSimulationData;

  let burst_count = contributionTable[0].contributedDps.length;

  let simulationDataByRoles: Array<SimulationDataByRole> = [];

  for (let i = 0; i < burst_count; i++) {
    let singleBurstResponses = contributionTable.map((response) => {
      return {
        partnerJobAbbrev: response.partnerJobAbbrev,
        contributedDps: response.contributedDps[i],
        minute: (i - 1) * 2,
      };
    });
    simulationDataByRoles.push(
      convertToContributionTable(singleBurstResponses)
    );
  }
  let LANGUAGE_TEXTS = AppLanguageTexts();

  return (
    <Box
      display="flex"
      flexDirection={"column"}
      sx={{ backgroundColor: AppConfigurations.backgroundOne }}
      width="100vw"
      alignItems={"center"}
      paddingBottom={20}
    >
      <Box display="flex">
        {BasicLeftMenu(LANGUAGE_TEXTS.BEST_PARTNER_PAGE_NAME)}
        <Box>
          {AppHeader()}
          <ResultTopBoardBox marginBottom="40px">
            {SimulationTitle(LANGUAGE_TEXTS.SIMULATION_RESULT_TEXT)}
            {PlayerInfo(
              responseJson.mainPlayerPower,
              mainPlayerJob,
              responseJson.combatTimeMillisecond,
              null,
              BEST_PARTNER_ITERATION_COUNT,
              1
            )}
          </ResultTopBoardBox>
          <ResultBoardBox>
            {SimulationTitle(LANGUAGE_TEXTS.OVERALL_TEXT)}
            {ContributionByRoleTable(simulationDataByRoles[0])}

            {simulationDataByRoles.slice(1).map((table, index) => {
              let burstMinute = index * 2;

              return (
                <>
                  {SimulationTitle(`${burstMinute}${LANGUAGE_TEXTS.BURST_TEXT}`)}
                  {ContributionByRoleTable(table)};
                </>
              );
            })}
          </ResultBoardBox>
          {Footer()}
        </Box>
      </Box>
    </Box>
  );
}

function convertToContributionTable(
  partnerSimulationData: Array<BestPartnerSingleBurst>
) {
  let table: SimulationDataByRole = {
    tanks: [],
    healers: [],
    melee: [],
    ranged: [],
    casters: [],
  };
  let LANGUAGE_TEXTS = AppLanguageTexts();

  for (let data of partnerSimulationData) {
    switch (data.partnerJobAbbrev) {
      case LANGUAGE_TEXTS.PLD_EN_NAME:
      case LANGUAGE_TEXTS.WAR_EN_NAME:
      case LANGUAGE_TEXTS.DRK_EN_NAME:
      case LANGUAGE_TEXTS.GNB_EN_NAME:
        table.tanks.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps === undefined ? 0 : data.contributedDps,
        });
        break;

      case LANGUAGE_TEXTS.WHM_EN_NAME:
      case LANGUAGE_TEXTS.AST_EN_NAME:
      case LANGUAGE_TEXTS.SCH_EN_NAME:
      case LANGUAGE_TEXTS.SGE_EN_NAME:
        table.healers.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps === undefined ? 0 : data.contributedDps,
        });
        break;

      case LANGUAGE_TEXTS.DRG_EN_NAME:
      case LANGUAGE_TEXTS.MNK_EN_NAME:
      case LANGUAGE_TEXTS.NIN_EN_NAME:
      case LANGUAGE_TEXTS.SAM_EN_NAME:
      case LANGUAGE_TEXTS.RPR_EN_NAME:
      case LANGUAGE_TEXTS.VPR_EN_NAME:
        table.melee.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps === undefined ? 0 : data.contributedDps,
        });
        break;

      case LANGUAGE_TEXTS.BRD_EN_NAME:
      case LANGUAGE_TEXTS.MCH_EN_NAME:
      case LANGUAGE_TEXTS.DNC_EN_NAME:
        table.ranged.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps === undefined ? 0 : data.contributedDps,
        });
        break;

      default:
        table.casters.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps === undefined ? 0 : data.contributedDps,
        });
        break;
    }
  }

  table.tanks.sort((a, b) => {
    let buffContributionA = a.buffContribution == null ? 0 : a.buffContribution;
    let buffContributionB = b.buffContribution == null ? 0 : b.buffContribution;

    return buffContributionB - buffContributionA;
  });

  table.healers.sort((a, b) => {
    let buffContributionA = a.buffContribution == null ? 0 : a.buffContribution;
    let buffContributionB = b.buffContribution == null ? 0 : b.buffContribution;

    return buffContributionB - buffContributionA;
  });

  table.melee.sort((a, b) => {
    let buffContributionA = a.buffContribution == null ? 0 : a.buffContribution;
    let buffContributionB = b.buffContribution == null ? 0 : b.buffContribution;

    return buffContributionB - buffContributionA;
  });

  table.ranged.sort((a, b) => {
    let buffContributionA = a.buffContribution == null ? 0 : a.buffContribution;
    let buffContributionB = b.buffContribution == null ? 0 : b.buffContribution;

    return buffContributionB - buffContributionA;
  });

  table.casters.sort((a, b) => {
    let buffContributionA = a.buffContribution == null ? 0 : a.buffContribution;
    let buffContributionB = b.buffContribution == null ? 0 : b.buffContribution;

    return buffContributionB - buffContributionA;
  });

  return table;
}
