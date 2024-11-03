import {
	MenuItem,
	Typography,
	Divider,
	Box,
} from "@mui/material";
import { JobMenuItem } from "../../items/JobMenuItem";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { AppConfigurations } from "../../../Themes";
import { EquipmentInput } from "../../../types/EquipmentInput";
import { isHealer, isTank } from "../../../types/ffxivdatabase/PlayerPower";
import {
	BUFF_JOBS_LIST,
	CASTER_JOBS,
	DPS_BUFF_JOBS,
	DPS_JOBS,
	HEALER_JOBS,
	MELEE_JOBS,
	OTHER_DPS_JOBS,
	RANGED_JOBS,
	TANK_JOBS,
} from "../../../types/ffxivdatabase/PartyCompositionMaker";
import { calculatePlayerPowerFromInputs } from "../../../types/ffxivdatabase/ItemSet";
import { PLD_EN_NAME, SCH_EN_NAME, TextDictionary } from "../../../const/languageTexts";
import { ITEM_BOTTOM_MENU_MIN_HEIGHT } from "../../items/Styles";
import { PartyComposition } from "../../../page/PartyComposition";
import { BottomMenuInput } from "../basicform/EquipmentInputForm";

let ALIGN = "center";

function filterDuplicateBuffJobs(
	jobList: Array<string>,
	mainCharacterJob: string,
	partyMemberJobAbbrevs: Array<string>
) {
	return jobList.filter((jobAbbrev) => {
		if (!BUFF_JOBS_LIST.includes(jobAbbrev)) {
			return true;
		}

		return (
			jobAbbrev !== mainCharacterJob &&
			!partyMemberJobAbbrevs.includes(jobAbbrev)
		);
	});
}

export function getRoleByIdAndMainCharacterJob(
	id: number,
	mainCharacterJob: string,
	partyMemberJobAbbrevs: Array<string>
) {
	let otherPartyMemberJobAbbrevs = partyMemberJobAbbrevs.filter(
		(_, index) => index !== id - 1
	);
	let tank_jobs = TANK_JOBS;
	let healer_jobs = filterDuplicateBuffJobs(
		HEALER_JOBS,
		mainCharacterJob,
		otherPartyMemberJobAbbrevs
	);
	let dps_jobs = filterDuplicateBuffJobs(
		DPS_JOBS,
		mainCharacterJob,
		otherPartyMemberJobAbbrevs
	);

	if (id == 1) {
		return tank_jobs;
	}

	if (id == 2) {
		if (isTank(mainCharacterJob)) {
			return healer_jobs;
		} else {
			return tank_jobs;
		}
	}

	if (id == 3) {
		return healer_jobs;
	}

	if (id == 4) {
		if (isTank(mainCharacterJob) || isHealer(mainCharacterJob)) {
			return dps_jobs;
		} else {
			return healer_jobs;
		}
	}

	return dps_jobs;
}

export function PartyMemberJobSelection(
	id: number,
	totalEquipmentState: EquipmentInput,
	setTotalState: Function,
	LANGUAGE_TEXTS: TextDictionary
) {
	const updateState = (index: number) => (value: string) => {
		const newJobNames =
			totalEquipmentState.equipmentDatas[0].partyMemberJobAbbrevs.map(
				(jobName, i) => {
					if (i === index) {
						return value;
					}
					return jobName;
				}
			);

		let newAvailablePartyIds =
			totalEquipmentState.equipmentDatas[0].partyMemberIds;
		newAvailablePartyIds = newAvailablePartyIds.filter(
			(partyId) => partyId !== id
		);

		if (value !== "Empty") {
			newAvailablePartyIds.push(id);
		}
		newAvailablePartyIds.sort((a, b) => a - b);

		let newTotalState = { ...totalEquipmentState };

		newTotalState.equipmentDatas.forEach((data) => {
			data.partyMemberJobAbbrevs = newJobNames;
			data.partyMemberIds = newAvailablePartyIds;
			let updatePower = calculatePlayerPowerFromInputs(data);
			data.power = updatePower;

			if (data.mainPlayerPartner1Id === id) {
				data.mainPlayerPartner1Id = null;
			}

			if (data.mainPlayerPartner2Id === id) {
				data.mainPlayerPartner2Id = null;
			}
		});

		setTotalState({ ...newTotalState });
	};

	let key = `job-select-partymember-${id}`;

	let jobAbbrevs = getRoleByIdAndMainCharacterJob(
		id,
		totalEquipmentState.equipmentDatas[0].mainPlayerJobAbbrev,
		totalEquipmentState.equipmentDatas[0].partyMemberJobAbbrevs
	);

	let playerLabel = jobAbbrevs.includes(SCH_EN_NAME)
		? LANGUAGE_TEXTS.HEALER_TEXT
		: jobAbbrevs.includes(PLD_EN_NAME)
			? LANGUAGE_TEXTS.TANK_TEXT
			: LANGUAGE_TEXTS.DPS_TEXT;

	return (
		<CustomFormControl fullWidth>
			<BottomMenuInput
				select
				id={key}
				value={totalEquipmentState.equipmentDatas[0].partyMemberJobAbbrevs[id - 1]}
				key={key}
				label={playerLabel}
				onChange={(e: React.ChangeEvent<HTMLInputElement>): void => updateState(id - 1)(e.target.value)}
				SelectProps={{
					MenuProps: {
						PaperProps: {
							sx: {
								backgroundColor: AppConfigurations.backgroundThree,
							},
						},
					},
				}}
			>
				{jobAbbrevs.map((jobAbbrev) => {
					return JobMenuItem(jobAbbrev, ALIGN, LANGUAGE_TEXTS, false);
				})}

				<Divider />
				<MenuItem value="Empty">
					<Box height={ITEM_BOTTOM_MENU_MIN_HEIGHT} display="flex" justifyContent={"center"} alignItems={"center"}>
						<Typography variant="body1" color="white" fontSize={AppConfigurations.body1FontSize} align="center">
							Empty
						</Typography>
					</Box>
				</MenuItem>
			</BottomMenuInput>
		</CustomFormControl>
	);
}


