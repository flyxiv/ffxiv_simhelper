import { AppLanguageTexts } from "../../const/languageTexts";

export const loadJobRelatedInfos = () => {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  const BUFF_JOBS_LIST = [
    LANGUAGE_TEXTS.AST_EN_NAME,
    LANGUAGE_TEXTS.SCH_EN_NAME,
    LANGUAGE_TEXTS.DRG_EN_NAME,
    LANGUAGE_TEXTS.MNK_EN_NAME,
    LANGUAGE_TEXTS.NIN_EN_NAME,
    LANGUAGE_TEXTS.RPR_EN_NAME,
    LANGUAGE_TEXTS.BRD_EN_NAME,
    LANGUAGE_TEXTS.DNC_EN_NAME,
    LANGUAGE_TEXTS.SMN_EN_NAME,
    LANGUAGE_TEXTS.RDM_EN_NAME,
    LANGUAGE_TEXTS.PCT_EN_NAME,
  ];
  const HEALER_JOBS = [LANGUAGE_TEXTS.WHM_EN_NAME, LANGUAGE_TEXTS.AST_EN_NAME, LANGUAGE_TEXTS.SCH_EN_NAME, LANGUAGE_TEXTS.SGE_EN_NAME];
  const TANK_JOBS = [LANGUAGE_TEXTS.PLD_EN_NAME, LANGUAGE_TEXTS.WAR_EN_NAME, LANGUAGE_TEXTS.DRK_EN_NAME, LANGUAGE_TEXTS.GNB_EN_NAME];
  const DPS_JOBS = [
    LANGUAGE_TEXTS.MNK_EN_NAME,
    LANGUAGE_TEXTS.DRG_EN_NAME,
    LANGUAGE_TEXTS.NIN_EN_NAME,
    LANGUAGE_TEXTS.SAM_EN_NAME,
    LANGUAGE_TEXTS.RPR_EN_NAME,
    LANGUAGE_TEXTS.VPR_EN_NAME,
    LANGUAGE_TEXTS.BRD_EN_NAME,
    LANGUAGE_TEXTS.MCH_EN_NAME,
    LANGUAGE_TEXTS.DNC_EN_NAME,
    LANGUAGE_TEXTS.BLM_EN_NAME,
    LANGUAGE_TEXTS.SMN_EN_NAME,
    LANGUAGE_TEXTS.RDM_EN_NAME,
    LANGUAGE_TEXTS.PCT_EN_NAME,
  ];

  return { BUFF_JOBS_LIST, HEALER_JOBS, TANK_JOBS, DPS_JOBS };
}


export interface SimulationDataByRole {
  tanks: Array<BestPartnerSimulationData>;
  healers: Array<BestPartnerSimulationData>;
  melee: Array<BestPartnerSimulationData>;
  ranged: Array<BestPartnerSimulationData>;
  casters: Array<BestPartnerSimulationData>;
}

export interface BestPartnerSimulationData {
  jobAbbrev: string;
  buffContribution: number | null;
}

export class PartyCompositionMaker {
  private tank1: BestPartnerSimulationData | null = null;
  private tank2: BestPartnerSimulationData | null = null;
  private mainHealer: BestPartnerSimulationData | null = null;
  private subHealer: BestPartnerSimulationData | null = null;
  private melee: BestPartnerSimulationData | null = null;
  private ranged: BestPartnerSimulationData | null = null;
  private caster: BestPartnerSimulationData | null = null;
  private additionalDps: BestPartnerSimulationData | null = null;
  private simulationDataByRole: SimulationDataByRole;

  constructor(
    mainPlayerJobAbbrev: string,
    simulationDataByRole: SimulationDataByRole
  ) {
    this.simulationDataByRole = simulationDataByRole;
    this.setMainCharacter(mainPlayerJobAbbrev);
  }

  setMainCharacter(mainPlayerJobAbbrev: string) {
    let LANGUAGE_TEXTS = AppLanguageTexts();
    switch (mainPlayerJobAbbrev) {
      case LANGUAGE_TEXTS.PLD_EN_NAME:
      case LANGUAGE_TEXTS.WAR_EN_NAME:
      case LANGUAGE_TEXTS.DRK_EN_NAME:
      case LANGUAGE_TEXTS.GNB_EN_NAME:
        this.tank1 = {
          jobAbbrev: mainPlayerJobAbbrev,
          buffContribution: null,
        };
        break;

      case LANGUAGE_TEXTS.WHM_EN_NAME:
      case LANGUAGE_TEXTS.AST_EN_NAME:
      case LANGUAGE_TEXTS.SCH_EN_NAME:
      case LANGUAGE_TEXTS.SGE_EN_NAME:
        this.mainHealer = {
          jobAbbrev: mainPlayerJobAbbrev,
          buffContribution: null,
        };
        break;

      case LANGUAGE_TEXTS.MNK_EN_NAME:
      case LANGUAGE_TEXTS.DRG_EN_NAME:
      case LANGUAGE_TEXTS.NIN_EN_NAME:
      case LANGUAGE_TEXTS.SAM_EN_NAME:
      case LANGUAGE_TEXTS.RPR_EN_NAME:
      case LANGUAGE_TEXTS.VPR_EN_NAME:
        this.melee = {
          jobAbbrev: mainPlayerJobAbbrev,
          buffContribution: null,
        };
        break;

      case LANGUAGE_TEXTS.BRD_EN_NAME:
      case LANGUAGE_TEXTS.MCH_EN_NAME:
      case LANGUAGE_TEXTS.DNC_EN_NAME:
        this.ranged = {
          jobAbbrev: mainPlayerJobAbbrev,
          buffContribution: null,
        };
        break;

      case LANGUAGE_TEXTS.BLM_EN_NAME:
      case LANGUAGE_TEXTS.SMN_EN_NAME:
      case LANGUAGE_TEXTS.RDM_EN_NAME:
      case LANGUAGE_TEXTS.PCT_EN_NAME:
        this.caster = {
          jobAbbrev: mainPlayerJobAbbrev,
          buffContribution: null,
        };
        break;
    }
  }

