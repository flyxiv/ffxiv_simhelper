import { Link } from "react-router-dom";
import { SimUIIcon } from "../components/image/SimUIIcon";
import "./Home.css";

const quickSimPageName = "quicksim";
const quickSimPagePath = `/${quickSimPageName}`;
const quickSimButtonImagePath = "src/assets/images/quickstart.png";

const gearCompareSimPageName = "statcompare";
const gearCompareSimPagePath = `/${gearCompareSimPageName}`;
const gearCompareSimButtonImagePath = "src/assets/images/statcompare.png";

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
