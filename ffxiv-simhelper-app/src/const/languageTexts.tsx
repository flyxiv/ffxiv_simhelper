import { useEffect, useState } from "react";
import { LanguageMode, useLanguage } from "../LanguageContext";

export const WD_STAT_EN_NAME = "WD";
export const STR_STAT_EN_NAME = "STR";
export const DEX_STAT_EN_NAME = "DEX";
export const INT_STAT_EN_NAME = "INT";
export const MIND_STAT_EN_NAME = "MND";
export const CRIT_STAT_EN_NAME = "CRT";
export const DH_STAT_EN_NAME = "DH";
export const DET_STAT_EN_NAME = "DET";
export const SKS_STAT_EN_NAME = "SKS";
export const SPS_STAT_EN_NAME = "SPS";
export const TEN_STAT_EN_NAME = "TEN";
export const PIE_STAT_EN_NAME = "PIE";

export const PLD_EN_NAME = "PLD";
export const WAR_EN_NAME = "WAR";
export const DRK_EN_NAME = "DRK";
export const GNB_EN_NAME = "GNB";

export const WHM_EN_NAME = "WHM";
export const AST_EN_NAME = "AST";

export const SCH_EN_NAME = "SCH";
export const SGE_EN_NAME = "SGE";
export const DRG_EN_NAME = "DRG";
export const MNK_EN_NAME = "MNK";
export const NIN_EN_NAME = "NIN";
export const SAM_EN_NAME = "SAM";
export const RPR_EN_NAME = "RPR";
export const VPR_EN_NAME = "VPR";

export const BRD_EN_NAME = "BRD";
export const MCH_EN_NAME = "MCH";
export const DNC_EN_NAME = "DNC";
export const BLM_EN_NAME = "BLM";
export const SMN_EN_NAME = "SMN";
export const RDM_EN_NAME = "RDM";
export const PCT_EN_NAME = "PCT";



export const MIDLANDER_HYUR_EN_NAME = "Midlander Hyur";

export const HIGHLANDER_HYUR_EN_NAME = "Highlander Hyur";

export const WILDWOOD_ELEZEN_EN_NAME = "Wildwood Elezen";

export const DUSKWIGHT_ELEZEN_EN_NAME = "Duskwight Elezen";

export const PLAINSFOLK_LALAFELL_EN_NAME = "Plainsfolk Lalafell";

export const DUNESFOLK_LALAFELL_EN_NAME = "Dunesfolk Lalafell";

export const SEEKER_OF_THE_SUN_MIQOTE_EN_NAME = "Seeker of the Sun Miqo'te";

export const KEEPER_OF_THE_MOON_MIQOTE_EN_NAME = "Keeper of the Moon Miqo'te";

export const SEA_WOLVES_ROEGADYN_EN_NAME = "Sea Wolves Roegadyn";

export const HELLSGUARD_ROEGADYN_EN_NAME = "Hellsguard Roegadyn";

export const RAEN_AU_RA_EN_NAME = "Raen Au Ra";

export const XAELA_AU_RA_EN_NAME = "Xaela Au Ra";

export const HELIONS_HROTHGAR_EN_NAME = "Helions Hrothgar";

export const THE_LOST_HROTHGAR_EN_NAME = "The Lost Hrothgar";

export const RAVA_VIERA_EN_NAME = "Rava Viera";

export const VEENA_VIERA_EN_NAME = "Veena Viera";

// Player Text
export const WEAPON_SLOT_EN_TEXT = "weapon";

export const OFFHAND_SLOT_EN_TEXT = "offHand";

export const HEAD_SLOT_EN_TEXT = "head";

export const BODY_SLOT_EN_TEXT = "body";

export const HANDS_SLOT_EN_TEXT = "hands";

export const LEGS_SLOT_EN_TEXT = "legs";

export const FEET_SLOT_EN_TEXT = "feet";

export const EARS_SLOT_EN_TEXT = "ears";

export const NECK_SLOT_EN_TEXT = "neck";

export const WRIST_SLOT_EN_TEXT = "wrists";

export const FINGER1_SLOT_EN_TEXT = "finger1";

export const FINGER2_SLOT_EN_TEXT = "finger2";

export const FOOD_SLOT_EN_TEXT = "food";

export const FINGER_SLOT_EN_TEXT = "finger";

export const RACES = [
  MIDLANDER_HYUR_EN_NAME,
  HIGHLANDER_HYUR_EN_NAME,
  WILDWOOD_ELEZEN_EN_NAME,
  DUSKWIGHT_ELEZEN_EN_NAME,
  PLAINSFOLK_LALAFELL_EN_NAME,
  DUNESFOLK_LALAFELL_EN_NAME,
  SEEKER_OF_THE_SUN_MIQOTE_EN_NAME,
  KEEPER_OF_THE_MOON_MIQOTE_EN_NAME,
  SEA_WOLVES_ROEGADYN_EN_NAME,
  HELLSGUARD_ROEGADYN_EN_NAME,
  RAEN_AU_RA_EN_NAME,
  XAELA_AU_RA_EN_NAME,
  HELIONS_HROTHGAR_EN_NAME,
  THE_LOST_HROTHGAR_EN_NAME,
  RAVA_VIERA_EN_NAME,
  VEENA_VIERA_EN_NAME,
];

export const PLD_KR_NAME = "나이트";
export const WAR_KR_NAME = "전사";
export const DRK_KR_NAME = "암흑기사";
export const GNB_KR_NAME = "건브레이커";
export const WHM_KR_NAME = "백마도사";
export const AST_KR_NAME = "점성술사";
export const SCH_KR_NAME = "학자";
export const SGE_KR_NAME = "현자";
export const DRG_KR_NAME = "용기사";
export const MNK_KR_NAME = "몽크";
export const NIN_KR_NAME = "닌자";
export const SAM_KR_NAME = "사무라이";
export const RPR_KR_NAME = "리퍼";
export const VPR_KR_NAME = "바이퍼";
export const BRD_KR_NAME = "음유시인";
export const MCH_KR_NAME = "기공사";

