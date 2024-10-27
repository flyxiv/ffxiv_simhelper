import { Typography, Box, styled } from "@mui/material";
import {
  TotalBarStyle,
  BuffBarBoxStyle,
  BuffBarStyle,
  PartyMemberBuffBoxStyle,
  PartyMemberIconBoxStyle,
  TotalRdpsBoxStyle,
  BuffTitleBarStyle,
} from "./Style";
import { PartyCompositionChartData } from "./GraphData";
import { AppConfigurations } from "../../Themes";

export const JOB_BAR_ITEM_HEIGHT = "65px";

const PartyMemberBuffBox = styled(Box)`
  ${PartyMemberBuffBoxStyle}
`;

const PartyMemberIconBox = styled(Box)`
  ${PartyMemberIconBoxStyle}
`;

const TotalBuffBox = styled(Box)`
  ${TotalBarStyle}
`;
const TotalRdpsBox = styled(Box)`
  ${TotalRdpsBoxStyle}
`;
const BuffTitleBar = styled(Box)`
  ${BuffTitleBarStyle}
`;

function JobBarChartPartyComposition(
  data: PartyCompositionChartData,
  maxContribution: number
) {
  let totalRdps = 0;
  let BuffBarBox = styled(Box)`
      ${BuffBarBoxStyle((100 * data.totalRdps) / maxContribution)}
    `;

  let Bar = styled(Box)`
      ${BuffBarStyle(0)}
    `;

  // UI상 표시값 사이 일치를 위해 rounding한 값을 더한 거로 totalRdps를 한 번 정규화해준다.
  let roundedRdps = Math.round(data.totalRdps);
  totalRdps += roundedRdps;

  return (
    <PartyMemberBuffBox height={JOB_BAR_ITEM_HEIGHT}>
      <PartyMemberIconBox>
        <Box height="25px" />
        <Box height="40px" display="flex" alignItems="center">
          {data.key}
        </Box>
      </PartyMemberIconBox>
      <TotalBuffBox>
        <BuffBarBox>
          <Box height="25px">
            {roundedRdps}
          </Box>
          <Bar />
        </BuffBarBox>
      </TotalBuffBox>
      <TotalRdpsBox>
        <Box height="25px" />
        <Typography variant="h6">{totalRdps}</Typography>
      </TotalRdpsBox>
    </PartyMemberBuffBox>
  );
}

export function GraphTitleRow(memberText: string, totalText: string) {
  return (
    <PartyMemberBuffBox>
      <PartyMemberIconBox>
        <Typography variant="h6" fontSize={AppConfigurations.body1FontSize}>
          {memberText}
        </Typography>
      </PartyMemberIconBox>
      <BuffTitleBar />
      <TotalRdpsBox>
        <Typography variant="h6" fontSize={AppConfigurations.body2FontSize}>
          {totalText}
        </Typography>
      </TotalRdpsBox>
    </PartyMemberBuffBox>
  );
}


export function PartyCompositionGraph(
  data: PartyCompositionChartData[],
) {
  const maxContribution = data.map((_, value) => value).reduce((max, current) => Math.max(max, current), 0);

  let partyCompositionBars = data.map((entry) => {
    return JobBarChartPartyComposition(entry, maxContribution);
  });

  return (
    <Box>
      {partyCompositionBars}
    </Box>
  );
}