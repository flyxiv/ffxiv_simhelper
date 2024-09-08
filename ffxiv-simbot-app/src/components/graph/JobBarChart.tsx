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
import { MEMBER_TEXT, TOTAL_TEXT } from "../../const/languageTexts";

export const JOB_BAR_ITEM_HEIGHT = "65px";
const IMAGE_SIZE = 30;
const IMAGE_SIZE_PX = `${IMAGE_SIZE}px`;

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
        <Box height="25px">
          {roundedRdps}
        </Box>
        <Bar height="40px" display="flex" alignItems="center" justifyContent="center">
          <img
            src={StatusIdToIconPathFactory(entry.statusId)}
            alt={"rdps"}
            width={IMAGE_SIZE_PX}
            height={IMAGE_SIZE_PX}
          />
        </Bar>
      </BuffBarBox>
    );
  });

  return (
    <PartyMemberBuffBox height={JOB_BAR_ITEM_HEIGHT}>
      <PartyMemberIconBox>
        <Box height="25px" />
        <Box height="40px" display="flex" alignItems="center">
          {JobIconFactory(data.jobName, IMAGE_SIZE)}
        </Box>
      </PartyMemberIconBox>
      <TotalBuffBox>{buffBars}</TotalBuffBox>
      <TotalRdpsBox>
        <Box height="25px" />
        <Typography variant="h6">{totalRdps}</Typography>
      </TotalRdpsBox>
    </PartyMemberBuffBox>
  );
}

export function GraphTitleRow() {
  let fontSize = 14;
  return (
    <PartyMemberBuffBox>
      <PartyMemberIconBox>
        <Typography variant="h6" fontSize={fontSize}>
          {MEMBER_TEXT}
        </Typography>
      </PartyMemberIconBox>
      <BuffTitleBar />
      <TotalRdpsBox>
        <Typography variant="h6" fontSize={fontSize}>
          {TOTAL_TEXT}
        </Typography>
      </TotalRdpsBox>
    </PartyMemberBuffBox>
  );
}