export const DNC_KR_NAME = "무도가";
export const BLM_KR_NAME = "흑마도사";
export const SMN_KR_NAME = "소환사";
export const RDM_KR_NAME = "적마도사";
export const PCT_KR_NAME = "픽토맨서";

// Player Text
export const JOB_EN_TEXT = "Job";
const JOB_KR_TEXT = "직업";

const RACE_EN_TEXT = "Race";
const RACE_KR_TEXT = "종족";

export const WEAPON_SLOT_KR_TEXT = "무기";
export const OFFHAND_SLOT_KR_TEXT = "보조무기";
export const HEAD_SLOT_KR_TEXT = "머리";

const BODY_SLOT_KR_TEXT = "몸통";
const HANDS_SLOT_KR_TEXT = "손";

const LEGS_SLOT_KR_TEXT = "다리";

const FEET_SLOT_KR_TEXT = "발";

const EARS_SLOT_KR_TEXT = "귀걸이";

const NECK_SLOT_KR_TEXT = "목걸이";

const WRIST_SLOT_KR_TEXT = "팔찌";

const FINGER1_SLOT_KR_TEXT = "반지1";

const FINGER2_SLOT_KR_TEXT = "반지2";

const FOOD_SLOT_KR_TEXT = "음식";

const AST_MELEE_PARTNER_EN_TEXT = "Melee Card Target";
const AST_MELEE_PARTNER_KR_TEXT = "근거리 카드 대상";

const AST_RANGED_PARTNER_EN_TEXT = "Ranged Card Target";
const AST_RANGED_PARTNER_KR_TEXT = "원거리 카드 대상";

const DNC_PARTNER_EN_TEXT = "Dance Partner";
const DNC_PARTNER_KR_TEXT = "무도가 파트너";

const FINGER_SLOT_KR_TEXT = "반지";


// Race Names
const MIDLANDER_HYUR_KR_NAME = "중부 휴런";

const HIGHLANDER_HYUR_KR_NAME = "고지 휴런";

const WILDWOOD_ELEZEN_KR_NAME = "숲 엘레젠";

const DUSKWIGHT_ELEZEN_KR_NAME = "밤 엘레젠";

const PLAINSFOLK_LALAFELL_KR_NAME = "평원 라라펠";

const DUNESFOLK_LALAFELL_KR_NAME = "사막 라라펠";

const SEEKER_OF_THE_SUN_MIQOTE_KR_NAME = "태양의 추종자 미코테";

const KEEPER_OF_THE_MOON_MIQOTE_KR_NAME = "달의 수호자 미코테";

const SEA_WOLVES_ROEGADYN_KR_NAME = "바다늑대 루가딘";

const HELLSGUARD_ROEGADYN_KR_NAME = "불꽃지킴이 루가딘";

const RAEN_AU_RA_KR_NAME = "렌 아우라";

const XAELA_AU_RA_KR_NAME = "젤라 아우라";

const HELIONS_HROTHGAR_KR_NAME = "맴도는 별 로스갈";

const THE_LOST_HROTHGAR_KR_NAME = "떠도는 별 로스갈";

const RAVA_VIERA_KR_NAME = "라바 비에라";

const VEENA_VIERA_KR_NAME = "비나 비에라";

const WD_STAT_KR_NAME = "무공";
const STR_STAT_KR_NAME = "힘";
const DEX_STAT_KR_NAME = "민첩";
const INT_STAT_KR_NAME = "지능";
const MIND_STAT_KR_NAME = "정신";


const CRIT_STAT_KR_NAME = "극대";
const DH_STAT_KR_NAME = "직격";
const DET_STAT_KR_NAME = "의지";
const SKS_STAT_KR_NAME = "기시";
const SPS_STAT_KR_NAME = "마시";
const TEN_STAT_KR_NAME = "불굴";
const PIE_STAT_KR_NAME = "신앙";



export interface TextDictionary {
  PLD_JOB_NAME: string,
  WAR_JOB_NAME: string,
  DRK_JOB_NAME: string,
  GNB_JOB_NAME: string,
  WHM_JOB_NAME: string,
  SCH_JOB_NAME: string,
  AST_JOB_NAME: string,

  SGE_JOB_NAME: string,

  DRG_JOB_NAME: string,
  MNK_JOB_NAME: string,
  NIN_JOB_NAME: string,
  SAM_JOB_NAME: string,
  RPR_JOB_NAME: string,
  VPR_JOB_NAME: string,

  BRD_JOB_NAME: string,
  MCH_JOB_NAME: string,
  DNC_JOB_NAME: string,

  BLM_JOB_NAME: string,
  SMN_JOB_NAME: string,
  RDM_JOB_NAME: string,
  PCT_JOB_NAME: string,


  // Menu Text
  NAVIGATE_TEXT: string,
  SAVED_GEARSETS_TEXT: string,
  LOADOUT_NAME_LABEL_TEXT: string,
  LOADOUT_WRITE_TEXT: string,
  LOADOUT_LOAD_TEXT: string,
  DEFAULT_LOADOUT_TEXT: string,
  LOAD_COMPLETE_TEXT: string,
  LOADOUT_NAME_TEXT: string,

  OVERWRITE_CONFIRM_TEXT: string,

  LOAD_CONFIRM_TEXT: string,
  CONFIRM_TEXT: string,

  CANCEL_TEXT: string,

