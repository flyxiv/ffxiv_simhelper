import { AppConfigurations, ENGLISH_MODE } from "../Themes";

// Job Names
export const PLD_EN_NAME = "PLD";
export const PLD_KR_NAME = "나이트";

export const WAR_EN_NAME = "WAR";
export const WAR_KR_NAME = "전사";

export const DRK_EN_NAME = "DRK";
export const DRK_KR_NAME = "암흑기사";

export const GNB_EN_NAME = "GNB";
export const GNB_KR_NAME = "건브레이커";

export const WHM_EN_NAME = "WHM";
export const WHM_KR_NAME = "백마도사";

export const AST_EN_NAME = "AST";
export const AST_KR_NAME = "점성술사";

export const SCH_EN_NAME = "SCH";
export const SCH_KR_NAME = "학자";

export const SGE_EN_NAME = "SGE";
export const SGE_KR_NAME = "현자";

export const DRG_EN_NAME = "DRG";
export const DRG_KR_NAME = "용기사";

export const MNK_EN_NAME = "MNK";
export const MNK_KR_NAME = "몽크";

export const NIN_EN_NAME = "NIN";
export const NIN_KR_NAME = "닌자";

export const SAM_EN_NAME = "SAM";
export const SAM_KR_NAME = "사무라이";

export const RPR_EN_NAME = "RPR";
export const RPR_KR_NAME = "리퍼";

export const VPR_EN_NAME = "VPR";
export const VPR_KR_NAME = "바이퍼";

export const BRD_EN_NAME = "BRD";
export const BRD_KR_NAME = "음유시인";

export const MCH_EN_NAME = "MCH";
export const MCH_KR_NAME = "기공사";

export const DNC_EN_NAME = "DNC";
export const DNC_KR_NAME = "무도가";

export const BLM_EN_NAME = "BLM";
export const BLM_KR_NAME = "흑마도사";

export const SMN_EN_NAME = "SMN";
export const SMN_KR_NAME = "소환사";

export const RDM_EN_NAME = "RDM";
export const RDM_KR_NAME = "적마도사";

export const PCT_EN_NAME = "PCT";
export const PCT_KR_NAME = "픽토맨서";

export const PLD_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? PLD_EN_NAME : PLD_KR_NAME;
export const WAR_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? WAR_EN_NAME : WAR_KR_NAME;
export const DRK_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? DRK_EN_NAME : DRK_KR_NAME;
export const GNB_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? GNB_EN_NAME : GNB_KR_NAME;

export const WHM_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? WHM_EN_NAME : WHM_KR_NAME;
export const SCH_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? SCH_EN_NAME : SCH_KR_NAME;
export const AST_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? AST_EN_NAME : AST_KR_NAME;
export const SGE_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? SGE_EN_NAME : SGE_KR_NAME;

export const DRG_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? DRG_EN_NAME : DRG_KR_NAME;
export const MNK_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? MNK_EN_NAME : MNK_KR_NAME;
export const NIN_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? NIN_EN_NAME : NIN_KR_NAME;
export const SAM_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? SAM_EN_NAME : SAM_KR_NAME;
export const RPR_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? RPR_EN_NAME : RPR_KR_NAME;
export const VPR_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? VPR_EN_NAME : VPR_KR_NAME;

export const BRD_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? BRD_EN_NAME : BRD_KR_NAME;
export const MCH_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? MCH_EN_NAME : MCH_KR_NAME;
export const DNC_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? DNC_EN_NAME : DNC_KR_NAME;

export const BLM_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? BLM_EN_NAME : BLM_KR_NAME;
export const SMN_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? SMN_EN_NAME : SMN_KR_NAME;
export const RDM_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? RDM_EN_NAME : RDM_KR_NAME;
export const PCT_JOB_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? PCT_EN_NAME : PCT_KR_NAME;

