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
import {
  AST_EN_NAME,
  BRD_EN_NAME,
  BURST_TEXT,
  DNC_EN_NAME,
  DRG_EN_NAME,
  DRK_EN_NAME,
  GNB_EN_NAME,
  MCH_EN_NAME,
  MNK_EN_NAME,
  NIN_EN_NAME,
  OVERALL_TEXT,
  PLD_EN_NAME,
  RPR_EN_NAME,
  SAM_EN_NAME,
  SCH_EN_NAME,
  SGE_EN_NAME,
  SIMULATION_RESULT_TEXT,
  VPR_EN_NAME,
  WAR_EN_NAME,
  WHM_EN_NAME,
} from "../const/languageTexts";

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
        {BasicLeftMenu()}
        <Box>
          {AppHeader()}
          <ResultTopBoardBox marginBottom="40px">
            {SimulationTitle(SIMULATION_RESULT_TEXT)}
            {PlayerInfo(
              responseJson.mainPlayerPower,
              mainPlayerJob,
              responseJson.combatTimeMillisecond
            )}
          </ResultTopBoardBox>
          <ResultBoardBox>
            {SimulationTitle(OVERALL_TEXT)}
            {ContributionByRoleTable(simulationDataByRoles[0])}

            {simulationDataByRoles.slice(1).map((table, index) => {
              let burstMinute = index * 2;

              return (
                <>
                  {SimulationTitle(BURST_TEXT(burstMinute))}
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

  for (let data of partnerSimulationData) {
    switch (data.partnerJobAbbrev) {
      case PLD_EN_NAME:
      case WAR_EN_NAME:
      case DRK_EN_NAME:
      case GNB_EN_NAME:
        table.tanks.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps === undefined ? 0 : data.contributedDps,
        });
        break;

      case WHM_EN_NAME:
      case AST_EN_NAME:
      case SCH_EN_NAME:
      case SGE_EN_NAME:
        table.healers.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps === undefined ? 0 : data.contributedDps,
        });
        break;

      case DRG_EN_NAME:
      case MNK_EN_NAME:
      case NIN_EN_NAME:
      case SAM_EN_NAME:
      case RPR_EN_NAME:
      case VPR_EN_NAME:
        table.melee.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps === undefined ? 0 : data.contributedDps,
        });
        break;

      case BRD_EN_NAME:
      case MCH_EN_NAME:
      case DNC_EN_NAME:
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
