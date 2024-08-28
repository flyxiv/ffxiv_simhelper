import { Link } from "react-router-dom";
import { SimUIIcon } from "../components/image/SimUIIcon";
import "./Home.css";
import { AppHeader } from "../components/image/AppHeader";
import { Footer } from "../components/basic/Footer";
import { Box } from "@mui/material";
import { ColorConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";

const quickSimPageName = "quicksim";
const quickSimPagePath = `/${quickSimPageName}`;
const quickSimButtonImagePath = "/images/quickstart.png";

const gearCompareSimPageName = "statcompare";
const gearCompareSimPagePath = `/${gearCompareSimPageName}`;
const gearCompareSimButtonImagePath = "/images/statcompare.png";

export function Home() {
  return (
    <Box sx={{ backgroundColor: ColorConfigurations.backgroundOne }}>
      <Box display="flex">
        {BasicLeftMenu()}
        <Box>
          {AppHeader()}
          <Box className="HomeBody">
            <div className="LeftBody">
              <Link to={quickSimPagePath}>
                {SimUIIcon(
                  quickSimButtonImagePath,
                  "quicksimimage",
                  quickSimPageName
                )}
              </Link>
            </div>
            <div className="RightBody">
              <Link to={gearCompareSimPagePath}>
                {SimUIIcon(
                  gearCompareSimButtonImagePath,
                  "gearcompareimage",
                  gearCompareSimPageName
                )}
              </Link>
            </div>
          </Box>
          {Footer()}
        </Box>
      </Box>
    </Box>
  );
}
