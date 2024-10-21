import { Box } from "@mui/material";
import "./Logo.css";
import { Link } from "react-router-dom";
import LOGO_IMAGE from "/src/assets/images/logo_dark.svg";

export const LOGO_SIZE = "15vh";


export const Logo = () => {
  return (
    <Link to="/">
      <Box
        component={"img"}
        src={LOGO_IMAGE}
        alt="logo"
        sx={{ height: LOGO_SIZE }}
        className="LogoImg"
      />
    </Link>
  );
};
