import { AppConfigurations, ENGLISH_MODE } from "../Themes";

// Job Names
export const PLD_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "PLD" : "나이트";
export const WAR_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "WAR" : "전사";
export const DRK_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "DRK" : "암흑기사";
export const GNB_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "GNB" : "건브레이커";

export const WHM_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "WHM" : "백마도사";
export const AST_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "AST" : "점성술사";
export const SCH_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "SCH" : "학자";
export const SGE_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "SGE" : "현자";

export const DRG_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "DRG" : "용기사";
export const MNK_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "MNK" : "몽크";
export const NIN_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "NIN" : "닌자";
export const SAM_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "SAM" : "사무라이";
export const RPR_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "RPR" : "리퍼";
export const VPR_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "VPR" : "바이퍼";

export const BRD_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "BRD" : "음유시인";
export const MCH_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "MCH" : "기공사";
export const DNC_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "DNC" : "무도가";

export const BLM_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "BLM" : "흑마도사";
export const SMN_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "SMN" : "소환사";
export const RDM_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "RDM" : "적마도사";
export const PCT_JOB_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "PCT" : "픽토맨서";

export const convert_to_english_job_abbrev = (jobAbbrev: string) => {
    switch (jobAbbrev) {
        case "나이트":
            return "PLD";
        case "전사":
            return "WAR";
        case "암흑기사":
            return "DRK";
        case "건브레이커":
            return "GNB";
        case "백마도사":
            return "WHM";
        case "점성술사":
            return "AST";
        case "학자":
            return "SCH";
        case "현자":
            return "SGE";
        case "용기사":
            return "DRG";
        case "몽크":
            return "MNK";
        case "닌자":
            return "NIN";
        case "사무라이":
            return "SAM";
        case "리퍼":
            return "RPR";
        case "바이퍼":
            return "VPR";
        case "음유시인":
            return "BRD";
        case "기공사":
            return "MCH";
        case "무도가":
            return "DNC";
        case "흑마도사":
            return "BLM";
        case "소환사":
            return "SMN";
        case "적마도사":
            return "RDM";
        case "픽토맨서":
            return "PCT";
    }
}


// Menu Text
export const NAVIGATE_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Navigate" : "빠른 이동";
export const LOADOUT_NAME_LABEL_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Name" : "이름";
export const LOADOUT_OVERWRITE_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Overwrite" : "저장";
export const LOADOUT_LOAD_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Load" : "로드";


// Player Text 
export const JOB_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Job" : "직업";
export const RACE_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Race" : "종족";
export const WEAPON_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "weapon" : "무기";
export const HEAD_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "head" : "머리";
export const BODY_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "body" : "몸통";
export const HANDS_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "hands" : "손";
export const WAIST_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "waist" : "허리";
export const LEGS_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "legs" : "다리";
export const FEET_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "feet" : "발";
export const EARRINGS_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "earrings" : "귀걸이";
export const NECKLACE_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "necklace" : "목걸이";
export const BRACELET_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "bracelet" : "팔찌";
export const FINGER1_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "finger1" : "반지1";
export const FINGER2_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "finger2" : "반지2";
export const FOOD_SLOT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "food" : "음식";
export const AST_MELEE_PARTNER_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Melee Card Target" : "근거리 카드 대상";
export const AST_RANGED_PARTNER_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Ranged Card Target" : "원거리 카드산대산";
export const DNC_PARTNER_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Dance Partner" : "무도가 파트너";



// Race Names
export const MIDLANDER_HYUR_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Midlander Hyur" : "중부 휴런";
export const HIGHLANDER_HYUR_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Highlander Hyur" : "고지 휴런";
export const WILDWOOD_ELEZEN_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Wildwood Elezen" : "숲 엘레젠";
export const DUSKWIGHT_ELEZEN_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Duskwight Elezen" : "밤 엘레젠";
export const PLAINSFOLK_LALAFELL_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Plainsfolk Lalafell" : "평원 라라펠";
export const DUNESFOLK_LALAFELL_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Dunesfolk Lalafell" : "사막 라라펠";
export const SEEKER_OF_THE_SUN_MIQOTE_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Seeker of the Sun Miqo'te" : "태양의 추종자 미코테";
export const KEEPER_OF_THE_MOON_MIQOTE_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Keeper of the Moon Miqo'te" : "달의 수호자 미코테";
export const SEA_WOLVES_ROEGADYN_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Sea Wolves Roegadyn" : "바다늑대 루가딘";
export const HELLSGUARD_ROEGADYN_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Hellsguard Roegadyn" : "불꽃지킴이 루가딘";
export const RAEN_AU_RA_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Raen Au Ra" : "렌 아우라";
export const XAELA_AU_RA_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Xaela Au Ra" : "젤라 아우라";
export const HELIONS_HROTHGAR_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Helions Hrothgar" : "맴도는 별 로스갈";
export const THE_LOST_HROTHGAR_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "The Lost Hrothgar" : "떠도는 별 로스갈";
export const RAVA_VIERA_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Rava Viera" : "라바 비에라";
export const VEENA_VIERA_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Veena Viera" : "비나 비에라";

export const RACES = [
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
];




