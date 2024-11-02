import { useState } from "react";
import { Box, styled } from "@mui/material";
import { BODY_WIDTH } from "../App";
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
import { PartyPosition } from "../components/input/jobselection/PartyMemberJobSelection";
import { SimulationTitle } from "../components/basic/SimulationTitle";

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
			if (i === PartyPosition.Other || i === PartyPosition.Melee || i === PartyPosition.Caster) {
				filters.push(`(${indexToRole(PartyPosition.Other)} = '${partyComposition[i]}' OR ${indexToRole(PartyPosition.Melee)} = '${partyComposition[i]}' OR ${indexToRole(PartyPosition.Caster)} = '${partyComposition[i]}')`);
			} else if (i === PartyPosition.Healer1 || i === PartyPosition.Healer2) {
				filters.push(`(${indexToRole(PartyPosition.Healer1)} = '${partyComposition[i]}' OR ${indexToRole(PartyPosition.Healer2)} = '${partyComposition[i]}')`);
			} else if (i === PartyPosition.Tank1 || i === PartyPosition.Tank2) {
				filters.push(`(${indexToRole(PartyPosition.Tank1)} = '${partyComposition[i]}' OR ${indexToRole(PartyPosition.Tank2)} = '${partyComposition[i]}')`);
			} else {
				filters.push(`${indexToRole(i)} = '${partyComposition[i]}'`);
			}
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

	// !!!!! rdpsData (=assets/data/party_rdps_table.json) MUST BE SORTED in descending order of rdps !!!!! 
	let minRdps = rdpsData[rdpsData.length - 1].rdps
	let maxRdps = rdpsData[0].rdps

	let queryString = `SELECT * FROM ? ${toQueryString(partyComposition)} LIMIT ${MAX_COMPOSITION_COUNT}`;
	let currentRecommendedPartyCompositions: PartyCompositionRdpsData[] = alasql(queryString, [rdpsData]);
	let partyCompositionChartData = currentRecommendedPartyCompositions.map(toPartyCompositionChartData);
	let maxRdpsOfPossibleComposition = partyCompositionChartData.map((data) => data.totalRdps).reduce((a, b) => Math.max(a, b), 0);

	return (
		<>
			<Box
				display="flex"
				sx={{ backgroundColor: AppConfigurations.backgroundOne }}
				width={"100%"}
			>
				{BasicLeftMenu(
					LANGUAGE_TEXTS.PARTY_COMPOSITION_PAGE_NAME,
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
					<Box width="100%" marginBottom={10}>
						<PartyCompositionInputContainer>
							{SelectionTitle(
								LANGUAGE_TEXTS.PARTY_COMPOSITION_INPUT_TEXT
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
					<Box alignContent={"center"} width="100%" display="flex" flexDirection="column" alignItems={"center"}>
						<ResultBoardBox>
							{SimulationTitle(LANGUAGE_TEXTS.PARTY_COMPOSITION_RESULT_TEXT)}
							{PartyCompositionGraph(partyCompositionChartData, minRdps, maxRdpsOfPossibleComposition, maxRdps)}
						</ResultBoardBox>
						<Box />
						{Footer()}
					</Box>
				</Box>
			</Box>
		</>
	);
}
