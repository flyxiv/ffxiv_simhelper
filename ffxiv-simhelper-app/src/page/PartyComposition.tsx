import { useState } from "react";
import { Box, styled } from "@mui/material";
import { BODY_WIDTH, PARTY_COMPOSITION_URL } from "../App";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";
import { SelectionTitle } from "../components/basic/SelectionTitle";
import {
    CustomizeBoardStyle,
    InputContainerStyle,
} from "./Styles";
import { AppLanguageTexts } from "../const/languageTexts";
import { DemoWarningText } from "../components/basic/WarningText";
import { HorizontalPartyInputPartyComposition } from "../components/input/partyinput/HorizontalPartyInput";
import { ResultBoardBoxStyle } from "../components/container/Styles";
import { PartyCompositionGraph } from "../components/graph/CompositionBarChart";
import alasql from "alasql";
import { PartyCompositionChartData } from "../components/graph/GraphData";
import partyRdpsTableJson from "../assets/data/party_rdps_table.json";

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

const MAX_COMPOSITION_COUNT = 50;

interface PartyCompositionRdpsData {
    tank1: string;
    tank2: string;
    healer1: string;
    healer2: string;
    melee: string;
    other: string;
    ranged: string;
    caster: string;
    rdps: number;
}


const rdpsData: PartyCompositionRdpsData[] = partyRdpsTableJson as PartyCompositionRdpsData[];


let PartyCompositionInputContainer = styled(Box)`
  ${InputContainerStyle},
`;

let CustomizeBoard = styled(Box)`
  ${CustomizeBoardStyle}
`;

export type PartyComposition = [string, string, string, string, string, string, string, string]
export const DEFAULT_COMPOSITION: PartyComposition = ["*", "*", "*", "*", "*", "*", "*", "*"];

function indexToRole(index: number): string {
    switch (index) {
        case 0:
            return "tank1";
        case 1:
            return "tank2";
        case 2:
            return "healer1";
        case 3:
            return "healer2";
        case 4:
            return "melee";
        case 5:
            return "other";
        case 6:
            return "ranged";
        default:
            return "caster";
    }
}

function toQueryString(partyComposition: PartyComposition): string {
    let hasFilter = partyComposition.find((job) => job !== "*") !== undefined;

    if (!hasFilter) {
        return "";
    }

    let queryString = "WHERE ";
    let filters = [];

    for (let i = 0; i < partyComposition.length; i++) {
        if (partyComposition[i] !== "*") {
            filters.push(`${indexToRole(i)} = '${partyComposition[i]}'`);
        }
    }

    return queryString + filters.join(" AND ");
}

function toPartyCompositionChartData(partyComposition: PartyCompositionRdpsData): PartyCompositionChartData {
    let key = [partyComposition.tank1, partyComposition.tank2, partyComposition.healer1, partyComposition.healer2, partyComposition.melee, partyComposition.other, partyComposition.ranged, partyComposition.caster];

    return {
        key,
        totalRdps: partyComposition.rdps
    }
}

export function PartyComposition() {
    let LANGUAGE_TEXTS = AppLanguageTexts();

    let [partyComposition, setPartyComposition] = useState(DEFAULT_COMPOSITION);
    let queryString = `SELECT * FROM ? ${toQueryString(partyComposition)} LIMIT ${MAX_COMPOSITION_COUNT}`;
    let currentRecommendedPartyCompositions: PartyCompositionRdpsData[] = alasql(queryString, [rdpsData]);
    let partyCompositionChartData = currentRecommendedPartyCompositions.map(toPartyCompositionChartData);

    return (
        <>
            <Box
                display="flex"
                sx={{ backgroundColor: AppConfigurations.backgroundOne }}
                width={"100%"}
            >
                {BasicLeftMenu(
                    PARTY_COMPOSITION_URL,
                    LANGUAGE_TEXTS
                )}
                <Box
                    width={BODY_WIDTH}
                    display="flex"
                    alignItems={"center"}
                    flexDirection={"column"}
                >
                    {AppHeader()}
                    {DemoWarningText(LANGUAGE_TEXTS.DEMO_WARNING_TEXT)}
                    <Box alignContent={"center"} width="100%">
                        <PartyCompositionInputContainer>
                            {SelectionTitle(
                                LANGUAGE_TEXTS.DPS_ANALYSIS_PARTY_INPUT_INFO_TEXT
                            )}
                            <CustomizeBoard>
                                {HorizontalPartyInputPartyComposition(
                                    partyComposition,
                                    setPartyComposition,
                                    LANGUAGE_TEXTS
                                )}
                            </CustomizeBoard>
                        </PartyCompositionInputContainer>

                    </Box>
                    <Box>
                        <ResultBoardBox>
                            {SelectionTitle(LANGUAGE_TEXTS.OVERALL_TEXT)}
                            {PartyCompositionGraph(partyCompositionChartData)}
                        </ResultBoardBox>
                        <Box />
                        {Footer()}
                    </Box>
                </Box>
            </Box>
        </>
    );
}
