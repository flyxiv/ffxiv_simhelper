import { Box, Grid, Typography } from "@mui/material";
import { BestPartnerSimulationData, SimulationDataByRole } from "../../types/ffxivdatabase/PartyCompositionMaker";

export function ContributionByRoleTable(simulationDataByRole: SimulationDataByRole, mainPlayerJobAbbrev: string) {
    let tankPartnerData = simulationDataByRole.tanks;
    let healerPartnerData = simulationDataByRole.healers;
    let meleePartnerData = simulationDataByRole.melee;
    let rangedPartnerData = simulationDataByRole.ranged;
    let casterPartnerData = simulationDataByRole.casters;

    return (
        <>
            {ContributionTableSingleRole("Tanks", tankPartnerData)}
        </>
    )
}

function ContributionTableSingleRole(role: string, simulationData: Array<BestPartnerSimulationData>) {
    return (
        <Grid>
            <Grid item xs={4} sx={{ backgroundColor: "black" }}>
                <Box>
                    <Typography variant="h6" align="center">
                        {role}
                    </Typography>
                    {simulationData.map((data) => {
                        return (
                            <Typography variant="body1" align="center">
                                {data.jobAbbrev}: {data.buffContribution}
                            </Typography>
                        )
                    })}
                </Box>
            </Grid>
        </Grid>
    )
}