  // Player Text
  JOB_TEXT: string,
  RACE_TEXT: string,
  WEAPON_SLOT_TEXT: string,
  OFFHAND_SLOT_TEXT: string,
  HEAD_SLOT_TEXT: string,
  BODY_SLOT_TEXT: string,
  HANDS_SLOT_TEXT: string,
  LEGS_SLOT_TEXT: string,
  FEET_SLOT_TEXT: string,
  EARS_SLOT_TEXT: string,
  NECK_SLOT_TEXT: string,
  WRIST_SLOT_TEXT: string,
  FINGER1_SLOT_TEXT: string,
  FINGER2_SLOT_TEXT: string,
  FOOD_SLOT_TEXT: string,
  AST_MELEE_PARTNER_TEXT: string,
  AST_RANGED_PARTNER_TEXT: string,
  DNC_PARTNER_TEXT: string,
  FINGER_SLOT_TEXT: string,
  MIDLANDER_HYUR_NAME: string,
  HIGHLANDER_HYUR_NAME: string,
  WILDWOOD_ELEZEN_NAME: string,
  DUSKWIGHT_ELEZEN_NAME: string,
  PLAINSFOLK_LALAFELL_NAME: string,
  DUNESFOLK_LALAFELL_NAME: string,
  SEEKER_OF_THE_SUN_MIQOTE_NAME: string,
  KEEPER_OF_THE_MOON_MIQOTE_NAME: string,
  SEA_WOLVES_ROEGADYN_NAME: string,
  HELLSGUARD_ROEGADYN_NAME: string,
  RAEN_AU_RA_NAME: string,
  XAELA_AU_RA_NAME: string,
  HELIONS_HROTHGAR_NAME: string,
  THE_LOST_HROTHGAR_NAME: string,
  RAVA_VIERA_NAME: string,
  VEENA_VIERA_NAME: string,

  // Page Names
  HOME_PAGE_NAME: string,
  DPS_ANALYSIS_PAGE_NAME: string,
  GEAR_COMPARE_PAGE_NAME: string,
  BEST_PARTNER_PAGE_NAME: string,
  BEST_STAT_PAGE_NAME: string,
  PLAYER_POWER_TEXT: string,
  DPS_ANALYSIS_PARTY_INPUT_INFO_TEXT: string,
  PARTY_MEMBER_ILVL_TEXT: string,
  POT_LABEL_TEXT: string,
  USE_POT_TEXT: string,
  NO_POT_TEXT: string,

  // Home
  DPS_ANALYSIS_DESCRIPTION_TEXT: string,
  GEAR_COMPARE_DESCRIPTION_TEXT: string,
  BEST_STATS_DESCRIPTION_TEXT: string,
  BEST_PARTNER_DESCRIPTION_TEXT: string,

  // Quick Sim
  DPS_ANALYSIS_INPUT_INFO_TEXT: string,

  // Gear Compare
  GEAR_COMPARE_INPUT_INFO_TEXT: string,
  COPY_BUTTON_TEXT: string,

  // Best Partner
  BEST_PARTNER_INPUT_INFO_TEXT: string,

  SPEED_LABEL_TEXT: string,

  // Results Text
  SIMULATION_RESULT_TEXT: string,
  PARTY_MEMBERS_TEXT: string,
  TIME_TEXT: string,

  EDPS_EXPLANATION_TEXT: string,

  // Dps Analysis Results
  DAMAGE_PROFILE_BUTTON_TEXT: string,
  SKILL_TITLE_TEXT: string,
  DAMAGE_PERCENTAGE_TEXT: string,
  TOTAL_DAMAGE_TEXT: string,
  CAST_TEXT: string,

  BEST_TEAMMATE_BUTTON_TEXT: string,
  MEMBER_TEXT: string,
  TOTAL_TEXT: string,

  MY_CONTRIBUTION_BUTTON_TEXT: string,

  ROTATION_SAMPLE_WARNING_TEXT: string,

  SAMURAI_ROTATION_WARNING_TEXT: string,

  MNK_ROTATION_WARNING_TEXT: string,

  ROTATION_SAMPLE_BUTTON_TEXT: string,
  COMBAT_TIME_TEXT: string,
  ABILITY_TEXT: string,
  IMPORTANT_STATUS_TEXT: string,

  GEARSET1_TEXT: string,
  GEARSET2_TEXT: string,

  // Best Partner Results
  BEST_PARTNER_BY_ROLE_TEXT: string,
  TANK_TEXT: string,
  HEALER_TEXT: string,
  DPS_TEXT: string,

  OVERALL_TEXT: string,
  BURST_SECTION_TITLE_TEXT: string,
  BURST_TEXT: string,

  // Best Stats Results
  BEST_STATS_TEXT: string,

  BEST_STATS_NAME_TEXT: string,

  EMPTY_TEXT: string,

  NAME_TEXT: string,
  STAT_TEXT: string,
  PREV_TEXT: string,
  NEXT_TEXT: string,

  WD_STAT_NAME: string,

  STR_STAT_NAME: string,
  DEX_STAT_NAME: string,
  INT_STAT_NAME: string,
  MIND_STAT_NAME: string,

  CRIT_STAT_NAME: string,
  DH_STAT_NAME: string,
  DET_STAT_NAME: string,
  SKS_STAT_NAME: string,
  SPS_STAT_NAME: string,
  TEN_STAT_NAME: string,
  PIE_STAT_NAME: string,

  // Power Names
  ITERATION_NAME: string,
  VARIANCE_NAME: string,
  COMPOSITION_NAME: string,
  WD_POWER_NAME: string,
  MAIN_STAT_POWER_NAME: string,
  CRT_RATE_POWER_NAME: string,
  CRT_POWER_NAME: string,
  DH_RATE_POWER_NAME: string,
  DET_POWER_NAME: string,
  SPEED_POWER_NAME: string,
  TENACITY_POWER_NAME: string,
  GCD_NAME: string,

  // Party Input Text
  TIME_INPUT_LABEL_TEXT: string,
  PARTY_MEMBER_LABEL_TEXT: string,

  language: LanguageMode
}