// Page Names
export const HOME_PAGE_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Home" : "시작";
export const QUICKSIM_PAGE_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Quick Sim" : "빠른 시뮬레이션";
export const GEAR_COMPARE_PAGE_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Gear Compare" : "장비셋 비교";
export const BEST_PARTNER_PAGE_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Best Partner" : "시너지 파트너";
export const STAT_WEIGHTS_PAGE_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Stat Weights" : "스탯 가중치";



export const PLAYER_POWER_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "3. Specific Player Power" : "3. 스탯별 결과 수치";
export const QUICK_SIM_PARTY_INPUT_INFO_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "2. Additional Settings" : "2. 파티 관련 설정을 입력해주세요";

// Home
export const QUICK_SIM_DESCRIPTION_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Quickly get analyze DPS of your input gearset." : "당신의 장비셋을 입력하고 빠르게 분석해보세요.";
export const GEAR_COMPARE_DESCRIPTION_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Compare two gearsets to see which is better for your damage." : "두 장비셋을 비교하여 더 높은 데미지를 내는 장비셋을 확인해보세요.";
export const STAT_WEIGHTS_DESCRIPTION_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Calculate which main/sub stats are more valuable to you." : "가장 딜상승 기대값이 높은 스탯을 분석합니다.";
export const BEST_PARTNER_DESCRIPTION_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Find out the teammates that will contribute the most RDPS for you(buff jobs only)." : "내 시너지를 가장 잘 사용해줄 조합을 찾아줍니다(시너지 직업만).";

// Quick Sim
export const QUICK_SIM_INPUT_INFO_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "1. Input Your Info" : "1. 분석할 장비셋을 입력해주세요";


// Gear Compare
export const GEAR_COMPARE_INPUT_INFO_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "1. Input Gearsets You Want To Compare" : "1. 비교할 두 장비셋을 입력해주세요";
export const COPY_BUTTON_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Copy" : "복사";




// Results Text
export const SIMULATION_RESULT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Simulation Result" : "시뮬레이션 결과";
export const PARTY_MEMBERS_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Party Members" : "파티원";
export const TIME_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Time(s)" : "전투시간(초)";

// Quick Sim Results
export const DAMAGE_PROFILE_BUTTON_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Damage Profile" : "데미지 분포";
export const SKILL_TITLE_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Skill" : "스킬";
export const DAMAGE_PERCENTAGE_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Dmg%" : "데미지%";
export const TOTAL_DAMAGE_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Total Dmg" : "총 데미지";
export const DAMAGE_PER_SECOND_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Cast" : "스윙 수";

export const BEST_TEAMMATE_BUTTON_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Teammate Contributions" : "파티원 기여도";
export const MEMBER_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Member" : "파티원";
export const TOTAL_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Total" : "합계";

export const MY_CONTRIBUTION_BUTTON_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "My Contributions" : "나의 기여도";

export const ROTATION_SAMPLE_BUTTON_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Rotation Sample" : "딜사이클 샘플";
export const COMBAT_TIME_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Combat Time" : "전투 시간";
export const ABILITY_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Ability" : "스킬";
export const IMPORTANT_STATUS_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Important Status" : "중요 버프들";


// Gear Compare Results
export const GEARSET1_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Gearset1" : "장비셋1";
export const GEARSET2_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Gearset2" : "장비셋2";


// Best Partner Results
export const BEST_PARTNER_BY_ROLE_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Best Partner By Role" : "역할군별 최고 파트너";
export const TANK_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Tanks" : "탱커";
export const HEALER_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Healers" : "힐러";
export const MELEE_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "DPS" : "딜러";


// Stat Weights Results
export const STAT_WEIGHTS_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "RDPS Increase Per Stat Point" : "스탯 1당 RDPS 증가 기대값";



// Stat Names
export const NAME_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Name" : "이름";
export const STAT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Stat" : "스탯";
export const PREV_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Prev" : "이전";
export const NEXT_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Next" : "다음";

export const WD_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "WD" : "무공";

export const STR_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "STR" : "힘";
export const DEX_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "DEX" : "민첩";
export const INT_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "INT" : "지능";
export const MIND_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "MND" : "정신";

export const CRIT_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "CRT" : "극대";
export const DH_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "DH" : "직격";
export const DET_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "DET" : "의지";
export const SKS_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "SKS" : "기시";
export const SPS_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "SPS" : "마시";
export const TEN_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "TEN" : "불굴";
export const PIE_STAT_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "PIE" : "신앙";



// Power Names
export const VALUES_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Values" : "수치";
export const WD_POWER_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Weapon" : "무기 공격력";
export const MAIN_STAT_POWER_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Main Stat" : "주 스탯";
export const CRT_RATE_POWER_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Crit Rate" : "극대 확률";
export const CRT_POWER_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Crit" : "극대 피해";
export const DH_RATE_POWER_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "DH Rate" : "직격 확률";
export const DET_POWER_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Det Rate" : "의지 효과";
export const SPEED_POWER_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Speed" : "도트/평타 위력";
export const TENACITY_POWER_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "Tenacity" : "불굴 효과";
export const GCD_NAME = AppConfigurations.languageMode === ENGLISH_MODE ? "GCD" : "글쿨";


// Party Input Text
export const TIME_INPUT_LABEL_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Combat Time(Seconds)" : "전투 시간(초)";
export const PARTY_MEMBER_LABEL_TEXT = AppConfigurations.languageMode === ENGLISH_MODE ? "Party Member" : "파티원";