export enum PartyPosition {
	Tank1 = 0,
	Tank2 = 1,
	Healer1 = 2,
	Healer2 = 3,
	Melee = 4,
	Other = 5,
	Ranged = 6,
	Caster = 7,
}

function getValidJobForSlot(
	id: number,
	partyComposition: PartyComposition,
) {
	switch (id as PartyPosition) {
		case PartyPosition.Tank1:
		case PartyPosition.Tank2:
			return TANK_JOBS;
		case PartyPosition.Healer1:
			return HEALER_JOBS.filter((job) => job !== partyComposition[3]);
		case PartyPosition.Healer2:
			return HEALER_JOBS.filter((job) => job !== partyComposition[2]);
		case PartyPosition.Melee:
			let possibleMeleeJobs = [];
			for (let job of MELEE_JOBS) {
				if (!(DPS_BUFF_JOBS.includes(job)) || (partyComposition[PartyPosition.Other] !== job)) {
					possibleMeleeJobs.push(job);
				}
			}
			return possibleMeleeJobs;
		case PartyPosition.Other:
			let possibleJobs = [];
			for (let job of OTHER_DPS_JOBS) {
				if (!(DPS_BUFF_JOBS.includes(job)) || ((partyComposition[PartyPosition.Melee] !== job) && (partyComposition[PartyPosition.Caster] !== job))) {
					possibleJobs.push(job);
				}
			}
			return possibleJobs;
		case PartyPosition.Caster:
			let possibleCasterJobs = [];

			for (let job of CASTER_JOBS) {
				if (!(DPS_BUFF_JOBS.includes(job)) || (partyComposition[PartyPosition.Other] !== job)) {
					possibleCasterJobs.push(job);
				}
			}
			return possibleCasterJobs;
		default:
			return RANGED_JOBS
	}
}


export function PartyMemberJobSelectionPartyComposition(
	id: number,
	partyComposition: PartyComposition,
	setPartyComposition: Function,
	LANGUAGE_TEXTS: TextDictionary
) {
	const updateState = (index: number) => (value: string) => {
		let newPartyComposition = [...partyComposition];
		newPartyComposition[index] = value;

		setPartyComposition(newPartyComposition);
	};

	let key = `job-select-partycomposition-${id}`;
	let possiblejobAbbrevs = getValidJobForSlot(id, partyComposition);

	let playerLabelText = id < PartyPosition.Healer1 ? LANGUAGE_TEXTS.TANK_TEXT : id < PartyPosition.Melee ? LANGUAGE_TEXTS.HEALER_TEXT : LANGUAGE_TEXTS.DPS_TEXT;
	let playerId = id < PartyPosition.Healer1 ? id + 1 : id < PartyPosition.Melee ? id - 1 : id - 3;
	let playerLabel = `${playerLabelText} ${playerId}`;

	return (
		<CustomFormControl fullWidth>
			<BottomMenuInput
				select
				id={key}
				value={partyComposition[id]}
				key={key}
				label={playerLabel}
				onChange={(e: React.ChangeEvent<HTMLInputElement>): void => updateState(id)(e.target.value)}
				SelectProps={{
					MenuProps: {
						PaperProps: {
							sx: {
								backgroundColor: AppConfigurations.backgroundThree,
							},
						},
					},
				}}
			>
				{possiblejobAbbrevs.map((jobAbbrev) => {
					return JobMenuItem(jobAbbrev, ALIGN, LANGUAGE_TEXTS, false);
				})}

				<Divider />
				<MenuItem value="*">
					<Box height={ITEM_BOTTOM_MENU_MIN_HEIGHT} display="flex" justifyContent={"center"} alignItems={"center"}>
						<Typography variant="body1" color="white" fontSize={AppConfigurations.body1FontSize} align="center">
							All
						</Typography>
					</Box>
				</MenuItem>
			</BottomMenuInput>
		</CustomFormControl>
	);
}