export const AppLanguageTexts: () => TextDictionary = () => {
  let { language } = useLanguage();

  // Job Names
  const PLD_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? PLD_EN_NAME : PLD_KR_NAME;
  const WAR_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? WAR_EN_NAME : WAR_KR_NAME;
  const DRK_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? DRK_EN_NAME : DRK_KR_NAME;
  const GNB_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? GNB_EN_NAME : GNB_KR_NAME;

  const WHM_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? WHM_EN_NAME : WHM_KR_NAME;
  const SCH_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? SCH_EN_NAME : SCH_KR_NAME;
  const AST_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? AST_EN_NAME : AST_KR_NAME;
  const SGE_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? SGE_EN_NAME : SGE_KR_NAME;

  const DRG_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? DRG_EN_NAME : DRG_KR_NAME;
  const MNK_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? MNK_EN_NAME : MNK_KR_NAME;
  const NIN_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? NIN_EN_NAME : NIN_KR_NAME;
  const SAM_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? SAM_EN_NAME : SAM_KR_NAME;
  const RPR_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? RPR_EN_NAME : RPR_KR_NAME;
  const VPR_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? VPR_EN_NAME : VPR_KR_NAME;

  const BRD_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? BRD_EN_NAME : BRD_KR_NAME;
  const MCH_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? MCH_EN_NAME : MCH_KR_NAME;
  const DNC_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? DNC_EN_NAME : DNC_KR_NAME;

  const BLM_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? BLM_EN_NAME : BLM_KR_NAME;
  const SMN_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? SMN_EN_NAME : SMN_KR_NAME;
  const RDM_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? RDM_EN_NAME : RDM_KR_NAME;
  const PCT_JOB_NAME =
    language === LanguageMode.ENGLISH_MODE ? PCT_EN_NAME : PCT_KR_NAME;


  // Menu Text
  const NAVIGATE_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Navigate" : "빠른 이동";
  const SAVED_GEARSETS_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Saved Gearsets"
      : "저장된 장비셋";
  const LOADOUT_NAME_LABEL_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Name" : "이름";
  const LOADOUT_WRITE_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Write" : "저장";
  const LOADOUT_LOAD_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Load" : "로드";
  const DEFAULT_LOADOUT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Default Loadout"
      : "기본 장비셋";
  const LOAD_COMPLETE_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Load Complete"
      : "불러오기 완료";

  const LOADOUT_NAME_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Loadout Name"
      : "장비셋 이름";

  const OVERWRITE_CONFIRM_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Overwrite gearset?"
      : "장비셋을 덮어쓰시겠습니까?";

  const LOAD_CONFIRM_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Load gearset?"
      : "장비셋을 불러오시겠습니까?";

  const CONFIRM_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Yes" : "확인";

  const CANCEL_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "No" : "취소";



  const JOB_TEXT =
    language === LanguageMode.ENGLISH_MODE ? JOB_EN_TEXT : JOB_KR_TEXT;
  const RACE_TEXT =
    language === LanguageMode.ENGLISH_MODE ? RACE_EN_TEXT : RACE_KR_TEXT;
  const WEAPON_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? WEAPON_SLOT_EN_TEXT
      : WEAPON_SLOT_KR_TEXT;
  const OFFHAND_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? OFFHAND_SLOT_EN_TEXT
      : OFFHAND_SLOT_KR_TEXT;
  const HEAD_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? HEAD_SLOT_EN_TEXT
      : HEAD_SLOT_KR_TEXT;
  const BODY_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? BODY_SLOT_EN_TEXT
      : BODY_SLOT_KR_TEXT;
  const HANDS_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? HANDS_SLOT_EN_TEXT
      : HANDS_SLOT_KR_TEXT;
  const LEGS_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? LEGS_SLOT_EN_TEXT
      : LEGS_SLOT_KR_TEXT;
  const FEET_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? FEET_SLOT_EN_TEXT
      : FEET_SLOT_KR_TEXT;
  const EARS_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? EARS_SLOT_EN_TEXT
      : EARS_SLOT_KR_TEXT;
  const NECK_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? NECK_SLOT_EN_TEXT
      : NECK_SLOT_KR_TEXT;
  const WRIST_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? WRIST_SLOT_EN_TEXT
      : WRIST_SLOT_KR_TEXT;
  const FINGER1_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? FINGER1_SLOT_EN_TEXT
      : FINGER1_SLOT_KR_TEXT;
  const FINGER2_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? FINGER2_SLOT_EN_TEXT
      : FINGER2_SLOT_KR_TEXT;
  const FOOD_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? FOOD_SLOT_EN_TEXT
      : FOOD_SLOT_KR_TEXT;
  const AST_MELEE_PARTNER_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? AST_MELEE_PARTNER_EN_TEXT
      : AST_MELEE_PARTNER_KR_TEXT;
  const AST_RANGED_PARTNER_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? AST_RANGED_PARTNER_EN_TEXT
      : AST_RANGED_PARTNER_KR_TEXT;
  const DNC_PARTNER_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? DNC_PARTNER_EN_TEXT
      : DNC_PARTNER_KR_TEXT;
  const FINGER_SLOT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? FINGER_SLOT_EN_TEXT
      : FINGER_SLOT_KR_TEXT;
  const MIDLANDER_HYUR_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? MIDLANDER_HYUR_EN_NAME
      : MIDLANDER_HYUR_KR_NAME;
  const HIGHLANDER_HYUR_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? HIGHLANDER_HYUR_EN_NAME
      : HIGHLANDER_HYUR_KR_NAME;
  const WILDWOOD_ELEZEN_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? WILDWOOD_ELEZEN_EN_NAME
      : WILDWOOD_ELEZEN_KR_NAME;
  const DUSKWIGHT_ELEZEN_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? DUSKWIGHT_ELEZEN_EN_NAME
      : DUSKWIGHT_ELEZEN_KR_NAME;
  const PLAINSFOLK_LALAFELL_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? PLAINSFOLK_LALAFELL_EN_NAME
      : PLAINSFOLK_LALAFELL_KR_NAME;
  const DUNESFOLK_LALAFELL_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? DUNESFOLK_LALAFELL_EN_NAME
      : DUNESFOLK_LALAFELL_KR_NAME;
  const SEEKER_OF_THE_SUN_MIQOTE_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? SEEKER_OF_THE_SUN_MIQOTE_EN_NAME
      : SEEKER_OF_THE_SUN_MIQOTE_KR_NAME;
  const KEEPER_OF_THE_MOON_MIQOTE_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? KEEPER_OF_THE_MOON_MIQOTE_EN_NAME
      : KEEPER_OF_THE_MOON_MIQOTE_KR_NAME;
  const SEA_WOLVES_ROEGADYN_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? SEA_WOLVES_ROEGADYN_EN_NAME
      : SEA_WOLVES_ROEGADYN_KR_NAME;
  const HELLSGUARD_ROEGADYN_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? HELLSGUARD_ROEGADYN_EN_NAME
      : HELLSGUARD_ROEGADYN_KR_NAME;
  const RAEN_AU_RA_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? RAEN_AU_RA_EN_NAME
      : RAEN_AU_RA_KR_NAME;
  const XAELA_AU_RA_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? XAELA_AU_RA_EN_NAME
      : XAELA_AU_RA_KR_NAME;
  const HELIONS_HROTHGAR_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? HELIONS_HROTHGAR_EN_NAME
      : HELIONS_HROTHGAR_KR_NAME;
  const THE_LOST_HROTHGAR_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? THE_LOST_HROTHGAR_EN_NAME
      : THE_LOST_HROTHGAR_KR_NAME;
  const RAVA_VIERA_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? RAVA_VIERA_EN_NAME
      : RAVA_VIERA_KR_NAME;
  const VEENA_VIERA_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? VEENA_VIERA_EN_NAME
      : VEENA_VIERA_KR_NAME;


  // Page Names
  const HOME_PAGE_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Home" : "시작";
  const DPS_ANALYSIS_PAGE_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? "DPS Analysis"
      : "DPS 분석";
  const GEAR_COMPARE_PAGE_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? "Gear Compare"
      : "장비셋 비교";
  const BEST_PARTNER_PAGE_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? "Best Partner"
      : "시너지 파트너";
  const BEST_STAT_PAGE_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? "Best Stat"
      : "스탯 가중치";

  const PLAYER_POWER_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Specific Player Power"
      : "스탯별 결과 수치";
  const DPS_ANALYSIS_PARTY_INPUT_INFO_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "2. Additional Settings"
      : "2. 파티 관련 설정을 입력해주세요";
  const PARTY_MEMBER_ILVL_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Party iLvl"
      : "파티원 템렙";
  const POT_LABEL_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Pot" : "탕약";
  const USE_POT_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "O(Use)" : "O(사용)";
  const NO_POT_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "X(No)" : "X(미사용)";

  // Home
  const DPS_ANALYSIS_DESCRIPTION_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Quickly analyze DPS for your input gearset."
      : "당신의 장비셋을 입력하고 빠르게 분석해보세요.";
  const GEAR_COMPARE_DESCRIPTION_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Compare two gearsets to see which is better for your damage."
      : "두 장비셋을 비교하여 더 높은 데미지를 내는 장비셋을 확인해보세요.";
  const BEST_STATS_DESCRIPTION_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Calculate which main/sub stats are more valuable to you."
      : "가장 딜상승 기대값이 높은 스탯을 분석합니다.";
  const BEST_PARTNER_DESCRIPTION_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Find out the teammates that will contribute the most RDPS for you(buff jobs only)."
      : "내 시너지를 가장 잘 사용해줄 조합을 찾아줍니다(시너지 직업만).";

  // Quick Sim
  const DPS_ANALYSIS_INPUT_INFO_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "1. Input Your Info"
      : "1. 분석할 장비셋을 입력해주세요";

  // Gear Compare
  const GEAR_COMPARE_INPUT_INFO_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "1. Input Gearsets You Want To Compare"
      : "1. 비교할 두 장비셋을 입력해주세요";
  const COPY_BUTTON_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Copy" : "복사";

  // Best Partner
  const BEST_PARTNER_INPUT_INFO_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "1. Input Your Info"
      : "1. 필요한 정보를 입력해주세요";

  const SPEED_LABEL_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "GCD" : "글쿨";

  // Results Text
  const SIMULATION_RESULT_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Simulation Result"
      : "시뮬레이션 결과";
  const PARTY_MEMBERS_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Party Members" : "파티원";
  const TIME_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Time(s)" : "전투시간(초)";

  const EDPS_EXPLANATION_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "!!! EDPS: RDPS + My total contribution to party members' buffs"
      : "!!! EDPS: RDPS + 파티원들 시너지에 대한 내 전체 기여도";

  // Dps Analysis Results
  const DAMAGE_PROFILE_BUTTON_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Damage Profile"
      : "데미지 분포";
  const SKILL_TITLE_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Skill" : "스킬";
  const DAMAGE_PERCENTAGE_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Dmg%" : "데미지%";
  const TOTAL_DAMAGE_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Total Dmg" : "총 데미지";
  const CAST_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Cast" : "스윙 수";

  const BEST_TEAMMATE_BUTTON_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Buffs Received"
      : "받은 기여도(RDPS)";
  const MEMBER_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Member" : "파티원";
  const TOTAL_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Total" : "합계";

  const MY_CONTRIBUTION_BUTTON_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Buffs Taken"
      : "준 기여도(RDPS)";

  const ROTATION_SAMPLE_WARNING_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "!!!This is a sample of the engine's simulation log, not a guide to playing the job. analyze with care."
      : "!!!이건 직업에 대한 가이드가 아닌, 시뮬레이션 엔진의 샘플 결과일 뿐입니다. 이를 감안하고 분석할 때 주의해주세요.";

  const SAMURAI_ROTATION_WARNING_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "!!!Samurai's 'tengentsu' is simulated as a 35s skill(trying to fit approximately 12 casts in a 6:30 fight)."
      : "!!!사무라이의 '심안'은 35초 스킬로 시뮬레이션 되었습니다(로그를 봤을 때 6:30 전투에서 12번 정도 사용됨).";

  const MNK_ROTATION_WARNING_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "!!!Monk's GCD in real ingame is affected by frame rate and ping, causing slight clips in GCD when double-weaving. The simulation can't take this into account, so Monk's potencies have been tuned a little down to compensate for this."
      : "!!!몽크의 실제 인게임 글쿨은 프레임레이트/핑에 영향을 많이 받고, 글쿨 사이에 논글쿨 두개를 쓰면 조금씩 클리핑이 발생하기도 합니다. 시뮬레이션은 그런 부분을 반영할 수 없어서 대신 몽크의 기본 위력을 조금 낮췄습니다.";

  const ROTATION_SAMPLE_BUTTON_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Rotation Sample"
      : "딜사이클 샘플";
  const COMBAT_TIME_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Combat Time" : "전투 시간";
  const ABILITY_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Ability" : "스킬";
  const IMPORTANT_STATUS_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Important Status"
      : "중요 버프들";

  // Gear Compare Results
  const GEARSET1_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Gearset1" : "장비셋1";
  const GEARSET2_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Gearset2" : "장비셋2";

  // Best Partner Results
  const BEST_PARTNER_BY_ROLE_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Best Partner By Role"
      : "역할군별 최고 파트너";
  const TANK_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Tank" : "탱커";
  const HEALER_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Healer" : "힐러";
  const DPS_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "DPS" : "딜러";

  const OVERALL_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Total Contribution"
      : "총합 기여도";
  const BURST_SECTION_TITLE_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Contributions at Each Burst"
      : "버스트마다의 기여도";
  const BURST_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? `Minute Burst`
      : `분 버스트`;

  // Best Stats Results
  const BEST_STATS_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "RDPS Increase Per Stat Point"
      : "스탯 1당 RDPS 증가 기대값";

  const BEST_STATS_NAME_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "RDPS Increase / 1 Stat Point"
      : "RDPS 증가 / 1 스탯 포인트";

  const EMPTY_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Empty" : "없음";

  // Stat Names
  const NAME_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Name" : "이름";
  const STAT_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Stat" : "스탯";
  const PREV_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Prev" : "이전";
  const NEXT_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Next" : "다음";


  const WD_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE ? WD_STAT_EN_NAME : WD_STAT_KR_NAME;


  const STR_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE ? STR_STAT_EN_NAME : STR_STAT_KR_NAME;
  const DEX_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE ? DEX_STAT_EN_NAME : DEX_STAT_KR_NAME;
  const INT_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE ? INT_STAT_EN_NAME : INT_STAT_KR_NAME;
  const MIND_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE ? MIND_STAT_EN_NAME : MIND_STAT_KR_NAME;
  const CRIT_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? CRIT_STAT_EN_NAME
      : CRIT_STAT_KR_NAME;
  const DH_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? DH_STAT_EN_NAME
      : DH_STAT_KR_NAME;
  const DET_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? DET_STAT_EN_NAME
      : DET_STAT_KR_NAME;
  const SKS_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? SKS_STAT_EN_NAME
      : SKS_STAT_KR_NAME;
  const SPS_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? SPS_STAT_EN_NAME
      : SPS_STAT_KR_NAME;
  const TEN_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? TEN_STAT_EN_NAME
      : TEN_STAT_KR_NAME;
  const PIE_STAT_NAME =
    language === LanguageMode.ENGLISH_MODE
      ? PIE_STAT_EN_NAME
      : PIE_STAT_KR_NAME;

  // Power Names
  const ITERATION_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Iteration" : "시뮬 횟수";
  const VARIANCE_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Error %" : "오차 범위";
  const COMPOSITION_NAME =
    language === LanguageMode.ENGLISH_MODE ? "PT%" : "조합%";
  const WD_POWER_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Weapon" : "무기 공격력";
  const MAIN_STAT_POWER_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Main Stat" : "주 스탯";
  const CRT_RATE_POWER_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Crit Rate" : "극대 확률";
  const CRT_POWER_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Crit Dmg%" : "극대 피해";
  const DH_RATE_POWER_NAME =
    language === LanguageMode.ENGLISH_MODE ? "DH Rate" : "직격 확률";
  const DET_POWER_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Det Rate" : "의지 효과";
  const SPEED_POWER_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Speed" : "도트/평타 위력";
  const TENACITY_POWER_NAME =
    language === LanguageMode.ENGLISH_MODE ? "Tenacity" : "불굴 효과";
  const GCD_NAME =
    language === LanguageMode.ENGLISH_MODE ? "GCD" : "글쿨";

  // Party Input Text
  const TIME_INPUT_LABEL_TEXT =
    language === LanguageMode.ENGLISH_MODE
      ? "Combat Time(Seconds)"
      : "전투 시간(초)";
  const PARTY_MEMBER_LABEL_TEXT =
    language === LanguageMode.ENGLISH_MODE ? "Party Member" : "파티원";


  let allTexts = {
    PLD_JOB_NAME,
    WAR_JOB_NAME,
    DRK_JOB_NAME,
    GNB_JOB_NAME,
    WHM_JOB_NAME,
    SCH_JOB_NAME,
    AST_JOB_NAME,

    SGE_JOB_NAME,

    DRG_JOB_NAME,
    MNK_JOB_NAME,
    NIN_JOB_NAME,
    SAM_JOB_NAME,
    RPR_JOB_NAME,
    VPR_JOB_NAME,

    BRD_JOB_NAME,
    MCH_JOB_NAME,
    DNC_JOB_NAME,

    BLM_JOB_NAME,
    SMN_JOB_NAME,
    RDM_JOB_NAME,
    PCT_JOB_NAME,


    // Menu Text
    NAVIGATE_TEXT,
    SAVED_GEARSETS_TEXT,
    LOADOUT_NAME_LABEL_TEXT,
    LOADOUT_WRITE_TEXT,
    LOADOUT_LOAD_TEXT,
    DEFAULT_LOADOUT_TEXT,
    LOAD_COMPLETE_TEXT,
    LOADOUT_NAME_TEXT,

    OVERWRITE_CONFIRM_TEXT,

    LOAD_CONFIRM_TEXT,
    CONFIRM_TEXT,

    CANCEL_TEXT,

    // Player Text
    JOB_TEXT,
    RACE_TEXT,
    WEAPON_SLOT_TEXT,
    OFFHAND_SLOT_TEXT,
    HEAD_SLOT_TEXT,
    BODY_SLOT_TEXT,
    HANDS_SLOT_TEXT,
    LEGS_SLOT_TEXT,
    FEET_SLOT_TEXT,
    EARS_SLOT_TEXT,
    NECK_SLOT_TEXT,
    WRIST_SLOT_TEXT,
    FINGER1_SLOT_TEXT,
    FINGER2_SLOT_TEXT,
    FOOD_SLOT_TEXT,
    AST_MELEE_PARTNER_TEXT,
    AST_RANGED_PARTNER_TEXT,
    DNC_PARTNER_TEXT,
    FINGER_SLOT_TEXT,
    MIDLANDER_HYUR_NAME,
    HIGHLANDER_HYUR_NAME,
    WILDWOOD_ELEZEN_NAME,
    DUSKWIGHT_ELEZEN_NAME,
    PLAINSFOLK_LALAFELL_NAME,
    DUNESFOLK_LALAFELL_NAME,
    SEEKER_OF_THE_SUN_MIQOTE_NAME,
    KEEPER_OF_THE_MOON_MIQOTE_NAME,
    SEA_WOLVES_ROEGADYN_NAME,
    HELLSGUARD_ROEGADYN_NAME,
    RAEN_AU_RA_NAME,
    XAELA_AU_RA_NAME,
    HELIONS_HROTHGAR_NAME,
    THE_LOST_HROTHGAR_NAME,
    RAVA_VIERA_NAME,
    VEENA_VIERA_NAME,

    // Page Names
    HOME_PAGE_NAME,
    DPS_ANALYSIS_PAGE_NAME,
    GEAR_COMPARE_PAGE_NAME,
    BEST_PARTNER_PAGE_NAME,
    BEST_STAT_PAGE_NAME,
    PLAYER_POWER_TEXT,
    DPS_ANALYSIS_PARTY_INPUT_INFO_TEXT,
    PARTY_MEMBER_ILVL_TEXT,
    POT_LABEL_TEXT,
    USE_POT_TEXT,
    NO_POT_TEXT,

    // Home
    DPS_ANALYSIS_DESCRIPTION_TEXT,
    GEAR_COMPARE_DESCRIPTION_TEXT,
    BEST_STATS_DESCRIPTION_TEXT,
    BEST_PARTNER_DESCRIPTION_TEXT,

    // Quick Sim
    DPS_ANALYSIS_INPUT_INFO_TEXT,

    // Gear Compare
    GEAR_COMPARE_INPUT_INFO_TEXT,
    COPY_BUTTON_TEXT,

    // Best Partner
    BEST_PARTNER_INPUT_INFO_TEXT,

    SPEED_LABEL_TEXT,

    // Results Text
    SIMULATION_RESULT_TEXT,
    PARTY_MEMBERS_TEXT,
    TIME_TEXT,

    EDPS_EXPLANATION_TEXT,

    // Dps Analysis Results
    DAMAGE_PROFILE_BUTTON_TEXT,
    SKILL_TITLE_TEXT,
    DAMAGE_PERCENTAGE_TEXT,
    TOTAL_DAMAGE_TEXT,
    CAST_TEXT,

    BEST_TEAMMATE_BUTTON_TEXT,
    MEMBER_TEXT,
    TOTAL_TEXT,

    MY_CONTRIBUTION_BUTTON_TEXT,

    ROTATION_SAMPLE_WARNING_TEXT,

    SAMURAI_ROTATION_WARNING_TEXT,

    MNK_ROTATION_WARNING_TEXT,

    ROTATION_SAMPLE_BUTTON_TEXT,
    COMBAT_TIME_TEXT,
    ABILITY_TEXT,
    IMPORTANT_STATUS_TEXT,

    GEARSET1_TEXT,
    GEARSET2_TEXT,

    // Best Partner Results
    BEST_PARTNER_BY_ROLE_TEXT,
    TANK_TEXT,
    HEALER_TEXT,
    DPS_TEXT,

    OVERALL_TEXT,
    BURST_SECTION_TITLE_TEXT,
    BURST_TEXT,

    // Best Stats Results
    BEST_STATS_TEXT,

    BEST_STATS_NAME_TEXT,

    EMPTY_TEXT,

    NAME_TEXT,
    STAT_TEXT,
    PREV_TEXT,
    NEXT_TEXT,

    WD_STAT_NAME,

    STR_STAT_NAME,
    DEX_STAT_NAME,
    INT_STAT_NAME,
    MIND_STAT_NAME,

    CRIT_STAT_NAME,
    DH_STAT_NAME,
    DET_STAT_NAME,
    SKS_STAT_NAME,
    SPS_STAT_NAME,
    TEN_STAT_NAME,
    PIE_STAT_NAME,

    // Power Names
    ITERATION_NAME,
    VARIANCE_NAME,
    COMPOSITION_NAME,
    WD_POWER_NAME,
    MAIN_STAT_POWER_NAME,
    CRT_RATE_POWER_NAME,
    CRT_POWER_NAME,
    DH_RATE_POWER_NAME,
    DET_POWER_NAME,
    SPEED_POWER_NAME,
    TENACITY_POWER_NAME,
    GCD_NAME,

    // Party Input Text
    TIME_INPUT_LABEL_TEXT,
    PARTY_MEMBER_LABEL_TEXT,

    language
  }

  const [texts, setTexts] = useState(allTexts);

  useEffect(() => {
    setTexts(allTexts);
  }, [language]);

  return texts;
}

