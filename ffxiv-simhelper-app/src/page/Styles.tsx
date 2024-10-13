import { AppConfigurations } from "../Themes"

export const CharacterDetailCustomizeBoardStyle = (width: string) => `
    display: flex;
    flex-direction: column;
    width: ${width};
`

export const InputContainerStyle = (width: string) => `
    flex-direction: row;
    margin-left: auto;
    margin-right: auto;
    align-items: flex-start;
    width: ${width};
`

export const CustomizeBoardStyle = `
    margin: auto;
    padding-top: 10px;
    padding-bottom: 10px;
    border: 1px solid black;
    background-color: ${AppConfigurations.backgroundTwo};
    width: 100%;
`

export const EquipmentBoardStyle = `
    align-items: center;
    border: 1px solid black;
    background-color: ${AppConfigurations.backgroundTwo};
    width: 100%;
`

export const CustomizeBoardSinglePartyInputStyle = (width: string) => `
    display: flex;
    margin: auto;
    justify-content: center;
    padding-top: 10px;
    padding-bottom: 10px;
    border: 1px solid black;
    background-color: ${AppConfigurations.backgroundTwo};
    width: ${width};
`