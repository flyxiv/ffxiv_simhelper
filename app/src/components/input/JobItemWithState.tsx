import { Box } from "@mui/material";
import { JobItem } from "./JobItem";

export function JobItemWithState(mainPlayerState: string) {
  return <Box justifyContent="center">{JobItem(mainPlayerState)}</Box>;
}
