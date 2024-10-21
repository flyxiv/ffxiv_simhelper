import { Box, Typography } from "@mui/material";
import { AppConfigurations } from "../../Themes";
import { BuyMeACoffee } from "../../components/icon/BuyMeACoffeeIcon";

export function Footer() {
  return (
    <Box
      className="Footer"
      component="footer"
      sx={{
        width: "100%",
        border: "1px solid black",
        textAlign: "center",
        backgroundColor: AppConfigurations.backgroundOne,
      }}
      display="flex"
      justifyContent="center"
      flexDirection={"column"}
      color="white"
      height="20vh"
    >
      <Typography
        variant="caption"
        sx={{ fontSize: AppConfigurations.body2FontSize }}
      >
        <p>
          Creator: Fly Ninetynine@Aegis | Kkoo Eat@Aegis | Essnah{" "}
        </p>
        <p>Email: ns090200@gmail.com</p>
      </Typography>

      {BuyMeACoffee()}
    </Box>
  );
}
