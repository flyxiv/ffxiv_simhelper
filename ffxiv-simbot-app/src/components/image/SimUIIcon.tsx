import "./SimUIIcon.css";

export const SimUIIcon = (
  buttonImagePath: string,
  altText: string,
  pagePath: string
) => {
  console.log(buttonImagePath);
  return (
    <div className="SimIcon">
      <img src={buttonImagePath} alt={altText} className="ButtonIcon" />
    </div>
  );
};