const convertToKoreanJobAbbrev = (jobAbbrev: string) => {
  switch (jobAbbrev) {
    case PLD_EN_NAME:
      return PLD_KR_NAME;
    case WAR_EN_NAME:
      return WAR_KR_NAME;
    case DRK_EN_NAME:
      return DRK_KR_NAME;
    case GNB_EN_NAME:
      return GNB_KR_NAME;
    case WHM_EN_NAME:
      return WHM_KR_NAME;
    case AST_EN_NAME:
      return AST_KR_NAME;
    case SCH_EN_NAME:
      return SCH_KR_NAME;
    case SGE_EN_NAME:
      return SGE_KR_NAME;
    case DRG_EN_NAME:
      return DRG_KR_NAME;
    case MNK_EN_NAME:
      return MNK_KR_NAME;
    case NIN_EN_NAME:
      return NIN_KR_NAME;
    case SAM_EN_NAME:
      return SAM_KR_NAME;
    case RPR_EN_NAME:
      return RPR_KR_NAME;
    case VPR_EN_NAME:
      return VPR_KR_NAME;
    case BRD_EN_NAME:
      return BRD_KR_NAME;
    case MCH_EN_NAME:
      return MCH_KR_NAME;
    case DNC_EN_NAME:
      return DNC_KR_NAME;
    case BLM_EN_NAME:
      return BLM_KR_NAME;
    case SMN_EN_NAME:
      return SMN_KR_NAME;
    case RDM_EN_NAME:
      return RDM_KR_NAME;
    case PCT_EN_NAME:
      return PCT_KR_NAME;
    default:
      return jobAbbrev;
  }
};

export function convertToJobText(jobAbbrev: string) {
  return AppConfigurations.languageMode === ENGLISH_MODE
    ? jobAbbrev
    : convertToKoreanJobAbbrev(jobAbbrev);
}

// Menu Text
export const NAVIGATE_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Navigate" : "빠른 이동";
export const SAVED_GEARSETS_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Saved Gearsets"
    : "저장된 장비셋";
export const LOADOUT_NAME_LABEL_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Name" : "이름";
export const LOADOUT_WRITE_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Write" : "저장";
export const LOADOUT_LOAD_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Load" : "로드";
export const DEFAULT_LOADOUT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Default Loadout"
    : "기본 장비셋";
export const LOAD_COMPLETE_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Load Complete"
    : "불러오기 완료";

export const LOADOUT_NAME_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Loadout Name" : "장비셋 이름";

export const OVERWRITE_CONFIRM_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Overwrite gearset?"
    : "장비셋을 덮어쓰시겠습니까?";

export const LOAD_CONFIRM_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Load gearset?"
    : "장비셋을 불러오시겠습니까?";

export const CONFIRM_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Yes" : "확인";

export const CANCEL_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "No" : "취소";


// Player Text
export const JOB_EN_TEXT = "Job";
export const JOB_KR_TEXT = "직업";

export const RACE_EN_TEXT = "Race";
export const RACE_KR_TEXT = "종족";

export const WEAPON_SLOT_EN_TEXT = "weapon";
export const WEAPON_SLOT_KR_TEXT = "무기";

export const OFFHAND_SLOT_EN_TEXT = "offHand";
export const OFFHAND_SLOT_KR_TEXT = "보조무기";

export const HEAD_SLOT_EN_TEXT = "head";
export const HEAD_SLOT_KR_TEXT = "머리";

export const BODY_SLOT_EN_TEXT = "body";
export const BODY_SLOT_KR_TEXT = "몸통";

export const HANDS_SLOT_EN_TEXT = "hands";
export const HANDS_SLOT_KR_TEXT = "손";

export const LEGS_SLOT_EN_TEXT = "legs";
export const LEGS_SLOT_KR_TEXT = "다리";

export const FEET_SLOT_EN_TEXT = "feet";
export const FEET_SLOT_KR_TEXT = "발";

export const EARS_SLOT_EN_TEXT = "ears";
export const EARS_SLOT_KR_TEXT = "귀걸이";

export const NECK_SLOT_EN_TEXT = "neck";
export const NECK_SLOT_KR_TEXT = "목걸이";

export const WRIST_SLOT_EN_TEXT = "wrists";
export const WRIST_SLOT_KR_TEXT = "팔찌";

