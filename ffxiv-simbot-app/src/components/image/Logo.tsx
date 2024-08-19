import "./Logo.css";
import { Link } from "react-router-dom";

export const Logo = () => {
  return (
    <div className="TitleDiv">
      <Link to="/">
        <img
          src='/images/logo_dark.svg'
          alt="logo"
          height="100px"
          width="200px"
          className="LogoImg"
        />
      </Link>
    </div>
  );
};
