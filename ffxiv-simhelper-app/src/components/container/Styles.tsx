import { AppConfigurations } from "../../Themes";
export const plusBackgroundColor = "#7B9FD3";
export const minusBackgroundColor = "#CF6679";
export const plusTextColor = "#00476F";
export const minusTextColor = "#850B21";

export const statusBoxWidth = "22vw";
export const ResultBoardBoxStyle = `
  width: 90%;
  min-height: 50vh;
  background-color: ${AppConfigurations.backgroundTwo};
  align-items: center;
  justify-content: flex-start;
  display: flex;
  flex-direction: column;
  padding-bottom: 100px;
`;

export const ResultBoardTopBoxStyle = `
  background-color: ${AppConfigurations.backgroundTwo};
  align-items: center;
  justify-content: flex-start;
  display: flex;
  flex-direction: column;
  width: 90%;
  padding-bottom: 20px;
`;

export const TitleBoxStyle = `
  background-color: white;
  color: black;
  padding-top: 10px;
  padding-bottom: 10px;
  padding-left: 40px;
  padding-right: 40px;

  margin-top: 20px;
  margin-bottom: 30px;
`;

export const DpsSummaryBoxStyle = `
  width: 100%;
  padding: 10px;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
`;

export const DpsBoxStyle = `
  width: 15%;
  aspect-ratio: 1;
  max-width: 200px;
  max-height: 80px;
  
  padding: 10px;
  margin: 0.4vw;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
`;

export const PlayerInfoBoxStyle = `
  width: 80%;
  margin: 3px;
  background: ${AppConfigurations.backgroundThree};
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: 10px;
  padding-bottom: 20px;
  padding-left: 30px;
  padding-right: 30px;
`;

export const PlayerInfoJobTitleStyle = `
  color: white;
  align-items: center;
  justify-content: center;
  display: flex;
  padding: 10px;
`;

export const PlayerStatInfoBoxStyle = `
  width: 80%;
  background: ${AppConfigurations.backgroundThree};
`;

export const StatOneLineBoxStyle = `
  width: 100%;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
`;

export const StatBoxStyle = `
  width: 10%;
  max-width: 50px;
  padding: 10px;
  margin: 0.4vw;
  align-items: center;
  justify-content: center;
`;

export const StatTitleTextBoxStyle = `
  color: white;
`;

export const StatTextBoxStyle = `
  width: 100%;
  background-color: white;
  color: black;
`;

export const SkillLogTableStyle = `
  background-color: ${AppConfigurations.backgroundThree};
  display: flex;
  flex-direction: column;
  justify-content: left;
  align-items: left;
  max-height: 100vh;
  overflow: auto;
  margin-top: 20px;
`;

export const SkillLogRowStyle = `
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  width: 100%;
`;

export const SkillLogCombatTimeBoxStyle = `
  color: white;
  width: 25%;
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: right;
`;

export const SkillEntityBoxStyle = `
  color: white;
  flex-direction: row;
  justify-content: left;
  align-items: center;
  display: flex;
  width: 100%;
  border: 1px solid white;
`;

export const SkillIconBoxStyle = `
  display: flex;
  flex-direction: row;
  justify-content: left;
  align-items: center;
  width: 25%;
  margin: 10px;
`;

export const StatusIconBoxStyle = (widthVw: string) => `
  display: flex;
  flex-direction: row;
  justify-content: left;
  align-items: center;
  width: ${widthVw};
`;

export const SkillLogCombatTimeTitleBoxStyle = `
  width: 5vw;
  color: white;
  justify-content: left;
`;

export const SkillIconBoxTitleStyle = `
  width: 12vw;
  color: white;
  display: flex;
  justify-content: center;
`;

export const StatusIconTitleBoxStyle = (widthVw: string) => `
  display: flex;
  flex-direction: row;
  justify-content: left;
  align-items: center;
  width: ${widthVw};
  color: white;
`;

export const StatSummaryBoxStyle = `
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  margin: auto;
`;

export const SingleStatBoxStyle = (width: string, maxWidth: number) => `
  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
  border: 1px solid #201F28;
  width: ${width};
  max-width: ${maxWidth}px;
  white-space: nowrap;
`;

export const EquipmentSubStatBoxStyle = `
  width: 80%;
  background-color: ${AppConfigurations.backgroundThree};
  display: flex;
`;

export const EquipmentSingleSubStatBoxStyle = (numberOfSubStats: number) => `
  width: ${100 / numberOfSubStats}%;
`;

export const EquipmentSingleBoxStyle = `
  border: 1px solid #201F28;
`;

export const LeftMenuLogoStyle = `
  background-color: ${AppConfigurations.primary};   
  color: white;
  height: 5vh;
  align-content: center;
  justify-content: center;
`;

export const LeftMenuTotalBarStyle = `
  background-color: ${AppConfigurations.backgroundThree};
  color: white;
  margin-left: 10px;
  margin-top: 10px;
`;

export const LeftMenuNavigationBarStyle = `
  color: white;
`;

export const LeftMenuNavigationItemStyle = `
  color: white;
`;