export function convertToRaceTextName(raceName: string, LANGUAGE_TEXTS: TextDictionary) {
  switch (raceName) {
    case MIDLANDER_HYUR_EN_NAME:
      return LANGUAGE_TEXTS.MIDLANDER_HYUR_NAME;
    case HIGHLANDER_HYUR_EN_NAME:
      return LANGUAGE_TEXTS.HIGHLANDER_HYUR_NAME;
    case WILDWOOD_ELEZEN_EN_NAME:
      return LANGUAGE_TEXTS.WILDWOOD_ELEZEN_NAME;
    case DUSKWIGHT_ELEZEN_EN_NAME:
      return LANGUAGE_TEXTS.DUSKWIGHT_ELEZEN_NAME;
    case PLAINSFOLK_LALAFELL_EN_NAME:
      return LANGUAGE_TEXTS.PLAINSFOLK_LALAFELL_NAME;
    case DUNESFOLK_LALAFELL_EN_NAME:
      return LANGUAGE_TEXTS.DUNESFOLK_LALAFELL_NAME;
    case SEEKER_OF_THE_SUN_MIQOTE_EN_NAME:
      return LANGUAGE_TEXTS.SEEKER_OF_THE_SUN_MIQOTE_NAME;
    case KEEPER_OF_THE_MOON_MIQOTE_EN_NAME:
      return LANGUAGE_TEXTS.KEEPER_OF_THE_MOON_MIQOTE_NAME;
    case SEA_WOLVES_ROEGADYN_EN_NAME:
      return LANGUAGE_TEXTS.SEA_WOLVES_ROEGADYN_NAME;
    case HELLSGUARD_ROEGADYN_EN_NAME:
      return LANGUAGE_TEXTS.HELLSGUARD_ROEGADYN_NAME;
    case RAEN_AU_RA_EN_NAME:
      return LANGUAGE_TEXTS.RAEN_AU_RA_NAME;
    case XAELA_AU_RA_EN_NAME:
      return LANGUAGE_TEXTS.XAELA_AU_RA_NAME;
    case HELIONS_HROTHGAR_EN_NAME:
      return LANGUAGE_TEXTS.HELIONS_HROTHGAR_NAME;
    case THE_LOST_HROTHGAR_EN_NAME:
      return LANGUAGE_TEXTS.THE_LOST_HROTHGAR_NAME;
    case RAVA_VIERA_EN_NAME:
      return LANGUAGE_TEXTS.RAVA_VIERA_NAME;
    case VEENA_VIERA_EN_NAME:
      return LANGUAGE_TEXTS.VEENA_VIERA_NAME;
    default:
      return raceName;
  }
}

