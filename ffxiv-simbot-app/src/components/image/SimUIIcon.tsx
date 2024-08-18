import "./SimUIIcon.css";
import { Link } from "react-router-dom";

export const SimUIIcon = (
  buttonImagePath: string,
  altText: string,
  pagePath: string
) => {
  let pageRelativeDirectory = `./${pagePath}`;
  console.log(buttonImagePath);
  return (
    <Link to={pageRelativeDirectory}>
      <div className="SimIcon">
        <img src={buttonImagePath} alt={altText} className="ButtonIcon" />
      </div>
    </Link>
  );
};
