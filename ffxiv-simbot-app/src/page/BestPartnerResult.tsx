import { Box, styled } from "@mui/material";
import { ResultBoardBoxStyle } from "../components/container/Styles";
import { PlayerInfo } from "../components/container/PlayerInfo";
import { SimulationTitle } from "../components/basic/SimulationTitle";
import { BEST_PARTNER_RESPONSE_SAVE_NAME } from "../App";
import { ColorConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppHeader } from "../components/image/AppHeader";
import { BestPartnerResponseTable } from "../types/BestPartnerResponse";
import { Footer } from "../components/basic/Footer";

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
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
          {Footer()}
        </Box>
      </Box>
    </Box>
  );
}
