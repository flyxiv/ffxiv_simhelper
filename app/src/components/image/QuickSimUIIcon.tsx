import "./QuickSimUIIcon.css";
import { Link } from "react-router-dom";

const buttonImagePath = process.env.PUBLIC_URL + "/images/quickstart.png";

export const QuickSimUIIcon = () => {
  return (
    <Link to="./quicksim">
      <div className="QuickSimIcon">
        <img
          src={buttonImagePath}
          alt="quicksim-button"
          className="ButtonIcon"
        />
      </div>
    </Link>
  );
};
