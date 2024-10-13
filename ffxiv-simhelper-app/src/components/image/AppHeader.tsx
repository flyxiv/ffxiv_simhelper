import { Box, Link } from "@mui/material";
import { Logo, LOGO_SIZE } from "./Logo";

export function AppHeader() {
  return (
    <Box
      className="LogoBox"
      display="flex"
      width="100%"
      minWidth={"800px"}
      justifyContent="center"
      sx={{ height: LOGO_SIZE, marginY: "30px" }}
    >
      <Link href="/">
        <Logo />
      </Link>
    </Box>
  );
}
