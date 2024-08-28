import { ColorConfigurations } from "../../Themes";
export const plusBackgroundColor = "#7B9FD3";
export const minusBackgroundColor = "#CF6679";
export const plusTextColor = "#00476F";
export const minusTextColor = "#850B21";

export const statusBoxWidth = "22vw";
export const ResultBoardBoxStyle = `
  background-color: ${ColorConfigurations.backgroundTwo};
  width: 45vw;
  align-items: center;
  justify-content: center;
  display: flex;
  flex-direction: column;
`;

export const TitleBoxStyle = `
  background-color: white;
  color: black;
  width: 25vh;
  padding: 10px;
  margin-top: 20px;
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
  width: 8vw;
  height: 13vh;
  padding: 10px;
  margin: 0.4vw;
  align-items: center;
  justify-content: center;
  aspect-ratio: 1 / 1;
`;

export const PlayerInfoBoxStyle = `
  width: 40vw;
  margin: 1vh;
  background: ${ColorConfigurations.backgroundThree};
`;

export const PlayerInfoJobTitleStyle = `
  color: white;
  align-items: center;
  justify-content: center;
  display: flex;
  padding: 10px;
`;

export const PlayerStatInfoBoxStyle = `
  width: 40vw;
  background: ${ColorConfigurations.backgroundThree};
`;

export const StatOneLineBoxStyle = `
  width: 100%;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
`;

export const StatBoxStyle = `
  width: 8vw;
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
  background-color: ${ColorConfigurations.backgroundThree};
  display: flex;
  width: 45vw;
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
  width: 45vw;
`;

export const SkillLogCombatTimeBoxStyle = `
  color: white;
  width: 5vw;
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
  width: 12vw;
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
  background-color: ${ColorConfigurations.backgroundTwo};
  width: 50%;
  margin: auto;
`;

export const SingleStatBoxStyle = `
  background-color: ${ColorConfigurations.backgroundThree};
  color: white;
  width: 4vw;
  margin: 5px 3px 5px 3px;
`;

export const EquipmentSubStatBoxStyle = `
  width: 80%;
  background-color: ${ColorConfigurations.backgroundThree};
  display: flex;
`;

export const EquipmentSingleSubStatBoxStyle = (numberOfSubStats: number) => `
  width: ${100 / numberOfSubStats}%;
  background-color: ${ColorConfigurations.backgroundThree};
`;

export const EquipmentSingleBoxStyle = `
  border: 1px solid ${"white"};
`;

export const LeftMenuLogoStyle = `
  background-color: ${ColorConfigurations.primary};   
  color: white;
  height: 5vh;
  align-content: center;
  justify-content: center;
`

export const LeftMenuTotalBarStyle = `
  background-color: ${ColorConfigurations.backgroundThree};
  color: white;
  margin-left: 10px;
  margin-top: 10px;
  height: 100vh;
`

export const LeftMenuNavigationBarStyle = `
  color: white;
`

export const LeftMenuNavigationItemStyle = `
  color: white;
`