  public makePartyComposition() {
    this.chooseTanks();
    this.chooseHealer();
    this.chooseDps();

    return {
      tank1: this.tank1,
      tank2: this.tank2,
      mainHealer: this.mainHealer,
      subHealer: this.subHealer,
      melee: this.melee,
      ranged: this.ranged,
      caster: this.caster,
      additionalDps: this.additionalDps,
    };
  }

  chooseTanks() {
    let tanksData = this.simulationDataByRole.tanks;

    tanksData.sort((a, b) => {
      let contributionA = a.buffContribution || 0;
      let contributionB = b.buffContribution || 0;
      return contributionB - contributionA;
    });

    let bestTank = tanksData.pop();

    if (this.tank1 === null && bestTank !== undefined) {
      this.tank1 = bestTank;
    }

    if (this.tank2 === null && bestTank !== undefined) {
      this.tank2 = bestTank;
    }
  }

  chooseHealer() {
    let healersData = this.simulationDataByRole.healers;

    healersData.sort((a, b) => {
      let contributionA = a.buffContribution || 0;
      let contributionB = b.buffContribution || 0;
      return contributionB - contributionA;
    });

    if (this.mainHealer === null) {
      let bestTwoHealers = [healersData[0], healersData[1]];
      bestTwoHealers.sort((a, b) => {
        return getHealerOrder(a.jobAbbrev) - getHealerOrder(b.jobAbbrev);
      });

      this.mainHealer = bestTwoHealers[0];
      this.subHealer = bestTwoHealers[1];
    } else {
      let bestHealer = healersData[0];
      this.subHealer = bestHealer;
    }
  }

  chooseDps() {
    let meleeData = this.simulationDataByRole.melee;
    let rangedData = this.simulationDataByRole.ranged;
    let casterData = this.simulationDataByRole.casters;

    meleeData.sort((a, b) => {
      let contributionA = a.buffContribution || 0;
      let contributionB = b.buffContribution || 0;
      return contributionB - contributionA;
    });
    rangedData.sort((a, b) => {
      let contributionA = a.buffContribution || 0;
      let contributionB = b.buffContribution || 0;
      return contributionB - contributionA;
    });
    casterData.sort((a, b) => {
      let contributionA = a.buffContribution || 0;
      let contributionB = b.buffContribution || 0;
      return contributionB - contributionA;
    });

    let bestMeleeIndex = 0;
    let bestRangedIndex = 0;
    let bestCasterIndex = 0;

    let bestMelee = meleeData[bestMeleeIndex];
    let bestRanged = rangedData[bestRangedIndex];
    let bestCaster = casterData[bestCasterIndex];

    if (this.melee === null && bestMelee !== undefined) {
      this.melee = bestMelee;
      bestMeleeIndex++;
    }

    if (this.ranged === null && bestRanged !== undefined) {
      this.ranged = bestRanged;
      bestRangedIndex++;
    }

    if (this.caster === null && bestCaster !== undefined) {
      this.caster = bestCaster;
      bestCasterIndex++;
    }

    let nextBestMelee = meleeData[bestMeleeIndex];
    let nextBestRanged = rangedData[bestRangedIndex];
    let nextBestCaster = casterData[bestCasterIndex];

    let nextBestMeleeContribution = nextBestMelee.buffContribution || 0;
    let nextBestRangedContribution = nextBestRanged.buffContribution || 0;
    let nextBestCasterContribution = nextBestCaster.buffContribution || 0;

    let bestDps = Math.max(
      nextBestMeleeContribution,
      nextBestRangedContribution,
      nextBestCasterContribution
    );

    if (bestDps === nextBestMeleeContribution) {
      this.additionalDps = nextBestMelee;
    } else if (bestDps === nextBestRangedContribution) {
      this.additionalDps = nextBestRanged;
    } else {
      this.additionalDps = nextBestCaster;
    }
  }
}

function getHealerOrder(healerJobAbbrev: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (healerJobAbbrev) {
    case LANGUAGE_TEXTS.WHM_EN_NAME:
      return 0;
    case LANGUAGE_TEXTS.AST_EN_NAME:
      return 1;
    case LANGUAGE_TEXTS.SCH_EN_NAME:
      return 2;
    default:
      return 3;
  }
}