export function convertToSlotText(slotName: string, LANGUAGE_TEXTS: TextDictionary) {
  switch (slotName) {
    case JOB_EN_TEXT:
      return LANGUAGE_TEXTS.JOB_TEXT;
    case RACE_EN_TEXT:
      return LANGUAGE_TEXTS.RACE_TEXT;
    case WEAPON_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.WEAPON_SLOT_TEXT;
    case OFFHAND_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.OFFHAND_SLOT_TEXT;
    case HEAD_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.HEAD_SLOT_TEXT;
    case BODY_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.BODY_SLOT_TEXT;
    case HANDS_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.HANDS_SLOT_TEXT;
    case LEGS_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.LEGS_SLOT_TEXT;
    case FEET_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.FEET_SLOT_TEXT;
    case EARS_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.EARS_SLOT_TEXT;
    case NECK_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.NECK_SLOT_TEXT;
    case WRIST_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.WRIST_SLOT_TEXT;
    case FINGER1_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.FINGER1_SLOT_TEXT;
    case FINGER2_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.FINGER2_SLOT_TEXT;
    case FOOD_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.FOOD_SLOT_TEXT;
    case AST_MELEE_PARTNER_EN_TEXT:
      return LANGUAGE_TEXTS.AST_MELEE_PARTNER_TEXT;
    case AST_RANGED_PARTNER_EN_TEXT:
      return LANGUAGE_TEXTS.AST_RANGED_PARTNER_TEXT;
    case DNC_PARTNER_EN_TEXT:
      return LANGUAGE_TEXTS.DNC_PARTNER_TEXT;
    case FINGER_SLOT_EN_TEXT:
      return LANGUAGE_TEXTS.FINGER_SLOT_TEXT;
    default:
      return slotName;
  }
}


