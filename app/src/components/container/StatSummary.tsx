import { styled, Box, Typography } from "@mui/material";
import { SingleStatBoxStyle, StatSummaryBoxStyle } from "./Styles";
import { CharacterStates } from "src/types/CharacterStates";
import { getStatNames } from "src/types/ffxivdatabase/Stats";
import { ItemSet } from "src/types/ffxivdatabase/ItemSet";

let StatSummaryBox = styled(Box)`
  ${StatSummaryBoxStyle}
`;

let SingleStatBox = styled(Box)`
  ${SingleStatBoxStyle}
`;

export function StatSummary(jobAbbrev: string, itemSet: ItemSet) {
  let jobStatNames = getStatNames(jobAbbrev);
  return (
    <StatSummaryBox>
      {jobStatNames.map((statName) => {
        return (
          <SingleStatBox>
            <Typography variant="body1">{statName}</Typography>
          </SingleStatBox>
        );
      })}
    </StatSummaryBox>
  );
}
