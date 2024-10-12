import { AppConfigurations } from "../../Themes";

export const MenuItemStyle = `
  color: white;
  width: 100%;
  height: 5vh;
  justify-content: left;
  background-color: ${AppConfigurations.backgroundThree};

  &:hover {
    background-color: ${AppConfigurations.backgroundTwo};
  }
  &.Mui-selected {
    background-color: ${AppConfigurations.backgroundThree}; 

    &:hover {
      background-color: ${AppConfigurations.backgroundTwo}; 
    }
  }

  divider: true;
`;


export const ITEM_BOTTOM_MENU_MIN_HEIGHT = "5vh";
export const ITEM_TOP_MENU_MIN_HEIGHT = "5vh";