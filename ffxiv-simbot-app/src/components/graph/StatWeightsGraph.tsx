import { Box, styled, Typography } from "@mui/material";
import { BestPartnerBuffBarStyle, BuffBarBoxStyle } from "./Style";
import { AppConfigurations } from "../../Themes";
import { StatWeightsData } from "../../page/StatWeightsResult";

const MAX_RANKINGS = 4;
const RANKING_ITEM_HEIGHT = "4vh";

export function StatWeightsGraphContainer(statWeights: Array<StatWeightsData>) {
  let maxStatWeight = statWeights[0].rdpsIncreasePerPoint;

  return (
    <Box
      sx={{
        width: "95%",
        color: "white",
      }}
    >
      {StatWeightsGraph(statWeights, maxStatWeight)}
    </Box>
  );
}

function StatWeightsGraph(
  statWeights: Array<StatWeightsData>,
  maxStatWeight: number
) {
  let weightBars = statWeights.map((statWeight, index) => {
    let StatWeightBarBox = styled(Box)`
      ${BuffBarBoxStyle(
      (100 * statWeight.rdpsIncreasePerPoint) / maxStatWeight
    )}
    `;

    let Bar = styled(Box)`
      ${BestPartnerBuffBarStyle(
      Math.min(index, MAX_RANKINGS),
      RANKING_ITEM_HEIGHT
    )}
    `;

    // UI상 표시값 사이 일치를 위해 rounding한 값을 더한 거로 totalRdps를 한 번 정규화해준다.
    let valueString = statWeight.rdpsIncreasePerPoint.toFixed(2);

    return (
      <Box display="flex" marginY={1} height={RANKING_ITEM_HEIGHT}>
        <Box
          height={RANKING_ITEM_HEIGHT}
          display="flex"
          alignItems={"center"}
          width="4%"
        >
          <Typography variant="body1" fontSize={12} align="right">
            {statWeight.statName}
          </Typography>
        </Box>
        <Box width="80%" marginLeft="2%" marginRight="2%">
          <StatWeightBarBox>
            <Bar />
          </StatWeightBarBox>
        </Box>
        <Box display="flex" alignItems="center">
          <Typography variant="body1" fontSize={12}>
            {valueString}
          </Typography>
        </Box>
      </Box>
    );
  });

  return (
    <Box
      display="flex"
      flexDirection="column"
      justifyContent={"center"}
      width="90%"
      paddingX="4%"
      backgroundColor={AppConfigurations.backgroundThree}
      marginTop="5%"
      paddingY="5%"
    >
      {weightBars}
    </Box>
  );
}
