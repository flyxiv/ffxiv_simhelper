import { Typography, Box, styled } from "@mui/material";
import {
	TotalBarStyle,
	BuffBarBoxStyle,
	BuffBarStyle,
	PartyMemberBuffBoxStyle,
	TotalRdpsBoxStyle,
	PartyCompositionIconBoxStyle,
	PartyCompositionMobileIconBoxStyle,
} from "./Style";
import { PartyCompositionChartData } from "./GraphData";
import { AppConfigurations } from "../../Themes";
import { jobAbbrevToJobIcon } from "../icon/jobicon/JobIconFactory";
import { TextDictionary } from "../../const/languageTexts";
import { isMobile } from "../../util";

export const JOB_BAR_ITEM_HEIGHT = "65px";

const PartyMemberBuffBox = styled(Box)`
  ${PartyMemberBuffBoxStyle}
`;

const PartyCompositionIconBox = styled(Box)`
  ${PartyCompositionIconBoxStyle}
`;

const PartyCompositionMobileIconBox = styled(Box)`
  ${PartyCompositionMobileIconBoxStyle}
`;

const TotalBuffBox = styled(Box)`
  ${TotalBarStyle}
`;
const TotalRdpsBox = styled(Box)`
  ${TotalRdpsBoxStyle}
`;

const BuffBarBox = (width: number) => styled(Box)`
	${BuffBarBoxStyle(width)}
`

const BarredBox = (idx: number) => styled(Box)`
	${BuffBarStyle(idx)}
`

function JobBarChartPartyComposition(
	data: PartyCompositionChartData,
	minContribution: number,
	maxContributionOfPossibleCompositions: number,
	maxContribution: number,
	index: number
) {
	let totalRdpsRounded = Math.round(data.totalRdps);
	let rdpsText = `${totalRdpsRounded}(${(data.totalRdps / maxContribution * 100).toFixed(1)}%)`;
	let maxDiff = maxContributionOfPossibleCompositions - minContribution;
	let diff = data.totalRdps - minContribution;

	let BarBox = BuffBarBox((100 * diff) / maxDiff);

	let Bar = BarredBox(Math.min(index, 4));

	return (
		<PartyMemberBuffBox height={JOB_BAR_ITEM_HEIGHT} display="flex" flexDirection={"column"} justifyContent={"center"} alignItems={"center"}>
			<PartyCompositionIconBox>
				<Box height="40px" display="flex" alignItems="center">
					{data.key.map((key) => {
						return (
							<Box component="img"
								src={jobAbbrevToJobIcon(key)}
								alt={"rdps"}
								width={35}
								height={35}
								margin={0.5}
							/>
						);
					})}
				</Box>
			</PartyCompositionIconBox>
			<Box width="60%">
				<TotalBuffBox>
					<BarBox height="40px">
						<Bar sx={{ height: "100%" }} />
					</BarBox>
				</TotalBuffBox>
			</Box>
			<TotalRdpsBox sx={{ height: "60px" }}>
				<Typography variant="h6" align="center">{rdpsText}</Typography>
			</TotalRdpsBox>
		</PartyMemberBuffBox>
	);
}


function JobBarChartPartyCompositionMobile(
	data: PartyCompositionChartData,
	maxContribution: number,
) {
	let totalRdpsRounded = Math.round(data.totalRdps);
	let rdpsText = `${totalRdpsRounded}(${(data.totalRdps / maxContribution * 100).toFixed(1)}%)`;

	return (
		<PartyMemberBuffBox height="85px" display="flex" flexDirection={"column"} justifyContent={"center"} alignItems={"center"}>
			<PartyCompositionMobileIconBox height="60px">
				<Box display="flex" alignItems="center">
					{data.key.slice(0, 4).map((key) => {
						return (
							<Box component="img"
								src={jobAbbrevToJobIcon(key)}
								alt={"rdps"}
								width={25}
								height={25}
								margin={0.5}
							/>
						);
					})}
				</Box>
				<Box display="flex" alignItems="center">
					{data.key.slice(4, 8).map((key) => {
						return (
							<Box component="img"
								src={jobAbbrevToJobIcon(key)}
								alt={"rdps"}
								width={25}
								height={25}
								margin={0.5}
							/>
						);
					})}
				</Box>
			</PartyCompositionMobileIconBox>
			<TotalRdpsBox sx={{ height: "60px", width: "100%", marginX: 2, alignItems: "center", display: "flex" }}>
				<Typography variant="h6" align="right">{rdpsText}</Typography>
			</TotalRdpsBox>
		</PartyMemberBuffBox>
	);
}


export function GraphTitleRow(memberText: string, totalText: string) {
	return (
		<PartyMemberBuffBox sx={{ width: "100%" }}>
			<PartyCompositionIconBox sx={{ width: isMobile() ? "300px" : "440px" }}>
				<Typography variant="h6" fontSize={AppConfigurations.body1FontSize} align="center">
					{memberText}
				</Typography>
			</PartyCompositionIconBox>
			<Box width="60%" />

			<TotalRdpsBox sx={{ width: isMobile() ? "100%" : "15%" }} paddingRight={2}>
				<Typography variant="h6" fontSize={AppConfigurations.body1FontSize} align={isMobile() ? "left" : "center"}>
					{totalText}
				</Typography>
			</TotalRdpsBox>
		</PartyMemberBuffBox>
	);
}

export function PartyCompositionGraph(
	data: PartyCompositionChartData[],
	minContribution: number,
	maxContributionOfPossibleComposition: number,
	maxComposition: number,
	isMobile: boolean,
	LANGUAGE_TEXTS: TextDictionary
) {
	return (
		<Box width="80%" display="flex" flexDirection={"column"} justifyContent={"center"} alignItems={"center"}>
			{GraphTitleRow(LANGUAGE_TEXTS.PARTY_COMPOSITION_TEXT, LANGUAGE_TEXTS.TOTAL_RDPS_TEXT)}
			{data.map((entry, index) => {
				return isMobile ? JobBarChartPartyCompositionMobile(entry, maxComposition) : JobBarChartPartyComposition(entry, minContribution, maxContributionOfPossibleComposition, maxComposition, index);
			})}
		</Box>
	);
}