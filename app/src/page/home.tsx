import { Link } from "react-router-dom";
import { SimUIIcon } from "../components/image/SimUIIcon";
import "./Home.css";
import { readEquipmentData } from "src/types/ffxivdatabase/Equipment";

const quickSimPageName = "quicksim";
const quickSimPagePath = `/${quickSimPageName}`;
const quickSimButtonImagePath =
  process.env.PUBLIC_URL + "/images/quickstart.png";

const gearCompareSimPageName = "statcompare";
const gearCompareSimPagePath = `/${gearCompareSimPageName}`;
const gearCompareSimButtonImagePath =
  process.env.PUBLIC_URL + "/images/statcompare.png";

export function Home() {
  return (
    <div className="HomeBody">
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
    </div>
  );
}
