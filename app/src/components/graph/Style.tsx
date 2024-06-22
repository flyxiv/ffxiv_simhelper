import { ColorConfigurations } from "src/Themes";
const rdpsColors = ["#FF4848", "#FF6C6C", "#FF9999", "#FFB7B7", "#FFDADA"];
export const imageSize = "30px";
export const barHeight = "15px";

export const GraphBoxStyle = `
width: 100%;
background-color: ${ColorConfigurations.backgroundTwo};
`;

export const GraphEntryStyle = `
    display: flex;
    flex-direction: row;
    justify-content: left;
    align-items: center;
`;

export const BuffBarBoxStyle = (portion: number) => `
  display: flex;
  flex-direction: column;
  color: white;
  justify-content: center;
  align-items: center;
  width: ${portion}%;
`;

export const TotalBarStyle = `
    display: flex;
    flex-direction: row;
    width: 100%;
    align-items: flex-start;
    justify-content: flex-start;
`;
export const BuffBarStyle = (index: number) => `
    background-color: ${rdpsColors[index]};
    width: 100%;
`;

export const PartyMemberBuffBoxStyle = `
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: flex-start;
    width: 100%;
    color: white;
`;

export const PartyMemberIconBoxStyle = `
    display: flex;
    width: 10%;
    flex-direction: column;
    margin-top: 3%;
    padding-left: 10px;
`;

export const TotalRdpsBoxStyle = `
    color: white;
    width: 10%;
    margin-top: 2.7%;
    margin-left: 1vw;
    margin-right: 0.5vw;
`;

export const BuffTitleBarStyle = `
    margin-top: 2.7%;
    width: 80%;
`;

export const SkillBoxStyle = `
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    color: white;
    width: 100%;
`;

export const SkillNameStyle = `
    width: 5vw;
    height: ${imageSize};
    display: flex;
    align-items: center;
    justify-content: right;
    padding-right: 1vw;
`;

export const SkillIconBoxStyle = `
    display: flex;
    flex-direction: column;
    justify-content: left;
    width: 3vw;
    padding-left: 0.5vw;
`;

export const SkillBarBoxStyle = `
    width: 22vw;
    align-items: center;
    height: ${imageSize};
    display: flex;
`;
export const SkillBarStyle = (portion: number) => `
  width: ${portion}%;
  background-color: ${ColorConfigurations.primaryVariant};
  height: ${barHeight};
`;

export const SkillPercentageBoxStyle = `
  width: 4vw;
  height: ${imageSize};
  display: flex;
  align-items: center;
  justify-content: right;
`;

export const TotalDamageBoxStyle = `
    width: 5vw;
    height: ${imageSize};
    display: flex;
    align-items: center;
    justify-content: right;
    margin-right: 1vw;
`;

export const SkillCountBoxStyle = `
    width: 3vw;
    height: ${imageSize};
    align-items: center;
    display: flex;
    justify-content: right;
`;

export const SkillTitleBoxStyle = `
    width: 9vw;
    height: ${imageSize};
    display: flex;
    align-items: center;
    justify-content: center;
`;

export const SkillPercentageTitleBoxStyle = `
  width: 4.5vw;
  height: ${imageSize};
  display: flex;
  align-items: center;
  justify-content: right;
`;
