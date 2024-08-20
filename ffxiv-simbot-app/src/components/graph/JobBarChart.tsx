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
import { StatusIdToIconPathFactory } from "../icon/abilityicon/StatusIconFactory";
import { JobIconFactory } from "../icon/jobicon/JobIconFactory";
import { TeammateChartData } from "./GraphData";

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

export function JobBarChartTeammate(
  data: TeammateChartData,
  maxContribution: number
) {
  let totalRdps = 0;
  let buffBars = data.rdpsEntry.map((entry, index) => {
    let BuffBarBox = styled(Box)`
      ${BuffBarBoxStyle((100 * entry.rdps) / maxContribution)}
    `;

    let Bar = styled(Box)`
      ${BuffBarStyle(index)}
    `;

    // UI상 표시값 사이 일치를 위해 rounding한 값을 더한 거로 totalRdps를 한 번 정규화해준다.
    let roundedRdps = Math.round(entry.rdps);
    totalRdps += roundedRdps;

    return (
      <BuffBarBox>
        {roundedRdps}
        <Bar>
          <img
            src={StatusIdToIconPathFactory(entry.statusId)}
            alt={"rdps"}
            width={25}
            height={25}
          />
        </Bar>
      </BuffBarBox>
    );
  });

  return (
    <PartyMemberBuffBox>
      <PartyMemberIconBox>
        {JobIconFactory(data.jobName, 25)}
      </PartyMemberIconBox>
      <TotalBuffBox>{buffBars}</TotalBuffBox>
      <TotalRdpsBox>
        <Typography variant="h6">{totalRdps}</Typography>
      </TotalRdpsBox>
    </PartyMemberBuffBox>
  );
}

export function GraphTitleRow() {
  return (
    <PartyMemberBuffBox>
      <PartyMemberIconBox>
        <Typography variant="h6">Member</Typography>
      </PartyMemberIconBox>
      <BuffTitleBar>
        <Typography variant="h6">RDPS Contribution</Typography>
      </BuffTitleBar>
      <TotalRdpsBox>
        <Typography variant="h6">Total</Typography>
      </TotalRdpsBox>
    </PartyMemberBuffBox>
  );
}
