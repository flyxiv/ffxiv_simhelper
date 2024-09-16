import { Box } from "@mui/material";
import "./Logo.css";
import { Link } from "react-router-dom";

export const LOGO_SIZE = "15vh";

export const Logo = () => {
  return (
    <Link to="/">
      <Box
        component={"img"}
        src="/images/logo_dark.svg"
        alt="logo"
        sx={{ height: LOGO_SIZE }}
        className="LogoImg"
      />
    </Link>
  );
};