export const convertToJobText = (jobAbbrev: string, LANGUAGE_TEXTS: TextDictionary) => {
  switch (jobAbbrev) {
    case PLD_EN_NAME:
      return LANGUAGE_TEXTS.PLD_JOB_NAME;
    case WAR_EN_NAME:
      return LANGUAGE_TEXTS.WAR_JOB_NAME;
    case DRK_EN_NAME:
      return LANGUAGE_TEXTS.DRK_JOB_NAME;
    case GNB_EN_NAME:
      return LANGUAGE_TEXTS.GNB_JOB_NAME;
    case WHM_EN_NAME:
      return LANGUAGE_TEXTS.WHM_JOB_NAME;
    case AST_EN_NAME:
      return LANGUAGE_TEXTS.AST_JOB_NAME;
    case SCH_EN_NAME:
      return LANGUAGE_TEXTS.SCH_JOB_NAME;
    case SGE_EN_NAME:
      return LANGUAGE_TEXTS.SGE_JOB_NAME;
    case DRG_EN_NAME:
      return LANGUAGE_TEXTS.DRG_JOB_NAME;
    case MNK_EN_NAME:
      return LANGUAGE_TEXTS.MNK_JOB_NAME;
    case NIN_EN_NAME:
      return LANGUAGE_TEXTS.NIN_JOB_NAME;
    case SAM_EN_NAME:
      return LANGUAGE_TEXTS.SAM_JOB_NAME;
    case RPR_EN_NAME:
      return LANGUAGE_TEXTS.RPR_JOB_NAME;
    case VPR_EN_NAME:
      return LANGUAGE_TEXTS.VPR_JOB_NAME;
    case BRD_EN_NAME:
      return LANGUAGE_TEXTS.BRD_JOB_NAME;
    case MCH_EN_NAME:
      return LANGUAGE_TEXTS.MCH_JOB_NAME;
    case DNC_EN_NAME:
      return LANGUAGE_TEXTS.DNC_JOB_NAME;
    case BLM_EN_NAME:
      return LANGUAGE_TEXTS.BLM_JOB_NAME;
    case SMN_EN_NAME:
      return LANGUAGE_TEXTS.SMN_JOB_NAME;
    case RDM_EN_NAME:
      return LANGUAGE_TEXTS.RDM_JOB_NAME;
    case PCT_EN_NAME:
      return LANGUAGE_TEXTS.PCT_JOB_NAME;
    default:
      return jobAbbrev;
  }
};


