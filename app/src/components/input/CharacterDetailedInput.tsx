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
          type="number"
          value={mainCharacterState.value.weaponDamage}
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.weaponAttack(value);
          }}
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
          type="number"
          value={mainCharacterState.value.mainStat}
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.mainStat(value);
          }}
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
          type="number"
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.criticalStrike(value);
          }}
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
          type="number"
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.directHit(value);
          }}
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
          type="number"
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.determination(value);
          }}
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
          type="number"
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.speed(value);
          }}
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
          type="number"
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.tenacity(value);
          }}
        />
      </Grid>
    </Grid>
  );
}
