import { AppConfigurations } from "../../Themes";
const rdpsColors = ["#FF4848", "#FF6C6C", "#FF9999", "#FFB7B7", "#FFDADA"];
export const imageSize = "30px";
export const barHeight = "15px";

export const GraphBoxStyle = `
    width: 100%;
    background-color: ${AppConfigurations.backgroundTwo};
    align-items: center;
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
    display: flex;
    justify-content: center;
`;

export const BestPartnerBuffBarStyle = (index: number, height: string) => `
    background-color: ${rdpsColors[index]};
    width: 100%;
    display: flex;
    justify-content: center;
    height: ${height};
`;

export const PartyMemberBuffBoxStyle = `
    margin-top: 20px;
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
    padding-left: 10px;
    justify-items: flex-end;
    align-items: center;
`;

export const TotalRdpsBoxStyle = `
    color: white;
    width: 15%;
    margin-left: 1vw;
`;

export const BuffTitleBarStyle = `
    width: 100%;
`;

export const SkillBoxStyle = `
    padding-bottom: 10px;
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    color: white;
    width: 100%;
    margin-bottom: 30px;
`;

export const SkillNameStyle = `
    height: ${imageSize};
    display: flex;
    align-items: center;
    justify-content: flex-end;
    margin-right: 10px;
`;

export const SkillIconBoxStyle = `
    display: flex;
    flex-direction: column;
    justify-content: left;
    padding-left: 0.5vw;
    width: 40%; 
    max-width: 50px;
    margin-right: 5px;
`;

export const SkillBarBoxStyle = `
    width: 80%;
    align-items: center;
    height: ${imageSize};
    display: flex;
`;
export const SkillBarStyle = (portion: number) => `
  width: ${portion}%;
  background-color: ${AppConfigurations.primaryVariant};
  height: ${barHeight};
`;

export const SkillPercentageBoxStyle = `
  width: 15%;
  height: ${imageSize};
  display: flex;
  align-items: center;
  justify-content: right;
`;

export const TotalDamageBoxStyle = `
    width: 25%;
    height: ${imageSize};
    display: flex;
    align-items: center;
    justify-content: right;
    margin-right: 1vw;
`;

export const SkillCountBoxStyle = `
    width: 15%;
    height: ${imageSize};
    align-items: center;
    display: flex;
    justify-content: right;
`;


