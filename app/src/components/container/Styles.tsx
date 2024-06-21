import { ColorConfigurations } from "src/Themes";

export const SummaryBoardBoxStyle = `
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
