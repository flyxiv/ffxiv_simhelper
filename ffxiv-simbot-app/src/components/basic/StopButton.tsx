import { Box, Button } from "@mui/material";
import { AppConfigurations } from "../../Themes";

export function StopButton() {
  return (
    <Button
      variant="contained"
      onClick={() => {
        window.location.reload();
      }}
      sx={{
        backgroundColor: AppConfigurations.alert,
        marginLeft: 2,
        "&:hover": {
          backgroundColor: AppConfigurations.alert,
        },
      }}
    >
      Cancel
    </Button>
  );
}
