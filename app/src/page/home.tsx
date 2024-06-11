import { Link } from "react-router-dom";
import { QuickSimUIIcon } from "../components/image/QuickSimUIIcon";
import "./Home.css";

export function Home() {
  return (
    <div className="HomeBody">
      <div className="LeftBody">
        <Link to="/quicksim">
          <QuickSimUIIcon />
        </Link>
      </div>
      <div className="RightBody">Hello</div>
    </div>
  );
}
