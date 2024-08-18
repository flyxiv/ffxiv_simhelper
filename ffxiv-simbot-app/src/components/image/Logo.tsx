import "./Logo.css";
import { Link } from "react-router-dom";
const logo = "src/assets/images/logo_dark.svg";

export const Logo = () => {
  return (
    <div className="TitleDiv">
      <Link to="/">
        <img
          src={logo}
          alt="logo"
          height="100px"
          width="200px"
          className="LogoImg"
        />
      </Link>
    </div>
  );
};
