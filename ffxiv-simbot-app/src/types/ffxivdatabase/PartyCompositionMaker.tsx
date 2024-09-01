export interface SimulationDataByRole {
  tanks: Array<SimulationData>;
  healers: Array<SimulationData>;
  melee: Array<SimulationData>;
  ranged: Array<SimulationData>;
  casters: Array<SimulationData>;
}

export interface SimulationData {
  jobAbbrev: string;
  buffContribution: number | null;
}

export class PartyCompositionMaker {
  private tank1: SimulationData | null = null;
  private tank2: SimulationData | null = null;
  private mainHealer: SimulationData | null = null;
  private subHealer: SimulationData | null = null;
  private melee: SimulationData | null = null;
  private ranged: SimulationData | null = null;
  private caster: SimulationData | null = null;
  private additionalDps: SimulationData | null = null;
  private simulationDataByRole: SimulationDataByRole;

  constructor(
    mainPlayerJobAbbrev: string,
    simulationDataByRole: SimulationDataByRole
  ) {
    this.simulationDataByRole = simulationDataByRole;
    this.setMainCharacter(mainPlayerJobAbbrev);
  }

  setMainCharacter(mainPlayerJobAbbrev: string) {
    switch (mainPlayerJobAbbrev) {
      case "PLD":
      case "WAR":
      case "DRK":
      case "GNB":
        this.tank1 = { jobAbbrev: mainPlayerJobAbbrev, buffContribution: null };
        break;

      case "WHM":
      case "AST":
      case "SCH":
      case "SGE":
        this.mainHealer = {
          jobAbbrev: mainPlayerJobAbbrev,
          buffContribution: null,
        };
        break;

      case "MNK":
      case "DRG":
      case "NIN":
      case "SAM":
      case "RPR":
      case "VPR":
        this.melee = { jobAbbrev: mainPlayerJobAbbrev, buffContribution: null };
        break;

      case "BRD":
      case "MCH":
      case "DNC":
        this.ranged = {
          jobAbbrev: mainPlayerJobAbbrev,
          buffContribution: null,
        };
        break;

      case "BLM":
      case "SMN":
      case "RDM":
      case "PCT":
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
      bestTwoHealers.sort((a, b) => { return (getHealerOrder(a.jobAbbrev) - getHealerOrder(b.jobAbbrev)) });

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
  switch (healerJobAbbrev) {
    case "WHM":
      return 0;
    case "AST":
      return 1;
    case "SCH":
      return 2;
    default:
      return 3;
  }
}