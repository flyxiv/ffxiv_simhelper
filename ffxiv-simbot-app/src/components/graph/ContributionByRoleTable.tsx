import { Box, Grid, styled, Typography } from "@mui/material";
import {
  BestPartnerSimulationData,
  SimulationDataByRole,
} from "../../types/ffxivdatabase/PartyCompositionMaker";
import { BestPartnerBuffBarStyle, BuffBarBoxStyle } from "./Style";
import { jobAbbrevToJobIconPath } from "../icon/jobicon/JobIconFactory";
import { AppConfigurations } from "../../Themes";

const MAX_RANKINGS = 4;
const RANKING_ITEM_HEIGHT = "4vh";

export function ContributionByRoleTable(
  simulationDataByRole: SimulationDataByRole,
  mainPlayerJobAbbrev: string
) {
  let tankPartnerData = simulationDataByRole.tanks;
  let healerPartnerData = simulationDataByRole.healers;
  let dpsPartnerData = simulationDataByRole.melee;

  dpsPartnerData = dpsPartnerData.concat(simulationDataByRole.ranged);
  dpsPartnerData = dpsPartnerData.concat(simulationDataByRole.casters);
  dpsPartnerData.sort((a, b) => {
    let aContribution = a.buffContribution === null ? 0 : a.buffContribution;
    let bContribution = b.buffContribution === null ? 0 : b.buffContribution;
    return bContribution - aContribution;
  });

  let maxTankContribution = Math.max.apply(
    null,
    tankPartnerData.map((data) =>
      data.buffContribution === null ? 0 : data.buffContribution
    )
  );
  let maxHealerContribution = Math.max.apply(
    null,
    healerPartnerData.map((data) =>
      data.buffContribution === null ? 0 : data.buffContribution
    )
  );
  let maxDpsContribution = Math.max.apply(
    null,
    dpsPartnerData.map((data) =>
      data.buffContribution === null ? 0 : data.buffContribution
    )
  );

  let maxContribution = Math.max(
    maxTankContribution,
    maxHealerContribution,
    maxDpsContribution
  );

  return (
    <Grid
      container
      sx={{
        width: "95%",
        color: "white",
      }}
    >
      <Grid container item xs={6} direction="column" spacing={3}>
        <Grid item>
          {ContributionTableSingleRole(
            "Tanks",
            tankPartnerData,
            maxContribution,
            maxTankContribution
          )}
        </Grid>
        <Grid item>
          {ContributionTableSingleRole(
            "Healers",
            healerPartnerData,
            maxContribution,
            maxHealerContribution
          )}
        </Grid>
      </Grid>

      <Grid item xs={6}>
        {ContributionTableSingleRole(
          "DPS",
          dpsPartnerData,
          maxContribution,
          maxDpsContribution
        )}
      </Grid>
    </Grid>
  );
}

function ContributionTableSingleRole(
  role: string,
  simulationData: Array<BestPartnerSimulationData>,
  maxContribution: number,
  maxRoleContribution: number
) {
  let buffBars = simulationData.map((data, index) => {
    let contribution =
      data.buffContribution === null ? 0 : data.buffContribution;
    let BuffBarBox = styled(Box)`
      ${BuffBarBoxStyle((100 * contribution) / maxRoleContribution)}
    `;

    let Bar = styled(Box)`
      ${BestPartnerBuffBarStyle(
      Math.min(index, MAX_RANKINGS),
      RANKING_ITEM_HEIGHT
    )}
    `;

    // UI상 표시값 사이 일치를 위해 rounding한 값을 더한 거로 totalRdps를 한 번 정규화해준다.
    let roundedRdps = Math.round((100 * contribution) / maxContribution);

    return (
      <Box display="flex" marginY={1} height={RANKING_ITEM_HEIGHT}>
        <Box height={RANKING_ITEM_HEIGHT} display="flex" alignItems={"center"}>
          <img
            src={jobAbbrevToJobIconPath(data.jobAbbrev)}
            alt={"rdps"}
            width={25}
            height={25}
          />
        </Box>
        <Box width="80%" marginLeft="2%" marginRight="2%">
          <BuffBarBox>
            <Bar />
          </BuffBarBox>
        </Box>
        <Box display="flex" alignItems="center">
          <Typography variant="body1" fontSize={12}>
            {roundedRdps}%
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
    >
      <Typography variant="h6" align="center">
        {role}
      </Typography>
      {buffBars}
    </Box>
  );
}
