import { Grid, TextField } from "@mui/material";
import { JobSelectionWithState } from "./JobSelectionWithState";
import { CharacterStates } from "src/types/CharacterStates";

export function CharacterDetailedInput(mainCharacterState: CharacterStates) {
  return (
    <Grid container spacing={2}>
      <Grid item xs={3}>
        {JobSelectionWithState(
          mainCharacterState.jobName,
          mainCharacterState.jobNameSetter
        )}
      </Grid>
      <Grid item xs={3}>
        <TextField
          label="Weapon Attack"
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          value={mainCharacterState.value.weaponDamage}
          onChange={(e) =>
            mainCharacterState.setter.weaponAttack(e.target.value)
          }
          fullWidth
        />
      </Grid>
      <Grid item xs={3}>
        <TextField
          label="Main Stat"
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          value={mainCharacterState.value.mainStat}
          onChange={(e) => mainCharacterState.setter.mainStat(e.target.value)}
          fullWidth
        />
      </Grid>
      <Grid item xs={3}>
        <TextField
          label="Critical Strike"
          value={mainCharacterState.value.criticalStrike}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) =>
            mainCharacterState.setter.criticalStrike(e.target.value)
          }
        />
      </Grid>
      <Grid item xs={3}>
        <TextField
          label="Direct Hit"
          value={mainCharacterState.value.directHit}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) => mainCharacterState.setter.directHit(e.target.value)}
        />
      </Grid>
      <Grid item xs={3}>
        <TextField
          label="Determination"
          value={mainCharacterState.value.determination}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) =>
            mainCharacterState.setter.determination(e.target.value)
          }
        />
      </Grid>
      <Grid item xs={3}>
        <TextField
          label="Speed"
          value={mainCharacterState.value.speed}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) => mainCharacterState.setter.speed(e.target.value)}
        />
      </Grid>
      <Grid item xs={3}>
        <TextField
          label="Tenacity"
          value={mainCharacterState.value.tenacity}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) => mainCharacterState.setter.tenacity(e.target.value)}
        />
      </Grid>
    </Grid>
  );
}
