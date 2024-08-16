import { Select, styled } from "@mui/material";
import { ColorConfigurations } from "src/Themes";

export const MenuItemStyle = `
  color: white;
  width: 100%;
  height: 5vh;
  justify-content: left;
  background-color: ${ColorConfigurations.backgroundThree};

  &:hover {
    background-color: ${ColorConfigurations.backgroundTwo};
  }
  &.Mui-selected {
    background-color: ${ColorConfigurations.backgroundThree}; 

    &:hover {
      background-color: ${ColorConfigurations.backgroundTwo}; 
    }
  }

  divider: true;
`;