export const FINGER1_SLOT_EN_TEXT = "finger1";
export const FINGER1_SLOT_KR_TEXT = "반지1";

export const FINGER2_SLOT_EN_TEXT = "finger2";
export const FINGER2_SLOT_KR_TEXT = "반지2";

export const FOOD_SLOT_EN_TEXT = "food";
export const FOOD_SLOT_KR_TEXT = "음식";

export const AST_MELEE_PARTNER_EN_TEXT = "Melee Card Target";
export const AST_MELEE_PARTNER_KR_TEXT = "근거리 카드 대상";

export const AST_RANGED_PARTNER_EN_TEXT = "Ranged Card Target";
export const AST_RANGED_PARTNER_KR_TEXT = "원거리 카드 대상";

export const DNC_PARTNER_EN_TEXT = "Dance Partner";
export const DNC_PARTNER_KR_TEXT = "무도가 파트너";

export const FINGER_SLOT_EN_TEXT = "finger";
export const FINGER_SLOT_KR_TEXT = "반지";

export const JOB_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? JOB_EN_TEXT : JOB_KR_TEXT;
export const RACE_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? RACE_EN_TEXT : RACE_KR_TEXT;
export const WEAPON_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? WEAPON_SLOT_EN_TEXT
    : WEAPON_SLOT_KR_TEXT;
export const OFFHAND_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? OFFHAND_SLOT_EN_TEXT
    : OFFHAND_SLOT_KR_TEXT;
export const HEAD_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? HEAD_SLOT_EN_TEXT
    : HEAD_SLOT_KR_TEXT;
export const BODY_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? BODY_SLOT_EN_TEXT
    : BODY_SLOT_KR_TEXT;
export const HANDS_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? HANDS_SLOT_EN_TEXT
    : HANDS_SLOT_KR_TEXT;
export const LEGS_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? LEGS_SLOT_EN_TEXT
    : LEGS_SLOT_KR_TEXT;
export const FEET_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? FEET_SLOT_EN_TEXT
    : FEET_SLOT_KR_TEXT;
export const EARS_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? EARS_SLOT_EN_TEXT
    : EARS_SLOT_KR_TEXT;
export const NECK_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? NECK_SLOT_EN_TEXT
    : NECK_SLOT_KR_TEXT;
export const WRIST_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? WRIST_SLOT_EN_TEXT
    : WRIST_SLOT_KR_TEXT;
export const FINGER1_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? FINGER1_SLOT_EN_TEXT
    : FINGER1_SLOT_KR_TEXT;
export const FINGER2_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? FINGER2_SLOT_EN_TEXT
    : FINGER2_SLOT_KR_TEXT;
export const FOOD_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? FOOD_SLOT_EN_TEXT
    : FOOD_SLOT_KR_TEXT;
export const AST_MELEE_PARTNER_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? AST_MELEE_PARTNER_EN_TEXT
    : AST_MELEE_PARTNER_KR_TEXT;
export const AST_RANGED_PARTNER_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? AST_RANGED_PARTNER_EN_TEXT
    : AST_RANGED_PARTNER_KR_TEXT;
export const DNC_PARTNER_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? DNC_PARTNER_EN_TEXT
    : DNC_PARTNER_KR_TEXT;
export const FINGER_SLOT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? FINGER_SLOT_EN_TEXT
    : FINGER_SLOT_KR_TEXT;

