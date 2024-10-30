import { Typography, Box, styled } from "@mui/material";
import {
  TotalBarStyle,
  BuffBarBoxStyle,
  BuffBarStyle,
  PartyMemberBuffBoxStyle,
  PartyMemberIconBoxStyle,
  TotalRdpsBoxStyle,
  BuffTitleBarStyle,
  PartyCompositionIconBoxStyle,
} from "./Style";
import { PartyCompositionChartData } from "./GraphData";
import { AppConfigurations } from "../../Themes";
import { jobAbbrevToJobIcon } from "../icon/jobicon/JobIconFactory";

export const JOB_BAR_ITEM_HEIGHT = "65px";

const PartyMemberBuffBox = styled(Box)`
  ${PartyMemberBuffBoxStyle}
`;

const PartyCompositionIconBox = styled(Box)`
  ${PartyCompositionIconBoxStyle}
`;

const TotalBuffBox = styled(Box)`
  ${TotalBarStyle}
`;
const TotalRdpsBox = styled(Box)`
  ${TotalRdpsBoxStyle}
`;

function JobBarChartPartyComposition(
  data: PartyCompositionChartData,
  minContribution: number,
  maxContribution: number,
  index: number
) {
  let totalRdps = Math.round(data.totalRdps);
  let maxDiff = maxContribution - minContribution;
  let diff = data.totalRdps - minContribution;

  console.log(minContribution, maxContribution);
  let BuffBarBox = styled(Box)`
      ${BuffBarBoxStyle((100 * diff) / maxDiff)}
    `;

  let Bar = styled(Box)`
      ${BuffBarStyle(Math.min(index, 4))}
    `;

  return (
    <PartyMemberBuffBox height={JOB_BAR_ITEM_HEIGHT} display="flex" flexDirection={"column"} justifyContent={"center"} alignItems={"center"}>
      <PartyCompositionIconBox>
        <Box height="40px" display="flex" alignItems="center">
          {data.key.map((key) => {
            return (
              <Box component="img"
                src={jobAbbrevToJobIcon(key)}
                alt={"rdps"}
                width={35}
                height={35}
                margin={0.5}
              />
            );
          })}
        </Box>
      </PartyCompositionIconBox>
      <Box width="60%">
        <TotalBuffBox>
          <BuffBarBox height="40px">
            <Bar sx={{ height: "100%" }} />
          </BuffBarBox>
        </TotalBuffBox>
      </Box>
      <TotalRdpsBox sx={{ height: "60px" }}>
        <Typography variant="h6" align="center">{totalRdps}</Typography>
      </TotalRdpsBox>
    </PartyMemberBuffBox>
  );
}

export function GraphTitleRow(memberText: string, totalText: string) {
  return (
    <PartyMemberBuffBox sx={{ width: "100%" }}>
      <PartyCompositionIconBox sx={{ width: "440px" }}>
        <Typography variant="h6" fontSize={AppConfigurations.body1FontSize} align="center">
          {memberText}
        </Typography>
      </PartyCompositionIconBox>
      <Box width="60%" />

      <TotalRdpsBox sx={{ width: "15%" }} paddingRight={2}>
        <Typography variant="h6" fontSize={AppConfigurations.body1FontSize} align="center">
          {totalText}
        </Typography>
      </TotalRdpsBox>
    </PartyMemberBuffBox>
  );
}


export function PartyCompositionGraph(
  data: PartyCompositionChartData[],
  minContribution: number,
  maxContribution: number
) {
  let partyCompositionBars = data.map((entry, index) => {
    return JobBarChartPartyComposition(entry, minContribution, maxContribution, index);
  });

  return (
    <Box width="80%" display="flex" flexDirection={"column"} justifyContent={"center"} alignItems={"center"}>
      {GraphTitleRow("Party Composition", "Median RDPS")}
      {partyCompositionBars}
    </Box>
  );
}