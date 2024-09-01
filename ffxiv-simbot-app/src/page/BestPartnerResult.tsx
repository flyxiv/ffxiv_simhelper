import { Box, styled } from "@mui/material";
import { ResultBoardBoxStyle } from "../components/container/Styles";
import { PlayerInfo } from "../components/container/PlayerInfo";
import { SimulationTitle } from "../components/basic/SimulationTitle";
import { BEST_PARTNER_RESPONSE_SAVE_NAME } from "../App";
import { ColorConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppHeader } from "../components/image/AppHeader";
import {
  BestPartnerResponse,
  BestPartnerResponseTable,
} from "../types/BestPartnerResponse";
import { Footer } from "../components/basic/Footer";
import {
  PartyCompositionMaker,
  SimulationDataByRole,
} from "../types/ffxivdatabase/PartyCompositionMaker";
import { ContributionByRoleTable } from "../components/graph/ContributionByRoleTable";

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

const BEST_PARTNERS_BY_ROLE_TEXT = "Best Partner By Role";

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
  let simulationDataByRole = convertToContributionTable(contributionTable);
  let partyCompositionMaker = new PartyCompositionMaker(
    mainPlayerJob,
    simulationDataByRole
  );

  return (
    <Box
      display="flex"
      flexDirection={"column"}
      sx={{ backgroundColor: ColorConfigurations.backgroundOne }}
      width="100vw"
      alignItems={"center"}
      paddingBottom={20}
    >
      <Box display="flex">
        {BasicLeftMenu()}
        <Box>
          {AppHeader()}
          <ResultBoardBox>
            {SimulationTitle("Simulation Result")}
            {PlayerInfo(mainPlayerJob, responseJson.combatTimeMillisecond)}
          </ResultBoardBox>
          <ResultBoardBox>
            {SimulationTitle(BEST_PARTNERS_BY_ROLE_TEXT)}
            {ContributionByRoleTable(simulationDataByRole, mainPlayerJob)}
          </ResultBoardBox>
          {Footer()}
        </Box>
      </Box>
    </Box>
  );
}

function convertToContributionTable(
  partnerSimulationData: Array<BestPartnerResponse>
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
      case "PLD":
      case "WAR":
      case "DRK":
      case "GNB":
        table.tanks.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps,
        });
        break;

      case "WHM":
      case "AST":
      case "SCH":
      case "SGE":
        table.healers.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps,
        });
        break;

      case "DRG":
      case "MNK":
      case "NIN":
      case "SAM":
      case "RPR":
      case "VPR":
        table.melee.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps,
        });
        break;

      case "BRD":
      case "MCH":
      case "DNC":
        table.ranged.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps,
        });
        break;

      default:
        table.casters.push({
          jobAbbrev: data.partnerJobAbbrev,
          buffContribution: data.contributedDps,
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