function convertToKoreanSlotName(slotName: string) {
  switch (slotName) {
    case JOB_EN_TEXT:
      return JOB_KR_TEXT;
    case RACE_EN_TEXT:
      return RACE_KR_TEXT;
    case WEAPON_SLOT_EN_TEXT:
      return WEAPON_SLOT_KR_TEXT;
    case OFFHAND_SLOT_EN_TEXT:
      return OFFHAND_SLOT_KR_TEXT;
    case HEAD_SLOT_EN_TEXT:
      return HEAD_SLOT_KR_TEXT;
    case BODY_SLOT_EN_TEXT:
      return BODY_SLOT_KR_TEXT;
    case HANDS_SLOT_EN_TEXT:
      return HANDS_SLOT_KR_TEXT;
    case LEGS_SLOT_EN_TEXT:
      return LEGS_SLOT_KR_TEXT;
    case FEET_SLOT_EN_TEXT:
      return FEET_SLOT_KR_TEXT;
    case EARS_SLOT_EN_TEXT:
      return EARS_SLOT_KR_TEXT;
    case NECK_SLOT_EN_TEXT:
      return NECK_SLOT_KR_TEXT;
    case WRIST_SLOT_EN_TEXT:
      return WRIST_SLOT_KR_TEXT;
    case FINGER1_SLOT_EN_TEXT:
      return FINGER1_SLOT_KR_TEXT;
    case FINGER2_SLOT_EN_TEXT:
      return FINGER2_SLOT_KR_TEXT;
    case FOOD_SLOT_EN_TEXT:
      return FOOD_SLOT_KR_TEXT;
    case AST_MELEE_PARTNER_EN_TEXT:
      return AST_MELEE_PARTNER_KR_TEXT;
    case AST_RANGED_PARTNER_EN_TEXT:
      return AST_RANGED_PARTNER_KR_TEXT;
    case DNC_PARTNER_EN_TEXT:
      return DNC_PARTNER_KR_TEXT;
    case FINGER_SLOT_EN_TEXT:
      return FINGER_SLOT_KR_TEXT;
    default:
      return slotName;
  }
}

export function convertToSlotText(slotName: string) {
  return AppConfigurations.languageMode === ENGLISH_MODE
    ? slotName
    : convertToKoreanSlotName(slotName);
}

// Race Names
export const MIDLANDER_HYUR_NAME_EN = "Midlander Hyur";
export const MIDLANDER_HYUR_NAME_KR = "중부 휴런";

export const HIGHLANDER_HYUR_NAME_EN = "Highlander Hyur";
export const HIGHLANDER_HYUR_NAME_KR = "고지 휴런";

export const WILDWOOD_ELEZEN_NAME_EN = "Wildwood Elezen";
export const WILDWOOD_ELEZEN_NAME_KR = "숲 엘레젠";

export const DUSKWIGHT_ELEZEN_NAME_EN = "Duskwight Elezen";
export const DUSKWIGHT_ELEZEN_NAME_KR = "밤 엘레젠";

export const PLAINSFOLK_LALAFELL_NAME_EN = "Plainsfolk Lalafell";
export const PLAINSFOLK_LALAFELL_NAME_KR = "평원 라라펠";

export const DUNESFOLK_LALAFELL_NAME_EN = "Dunesfolk Lalafell";
export const DUNESFOLK_LALAFELL_NAME_KR = "사막 라라펠";

export const SEEKER_OF_THE_SUN_MIQOTE_NAME_EN = "Seeker of the Sun Miqo'te";
export const SEEKER_OF_THE_SUN_MIQOTE_NAME_KR = "태양의 추종자 미코테";

export const KEEPER_OF_THE_MOON_MIQOTE_NAME_EN = "Keeper of the Moon Miqo'te";
export const KEEPER_OF_THE_MOON_MIQOTE_NAME_KR = "달의 수호자 미코테";

export const SEA_WOLVES_ROEGADYN_NAME_EN = "Sea Wolves Roegadyn";
export const SEA_WOLVES_ROEGADYN_NAME_KR = "바다늑대 루가딘";

export const HELLSGUARD_ROEGADYN_NAME_EN = "Hellsguard Roegadyn";
export const HELLSGUARD_ROEGADYN_NAME_KR = "불꽃지킴이 루가딘";

export const RAEN_AU_RA_NAME_EN = "Raen Au Ra";
export const RAEN_AU_RA_NAME_KR = "렌 아우라";

export const XAELA_AU_RA_NAME_EN = "Xaela Au Ra";
export const XAELA_AU_RA_NAME_KR = "젤라 아우라";

