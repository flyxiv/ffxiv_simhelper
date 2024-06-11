export const SkillIdToIconPathFactory = (skillId: number) => {
  const actionIconDirectory = process.env.PUBLIC_URL + "/images/actions";
  switch (skillId) {
    // PLD
    case 1900:
      return actionIconDirectory + "/PLD/requiescat.png";
    case 1901:
      return actionIconDirectory + "/PLD/goring_blade.png";
    case 1902:
      return actionIconDirectory + "/PLD/circle_of_scorn.png";
    case 1903:
      return actionIconDirectory + "/PLD/royal_authority.png";
    case 1904:
      return actionIconDirectory + "/PLD/confiteor.png";
    case 1905:
      return actionIconDirectory + "/PLD/holy_spirit.png";
    case 1906:
      return actionIconDirectory + "/PLD/atonement.png";
    case 1907:
      return actionIconDirectory + "/PLD/explacion.png";
    case 1908:
      return actionIconDirectory + "/PLD/blade_of_faith.png";
    case 1909:
      return actionIconDirectory + "/PLD/blade_of_truth.png";
    case 1910:
      return actionIconDirectory + "/PLD/blade_of_valor.png";
    case 1911:
      return actionIconDirectory + "/PLD/intervene.png";
    case 1912:
      return actionIconDirectory + "/PLD/fight_or_flight.png";
    case 1913:
      return actionIconDirectory + "/PLD/fast_blade.png";
    case 1914:
      return actionIconDirectory + "/PLD/riot_blade.png";
    case 1915:
      return actionIconDirectory + "/PLD/holy_spirit.png";

    // WAR
    case 100:
      return actionIconDirectory + "/WAR/heavy_swing.png";
    case 101:
      return actionIconDirectory + "/WAR/maim.png";
    case 102:
      return actionIconDirectory + "/WAR/storm's_eye.png";
    case 103:
      return actionIconDirectory + "/WAR/infuriate.png";
    case 104:
      return actionIconDirectory + "/WAR/fell_cleave.png";
    case 105:
      return actionIconDirectory + "/WAR/onslaught.png";
    case 106:
      return actionIconDirectory + "/WAR/upheaval.png";
    case 107:
      return actionIconDirectory + "/WAR/inner_release.png";
    case 108:
      return actionIconDirectory + "/WAR/primal_rend.png";
    case 109:
      return actionIconDirectory + "/WAR/storm's_path.png";
    case 110:
      return actionIconDirectory + "/WAR/inner_chaos.png";
    case 111:
      return actionIconDirectory + "/WAR/fell_cleave.png";

    // WHM
    case 400:
      return actionIconDirectory + "/WHM/glare.png";
    case 401:
      return actionIconDirectory + "/WHM/dia.png";
    case 402:
      return actionIconDirectory + "/WHM/afflatus_misery.png";
    case 403:
      return actionIconDirectory + "/WHM/afflatus_rapture.png";

    // SGE
    case 700:
      return actionIconDirectory + "/SGE/eukrasian_dosis_III.png";
    case 701:
      return actionIconDirectory + "/SGE/dosis_III.png";
    case 702:
      return actionIconDirectory + "/SGE/phlegma.png";

    // DRG
    case 800:
      return actionIconDirectory + "/DRG/life_surge.png";
    case 801:
      return actionIconDirectory + "/DRG/true_thrust.png";
    case 802:
      return actionIconDirectory + "/DRG/raiden_thrust.png";
    case 803:
      return actionIconDirectory + "/DRG/vorpal_thrust.png";
    case 804:
      return actionIconDirectory + "/DRG/disembowel.png";
    case 805:
      return actionIconDirectory + "/DRG/heavens'_thrust.png";
    case 806:
      return actionIconDirectory + "/DRG/chaotic_spring.png";
    case 807:
      return actionIconDirectory + "/DRG/fang_and_claw.png";
    case 808:
      return actionIconDirectory + "/DRG/wheeling_thrust.png";
    case 809:
      return actionIconDirectory + "/DRG/fang_and_claw.png";
    case 810:
      return actionIconDirectory + "/DRG/wheeling_thrust.png";
    case 811:
      return actionIconDirectory + "/DRG/lance_charge.png";
    case 812:
      return actionIconDirectory + "/DRG/heavens'_thrust.png";
    case 813:
      return actionIconDirectory + "/DRG/fang_and_claw.png";
    case 814:
      return actionIconDirectory + "/DRG/wheeling_thrust.png";
    case 815:
      return actionIconDirectory + "/DRG/high_jump.png";
    case 816:
      return actionIconDirectory + "/DRG/mirage_dive.png";
    case 817:
      return actionIconDirectory + "/DRG/dragon_sight.png";
    case 818:
      return actionIconDirectory + "/DRG/spineshatter_dive.png";
    case 819:
      return actionIconDirectory + "/DRG/dragonfire_dive.png";
    case 820:
      return actionIconDirectory + "/DRG/battle_litany.png";
    case 821:
      return actionIconDirectory + "/DRG/geirskogul.png";
    case 822:
      return actionIconDirectory + "/DRG/geirskogul.png";
    case 823:
      return actionIconDirectory + "/DRG/nastrond.png";
    case 824:
      return actionIconDirectory + "/DRG/stardiver.png";
    case 825:
      return actionIconDirectory + "/DRG/wyrmwind_thrust.png";

    // MNK
    case 900:
      return actionIconDirectory + "/MNK/bootshine.png";
    case 901:
      return actionIconDirectory + "/MNK/true_strike.png";
    case 902:
      return actionIconDirectory + "/MNK/snap_punch.png";
    case 903:
      return actionIconDirectory + "/MNK/twin_snakes.png";
    case 904:
      return actionIconDirectory + "/MNK/demolish.png";
    case 905:
      return actionIconDirectory + "/MNK/dragon_kick.png";
    case 906:
      return actionIconDirectory + "/MNK/elixir_field.png";
    case 907:
      return actionIconDirectory + "/MNK/rising_pheonix.png";
    case 908:
      return actionIconDirectory + "/MNK/phantom_rush.png";
    case 909:
      return actionIconDirectory + "/MNK/brotherhood.png";
    case 910:
      return actionIconDirectory + "/MNK/riddle_of_fire.png";
    case 911:
      return actionIconDirectory + "/MNK/riddle_of_wind.png";
    case 912:
      return actionIconDirectory + "/MNK/perfect_balance.png";
    case 913:
      return actionIconDirectory + "/MNK/the_forbidden_chakra.png";
    case 914:
      return actionIconDirectory + "/MNK/bootshine.png";
    case 915:
      return actionIconDirectory + "/MNK/true_strike.png";
    case 916:
      return actionIconDirectory + "/MNK/snap_punch.png";
    case 917:
      return actionIconDirectory + "/MNK/twin_snakes.png";
    case 918:
      return actionIconDirectory + "/MNK/demolish.png";
    case 919:
      return actionIconDirectory + "/MNK/dragon_kick.png";

    // NIN
    case 1000:
      return actionIconDirectory + "/NIN/huton.png";
    case 1001:
      return actionIconDirectory + "/NIN/raiton.png";
    case 1002:
      return actionIconDirectory + "/NIN/fleeting_raiju.png";
    case 1003:
      return actionIconDirectory + "/NIN/hyosho_ranryu.png";
    case 1004:
      return actionIconDirectory + "/NIN/suiton.png";
    case 1005:
      return actionIconDirectory + "/NIN/spinning_edge.png";
    case 1006:
      return actionIconDirectory + "/NIN/gust_slash.png";
    case 1007:
      return actionIconDirectory + "/NIN/aeolian_edge.png";
    case 1008:
      return actionIconDirectory + "/NIN/armor_crush.png";
    case 1009:
      return actionIconDirectory + "/NIN/mug.png";
    case 1010:
      return actionIconDirectory + "/NIN/trick_attack.png";
    case 1011:
      return actionIconDirectory + "/NIN/kassatsu.png";
    case 1012:
      return actionIconDirectory + "/NIN/bhavacakra.png";
    case 1013:
      return actionIconDirectory + "/NIN/ten_chi_jin.png";
    case 1014:
      return actionIconDirectory + "/NIN/fuma_shuriken.png";
    case 1015:
      return actionIconDirectory + "/NIN/raiton.png";
    case 1016:
      return actionIconDirectory + "/NIN/suiton.png";
    case 1017:
      return actionIconDirectory + "/NIN/bunshin.png";
    case 1018:
      return actionIconDirectory + "/NIN/dream_within_a_dream.png";
    case 1019:
      return actionIconDirectory + "/NIN/phantom_kamaitachi.png";
    case 1020:
      return actionIconDirectory + "/NIN/meisui.png";
    case 1021:
      return actionIconDirectory + "/NIN/bhavacakra.png";
    case 1022:
      return actionIconDirectory + "/NIN/bunshin.png";
    case 1023:
      return actionIconDirectory + "/NIN/ninjutsu.png";

    // BRD
    case 1300:
      return actionIconDirectory + "/BRD/burst_shot.png";
    case 1301:
      return actionIconDirectory + "/BRD/refulgent_arrow.png";
    case 1302:
      return actionIconDirectory + "/BRD/raging_strike.png";
    case 1303:
      return actionIconDirectory + "/BRD/blood_letter.png";
    case 1304:
      return actionIconDirectory + "/BRD/caustic_bite.png";
    case 1305:
      return actionIconDirectory + "/BRD/stormbite.png";
    case 1306:
      return actionIconDirectory + "/BRD/apex_arrow.png";
    case 1307:
      return actionIconDirectory + "/BRD/sidewinder.png";
    case 1308:
      return actionIconDirectory + "/BRD/iron_jaws.png";
    case 1309:
      return actionIconDirectory + "/BRD/empyreal_arrow.png";
    case 1310:
      return actionIconDirectory + "/BRD/pitch_perfect.png";
    case 1311:
      return actionIconDirectory + "/BRD/battle_voice.png";
    case 1312:
      return actionIconDirectory + "/BRD/the_wanderer's_minuet.png";
    case 1313:
      return actionIconDirectory + "/BRD/mages_ballad.png";
    case 1314:
      return actionIconDirectory + "/BRD/armys_paeon.png";
    case 1315:
      return actionIconDirectory + "/BRD/barrage.png";
    case 1316:
      return actionIconDirectory + "/BRD/blast_arrow.png";
    case 1317:
      return actionIconDirectory + "/BRD/radiant_finale.png";
    case 1318:
      return actionIconDirectory + "/BRD/refulgent_arrow.png";
    case 1319:
      return actionIconDirectory + "/BRD/pitch_perfect.png";
    case 1320:
      return actionIconDirectory + "/BRD/pitch_perfect.png";

    // DNC
    case 1500:
      return actionIconDirectory + "/DNC/cascade.png";
    case 1501:
      return actionIconDirectory + "/DNC/fountain.png";
    case 1502:
      return actionIconDirectory + "/DNC/standard_step.png";
    case 1503:
      return actionIconDirectory + "/DNC/technical_step.png";
    case 1504:
      return actionIconDirectory + "/DNC/devilment.png";
    case 1505:
      return actionIconDirectory + "/DNC/flourish.png";
    case 1506:
      return actionIconDirectory + "/DNC/fan_dance.png";
    case 1507:
      return actionIconDirectory + "/DNC/fan_dance_III.png";
    case 1508:
      return actionIconDirectory + "/DNC/fan_dance_IV.png";
    case 1509:
      return actionIconDirectory + "/DNC/reverse_cascade.png";
    case 1510:
      return actionIconDirectory + "/DNC/fountainfall.png";
    case 1511:
      return actionIconDirectory + "/DNC/saber_dance.png";
    case 1512:
      return actionIconDirectory + "/DNC/starfall_dance.png";
    case 1513:
      return actionIconDirectory + "/DNC/reverse_cascade.png";
    case 1514:
      return actionIconDirectory + "/DNC/fountainfall.png";
    case 1515:
      return actionIconDirectory + "/DNC/tillana.png";

    // BLM
    case 1700:
      return actionIconDirectory + "/BLM/transpose.png";
    case 1701:
      return actionIconDirectory + "/BLM/thunder_III.png";
    case 1702:
      return actionIconDirectory + "/BLM/thunder_III.png";
    case 1703:
      return actionIconDirectory + "/BLM/fire_IV.png";
    case 1704:
      return actionIconDirectory + "/BLM/fire_IV.png";
    case 1705:
      return actionIconDirectory + "/BLM/fire_III.png";
    case 1706:
      return actionIconDirectory + "/BLM/fire_III.png";
    case 1707:
      return actionIconDirectory + "/BLM/despair.png";
    case 1708:
      return actionIconDirectory + "/BLM/despair.png";
    case 1709:
      return actionIconDirectory + "/BLM/despair.png";
    case 1710:
      return actionIconDirectory + "/BLM/xenoglossy.png";
    case 1711:
      return actionIconDirectory + "/BLM/paradox.png";
    case 1712:
      return actionIconDirectory + "/BLM/blizzard_III.png";
    case 1713:
      return actionIconDirectory + "/BLM/blizzard_IV.png";
    case 1714:
      return actionIconDirectory + "/BLM/triplecast.png";
    case 1715:
      return actionIconDirectory + "/BLM/ley_lines.png";
    case 1716:
      return actionIconDirectory + "/BLM/sharpcast.png";
    case 1717:
      return actionIconDirectory + "/BLM/amplifier.png";
    case 1718:
      return actionIconDirectory + "/BLM/fire_III.png";

    case 0:
      return actionIconDirectory + "/BLM/MagicalRangedRollAction/swiftcast.png";
    case 10001:
      return actionIconDirectory + "/auto-attack.png";
    default:
      return `unknown skill id: ${skillId}`;
  }
};
