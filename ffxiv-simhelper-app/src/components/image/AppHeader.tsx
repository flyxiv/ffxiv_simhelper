import { Box, Link } from "@mui/material";
import { Logo, LOGO_SIZE } from "./Logo";

export function AppHeader() {
  return (
    <Box
      className="LogoBox"
      display="flex"
      width="100%"
      justifyContent="center"
      sx={{ height: LOGO_SIZE }}
    >
      <Link href="/">
        <Logo />
      </Link>
    </Box>
  );
}