export const HELIONS_HROTHGAR_NAME_EN = "Helions Hrothgar";
export const HELIONS_HROTHGAR_NAME_KR = "맴도는 별 로스갈";

export const THE_LOST_HROTHGAR_NAME_EN = "The Lost Hrothgar";
export const THE_LOST_HROTHGAR_NAME_KR = "떠도는 별 로스갈";

export const RAVA_VIERA_NAME_EN = "Rava Viera";
export const RAVA_VIERA_NAME_KR = "라바 비에라";

export const VEENA_VIERA_NAME_EN = "Veena Viera";
export const VEENA_VIERA_NAME_KR = "비나 비에라";

export const MIDLANDER_HYUR_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? MIDLANDER_HYUR_NAME_EN
    : MIDLANDER_HYUR_NAME_KR;
export const HIGHLANDER_HYUR_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? HIGHLANDER_HYUR_NAME_EN
    : HIGHLANDER_HYUR_NAME_KR;
export const WILDWOOD_ELEZEN_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? WILDWOOD_ELEZEN_NAME_EN
    : WILDWOOD_ELEZEN_NAME_KR;
export const DUSKWIGHT_ELEZEN_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? DUSKWIGHT_ELEZEN_NAME_EN
    : DUSKWIGHT_ELEZEN_NAME_KR;
export const PLAINSFOLK_LALAFELL_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? PLAINSFOLK_LALAFELL_NAME_EN
    : PLAINSFOLK_LALAFELL_NAME_KR;
export const DUNESFOLK_LALAFELL_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? DUNESFOLK_LALAFELL_NAME_EN
    : DUNESFOLK_LALAFELL_NAME_KR;
export const SEEKER_OF_THE_SUN_MIQOTE_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? SEEKER_OF_THE_SUN_MIQOTE_NAME_EN
    : SEEKER_OF_THE_SUN_MIQOTE_NAME_KR;
export const KEEPER_OF_THE_MOON_MIQOTE_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? KEEPER_OF_THE_MOON_MIQOTE_NAME_EN
    : KEEPER_OF_THE_MOON_MIQOTE_NAME_KR;
export const SEA_WOLVES_ROEGADYN_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? SEA_WOLVES_ROEGADYN_NAME_EN
    : SEA_WOLVES_ROEGADYN_NAME_KR;
export const HELLSGUARD_ROEGADYN_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? HELLSGUARD_ROEGADYN_NAME_EN
    : HELLSGUARD_ROEGADYN_NAME_KR;
export const RAEN_AU_RA_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? RAEN_AU_RA_NAME_EN
    : RAEN_AU_RA_NAME_KR;
export const XAELA_AU_RA_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? XAELA_AU_RA_NAME_EN
    : XAELA_AU_RA_NAME_KR;
export const HELIONS_HROTHGAR_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? HELIONS_HROTHGAR_NAME_EN
    : HELIONS_HROTHGAR_NAME_KR;
export const THE_LOST_HROTHGAR_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? THE_LOST_HROTHGAR_NAME_EN
    : THE_LOST_HROTHGAR_NAME_KR;
export const RAVA_VIERA_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? RAVA_VIERA_NAME_EN
    : RAVA_VIERA_NAME_KR;
export const VEENA_VIERA_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? VEENA_VIERA_NAME_EN
    : VEENA_VIERA_NAME_KR;

export const RACES = [
  MIDLANDER_HYUR_NAME_EN,
  HIGHLANDER_HYUR_NAME_EN,
  WILDWOOD_ELEZEN_NAME_EN,
  DUSKWIGHT_ELEZEN_NAME_EN,
  PLAINSFOLK_LALAFELL_NAME_EN,
  DUNESFOLK_LALAFELL_NAME_EN,
  SEEKER_OF_THE_SUN_MIQOTE_NAME_EN,
  KEEPER_OF_THE_MOON_MIQOTE_NAME_EN,
  SEA_WOLVES_ROEGADYN_NAME_EN,
  HELLSGUARD_ROEGADYN_NAME_EN,
  RAEN_AU_RA_NAME_EN,
  XAELA_AU_RA_NAME_EN,
  HELIONS_HROTHGAR_NAME_EN,
  THE_LOST_HROTHGAR_NAME_EN,
  RAVA_VIERA_NAME_EN,
  VEENA_VIERA_NAME_EN,
];

