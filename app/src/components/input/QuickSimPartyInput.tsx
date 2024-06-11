import { Grid, TextField } from "@mui/material";
import { JobSelection } from "./JobSelection";

export function QuickSimPartyInput(
  playerIds: number[],
  partyJobs: string[],
  partySetter: React.Dispatch<React.SetStateAction<string[]>>,
  time: number,
  timeSetter: Function
) {
  return (
    <Grid container spacing={2}>
      <Grid item xs={3}>
        <TextField
          label="Combat Time(in seconds)"
          value={time}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          onChange={(e) => timeSetter(e.target.value)}
          key="time"
          fullWidth
        />
      </Grid>
      {playerIds.map((playerId) => (
        <Grid item xs={3}>
          {JobSelection(playerId, partyJobs, partySetter)}
        </Grid>
      ))}
    </Grid>
  );
}
