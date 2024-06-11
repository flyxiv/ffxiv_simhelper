import { Divider, List, ListItem, Typography } from "@mui/material";
import { SimulationData } from "src/types/QuickSimResponse";

const style = {
  py: 0,
  width: "90%",
  maxWidth: 300,
  borderRadius: 2,
  border: "1px solid",
  borderColor: "divider",
  backgroundColor: "white",
};

export const Summary = (props: SimulationData) => {
  const summary = props.simulationSummary;

  return (
    <List sx={style}>
      <ListItem>
        <Typography variant="h6">RDPS: {summary.rdps}</Typography>
      </ListItem>
      <Divider component="li" />
      <ListItem>
        <Typography variant="h6">PDPS: {summary.pdps}</Typography>
      </ListItem>
      <Divider component="li" />
      <ListItem>
        <Typography variant="h6">ADPS: {summary.adps}</Typography>
      </ListItem>
      <Divider component="li" />
      <ListItem>
        <Typography variant="h6">EDPS: {summary.edps}</Typography>
      </ListItem>
    </List>
  );
};