function convertToKoreanRaceName(raceName: string) {
  switch (raceName) {
    case MIDLANDER_HYUR_NAME_EN:
      return MIDLANDER_HYUR_NAME_KR;
    case HIGHLANDER_HYUR_NAME_EN:
      return HIGHLANDER_HYUR_NAME_KR;
    case WILDWOOD_ELEZEN_NAME_EN:
      return WILDWOOD_ELEZEN_NAME_KR;
    case DUSKWIGHT_ELEZEN_NAME_EN:
      return DUSKWIGHT_ELEZEN_NAME_KR;
    case PLAINSFOLK_LALAFELL_NAME_EN:
      return PLAINSFOLK_LALAFELL_NAME_KR;
    case DUNESFOLK_LALAFELL_NAME_EN:
      return DUNESFOLK_LALAFELL_NAME_KR;
    case SEEKER_OF_THE_SUN_MIQOTE_NAME_EN:
      return SEEKER_OF_THE_SUN_MIQOTE_NAME_KR;
    case KEEPER_OF_THE_MOON_MIQOTE_NAME_EN:
      return KEEPER_OF_THE_MOON_MIQOTE_NAME_KR;
    case SEA_WOLVES_ROEGADYN_NAME_EN:
      return SEA_WOLVES_ROEGADYN_NAME_KR;
    case HELLSGUARD_ROEGADYN_NAME_EN:
      return HELLSGUARD_ROEGADYN_NAME_KR;
    case RAEN_AU_RA_NAME_EN:
      return RAEN_AU_RA_NAME_KR;
    case XAELA_AU_RA_NAME_EN:
      return XAELA_AU_RA_NAME_KR;
    case HELIONS_HROTHGAR_NAME_EN:
      return HELIONS_HROTHGAR_NAME_KR;
    case THE_LOST_HROTHGAR_NAME_EN:
      return THE_LOST_HROTHGAR_NAME_KR;
    case RAVA_VIERA_NAME_EN:
      return RAVA_VIERA_NAME_KR;
    case VEENA_VIERA_NAME_EN:
      return VEENA_VIERA_NAME_KR;
    default:
      return raceName;
  }
}

export function convertToRaceText(raceName: string) {
  return AppConfigurations.languageMode === ENGLISH_MODE
    ? raceName
    : convertToKoreanRaceName(raceName);
}

// Page Names
export const HOME_PAGE_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Home" : "시작";
export const QUICKSIM_PAGE_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Quick Sim"
    : "빠른 시뮬레이션";
export const GEAR_COMPARE_PAGE_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Gear Compare"
    : "장비셋 비교";
export const BEST_PARTNER_PAGE_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Best Partner"
    : "시너지 파트너";
export const STAT_WEIGHTS_PAGE_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Stat Weights"
    : "스탯 가중치";

export const PLAYER_POWER_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Specific Player Power"
    : "스탯별 결과 수치";
export const QUICK_SIM_PARTY_INPUT_INFO_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "2. Additional Settings"
    : "2. 파티 관련 설정을 입력해주세요";
export const PARTY_MEMBER_ILVL_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Party Members iLvl"
    : "파티원 템렙";
export const POT_LABEL_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Pot" : "탕약";
export const USE_POT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "O(Use)" : "O(사용)";
export const NO_POT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "X(No)" : "X(미사용)";

// Home
export const QUICK_SIM_DESCRIPTION_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Quickly get analyze DPS of your input gearset."
    : "당신의 장비셋을 입력하고 빠르게 분석해보세요.";
export const GEAR_COMPARE_DESCRIPTION_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Compare two gearsets to see which is better for your damage."
    : "두 장비셋을 비교하여 더 높은 데미지를 내는 장비셋을 확인해보세요.";
export const STAT_WEIGHTS_DESCRIPTION_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Calculate which main/sub stats are more valuable to you."
    : "가장 딜상승 기대값이 높은 스탯을 분석합니다.";
export const BEST_PARTNER_DESCRIPTION_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Find out the teammates that will contribute the most RDPS for you(buff jobs only)."
    : "내 시너지를 가장 잘 사용해줄 조합을 찾아줍니다(시너지 직업만).";

// Quick Sim
export const QUICK_SIM_INPUT_INFO_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "1. Input Your Info"
    : "1. 분석할 장비셋을 입력해주세요";

// Gear Compare
export const GEAR_COMPARE_INPUT_INFO_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "1. Input Gearsets You Want To Compare"
    : "1. 비교할 두 장비셋을 입력해주세요";
export const COPY_BUTTON_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Copy" : "복사";

// Best Partner
export const BEST_PARTNER_INPUT_INFO_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "1. Input Your Info"
    : "1. 필요한 정보를 입력해주세요";

export const SPEED_LABEL_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "GCD" : "글쿨";

// Results Text
export const SIMULATION_RESULT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Simulation Result"
    : "시뮬레이션 결과";
export const PARTY_MEMBERS_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Party Members" : "파티원";
export const TIME_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Time(s)" : "전투시간(초)";

// Quick Sim Results
export const DAMAGE_PROFILE_BUTTON_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Damage Profile"
    : "데미지 분포";
export const SKILL_TITLE_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Skill" : "스킬";
export const DAMAGE_PERCENTAGE_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Dmg%" : "데미지%";
export const TOTAL_DAMAGE_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Total Dmg" : "총 데미지";
export const CAST_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Cast" : "스윙 수";

export const BEST_TEAMMATE_BUTTON_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Buffs Received"
    : "받은 기여도(RDPS)";
export const MEMBER_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Member" : "파티원";
export const TOTAL_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Total" : "합계";

export const MY_CONTRIBUTION_BUTTON_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Buffs Taken"
    : "준 기여도(RDPS)";

export const ROTATION_SAMPLE_WARNING_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "!!!This is a sample of the engine's simulation log, not a guide to playing the job. analyze with care."
    : "!!!이건 직업에 대한 가이드가 아닌, 시뮬레이션 엔진의 샘플 결과일 뿐입니다. 이를 감안하고 분석할 때 주의해주세요.";

export const SAMURAI_ROTATION_WARNING_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "!!!Samurai's 'tengentsu' is simulated as a 35s skill(looking at logsmatch its 12 casts in a 6:30 fight)."
    : "!!!사무라이의 '심안'은 35초 스킬로 시뮬레이션 되었습니다(로그를 봤을 때 6:30 전투에서 12번 정도 사용됨).";

export const ROTATION_SAMPLE_BUTTON_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Rotation Sample"
    : "딜사이클 샘플";
export const COMBAT_TIME_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Combat Time" : "전투 시간";
export const ABILITY_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Ability" : "스킬";
export const IMPORTANT_STATUS_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Important Status"
    : "중요 버프들";

// Gear Compare Results
export const GEARSET1_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Gearset1" : "장비셋1";
export const GEARSET2_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Gearset2" : "장비셋2";

// Best Partner Results
export const BEST_PARTNER_BY_ROLE_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Best Partner By Role"
    : "역할군별 최고 파트너";
export const TANK_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Tank" : "탱커";
export const HEALER_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Healer" : "힐러";
export const DPS_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "DPS" : "딜러";

export const OVERALL_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Total Contribution"
    : "총합 기여도";
export const BURST_SECTION_TITLE_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Contributions at Each Burst"
    : "버스트마다의 기여도";
export const BURST_TEXT = (burst_minute: number) =>
  AppConfigurations.languageMode === ENGLISH_MODE
    ? `${burst_minute} Minute Burst`
    : `${burst_minute}분 버스트`;

// Stat Weights Results
export const STAT_WEIGHTS_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "RDPS Increase Per Stat Point"
    : "스탯 1당 RDPS 증가 기대값";

export const STAT_WEIGHTS_NAME_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "RDPS Increase / 1 Stat Point"
    : "RDPS 증가 / 1 스탯 포인트";

export const EMPTY_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Empty" : "없음";

// Stat Names
export const NAME_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Name" : "이름";
export const STAT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Stat" : "스탯";
export const PREV_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Prev" : "이전";
export const NEXT_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Next" : "다음";

export const WD_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "WD" : "무공";

export const STR_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "STR" : "힘";
export const DEX_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "DEX" : "민첩";
export const INT_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "INT" : "지능";
export const MIND_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "MND" : "정신";

export const CRIT_STAT_EN_NAME = "CRT";
export const DH_STAT_EN_NAME = "DH";
export const DET_STAT_EN_NAME = "DET";
export const SKS_STAT_EN_NAME = "SKS";
export const SPS_STAT_EN_NAME = "SPS";
export const TEN_STAT_EN_NAME = "TEN";
export const PIE_STAT_EN_NAME = "PIE";

export const CRIT_STAT_KR_NAME = "극대";
export const DH_STAT_KR_NAME = "직격";
export const DET_STAT_KR_NAME = "의지";
export const SKS_STAT_KR_NAME = "기시";
export const SPS_STAT_KR_NAME = "마시";
export const TEN_STAT_KR_NAME = "불굴";
export const PIE_STAT_KR_NAME = "신앙";

export const CRIT_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? CRIT_STAT_EN_NAME
    : CRIT_STAT_KR_NAME;
export const DH_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? DH_STAT_EN_NAME
    : DH_STAT_KR_NAME;
export const DET_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? DET_STAT_EN_NAME
    : DET_STAT_KR_NAME;
export const SKS_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? SKS_STAT_EN_NAME
    : SKS_STAT_KR_NAME;
export const SPS_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? SPS_STAT_EN_NAME
    : SPS_STAT_KR_NAME;
export const TEN_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? TEN_STAT_EN_NAME
    : TEN_STAT_KR_NAME;
export const PIE_STAT_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? PIE_STAT_EN_NAME
    : PIE_STAT_KR_NAME;

export function convertToEnglishSubStatName(subStatName: string) {
  switch (subStatName) {
    case CRIT_STAT_KR_NAME:
      return CRIT_STAT_EN_NAME;
    case DH_STAT_KR_NAME:
      return DH_STAT_EN_NAME;
    case DET_STAT_KR_NAME:
      return DET_STAT_EN_NAME;
    case SKS_STAT_KR_NAME:
      return SKS_STAT_EN_NAME;
    case SPS_STAT_KR_NAME:
      return SPS_STAT_EN_NAME;
    case TEN_STAT_KR_NAME:
      return TEN_STAT_EN_NAME;
    case PIE_STAT_KR_NAME:
      return PIE_STAT_EN_NAME;
    default:
      return subStatName;
  }
}

// Power Names
export const ITERATION_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Iteration" : "시뮬 횟수";
export const VARIANCE_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Error %" : "오차 범위";
export const COMPOSITION_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "PT%" : "조합%";
export const WD_POWER_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Weapon" : "무기 공격력";
export const MAIN_STAT_POWER_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Main Stat" : "주 스탯";
export const CRT_RATE_POWER_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Crit Rate" : "극대 확률";
export const CRT_POWER_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Crit" : "극대 피해";
export const DH_RATE_POWER_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "DH Rate" : "직격 확률";
export const DET_POWER_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Det Rate" : "의지 효과";
export const SPEED_POWER_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Speed" : "도트/평타 위력";
export const TENACITY_POWER_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Tenacity" : "불굴 효과";
export const GCD_NAME =
  AppConfigurations.languageMode === ENGLISH_MODE ? "GCD" : "글쿨";

// Party Input Text
export const TIME_INPUT_LABEL_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE
    ? "Combat Time(Seconds)"
    : "전투 시간(초)";
export const PARTY_MEMBER_LABEL_TEXT =
  AppConfigurations.languageMode === ENGLISH_MODE ? "Party Member" : "파티원";
