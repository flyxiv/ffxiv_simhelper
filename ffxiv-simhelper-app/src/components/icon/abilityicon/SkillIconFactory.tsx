import { IMAGES_DIRECTORY } from "../../../const/BaseDirectory";

export const SkillIdToIconPathFactory = (skillId: number) => {
  const actionIconDirectory = `${IMAGES_DIRECTORY}/actions`;
  switch (skillId) {
    // PLD
    case 1900:
      return actionIconDirectory + "/PLD/imperator.png";
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
      return actionIconDirectory + "/PLD/expiacion.png";
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
    case 1916:
      return actionIconDirectory + "/PLD/supplication.png";
    case 1917:
      return actionIconDirectory + "/PLD/sepulchre.png";
    case 1918:
      return actionIconDirectory + "/PLD/blade_of_honor.png";
    case 1919:
      return actionIconDirectory + "/PLD/circle_of_scorn.png";

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
    case 112:
      return actionIconDirectory + "/WAR/primal_wrath.png";
    case 113:
      return actionIconDirectory + "/WAR/primal_ruination.png";

    // DRK
    case 200:
      return actionIconDirectory + "/DRK/hard_slash.png";
    case 201:
      return actionIconDirectory + "/DRK/syphon_strike.png";
    case 202:
      return actionIconDirectory + "/DRK/souleater.png";
    case 203:
      return actionIconDirectory + "/DRK/edge_of_shadow.png";
    case 204:
      return actionIconDirectory + "/DRK/disesteem.png";
    case 205:
      return actionIconDirectory + "/DRK/salted_earth.png";
    case 206:
      return actionIconDirectory + "/DRK/plunge.png";
    case 207:
      return actionIconDirectory + "/DRK/carve_and_spit.png";
    case 208:
      return actionIconDirectory + "/DRK/delirium.png";
    case 209:
      return actionIconDirectory + "/DRK/shadowbringer.png";
    case 210:
      return actionIconDirectory + "/DRK/bloodspiller.png";
    case 211:
      return actionIconDirectory + "/DRK/living_shadow.png";
    case 212:
      return actionIconDirectory + "/DRK/scarlet_delirium.png";
    case 213:
      return actionIconDirectory + "/DRK/salt_and_darkness.png";
    case 214:
      return actionIconDirectory + "/DRK/comeuppance.png";
    case 215:
      return actionIconDirectory + "/DRK/torcleaver.png";

    // GNB
    case 300:
      return actionIconDirectory + "/GNB/keen_edge.png";
    case 301:
      return actionIconDirectory + "/GNB/brutal_shell.png";
    case 302:
      return actionIconDirectory + "/GNB/solid_barrel.png";
    case 303:
      return actionIconDirectory + "/GNB/sonic_break.png";
    case 304:
      return actionIconDirectory + "/GNB/bow_shock.png";
    case 305:
      return actionIconDirectory + "/GNB/no_mercy.png";
    case 306:
      return actionIconDirectory + "/GNB/rough_divide.png";
    case 307:
      return actionIconDirectory + "/GNB/gnashing_fang.png";
    case 308:
      return actionIconDirectory + "/GNB/savage_claw.png";
    case 309:
      return actionIconDirectory + "/GNB/wicked_talon.png";
    case 310:
      return actionIconDirectory + "/GNB/jugular_rip.png";
    case 311:
      return actionIconDirectory + "/GNB/abdomen_tear.png";
    case 312:
      return actionIconDirectory + "/GNB/eye_gouge.png";
    case 313:
      return actionIconDirectory + "/GNB/blasting_zone.png";
    case 314:
      return actionIconDirectory + "/GNB/double_down.png";
    case 315:
      return actionIconDirectory + "/GNB/bloodfest.png";
    case 316:
      return actionIconDirectory + "/GNB/burst_strike.png";
    case 317:
      return actionIconDirectory + "/GNB/hypervelocity.png";
    case 318:
      return actionIconDirectory + "/GNB/reign_of_beasts.png";
    case 319:
      return actionIconDirectory + "/GNB/noble_blood.png";
    case 320:
      return actionIconDirectory + "/GNB/lion_heart.png";
    case 321:
      return actionIconDirectory + "/GNB/sonic_break.png";
    case 322:
      return actionIconDirectory + "/GNB/bow_shock.png";

    // WHM
    case 400:
      return actionIconDirectory + "/WHM/glare_III.png";
    case 401:
      return actionIconDirectory + "/WHM/dia.png";
    case 402:
      return actionIconDirectory + "/WHM/afflatus_misery.png";
    case 403:
      return actionIconDirectory + "/WHM/afflatus_rapture.png";
    case 404:
      return actionIconDirectory + "/WHM/assize.png";
    case 405:
      return actionIconDirectory + "/WHM/presence_of_mind.png";
    case 406:
      return actionIconDirectory + "/WHM/glare_iv.png";
    case 407:
      return actionIconDirectory + "/WHM/dia.png";

    // AST
    case 500:
      return actionIconDirectory + "/AST/fall_malefic.png";
    case 501:
      return actionIconDirectory + "/AST/combust_III.png";
    case 502:
      return actionIconDirectory + "/AST/earthly_star.png";
    case 503:
      return actionIconDirectory + "/AST/astral_draw.png";
    case 504:
      return actionIconDirectory + "/AST/umbral_draw.png";
    case 505:
      return actionIconDirectory + "/AST/the_balance.png";
    case 506:
      return actionIconDirectory + "/AST/the_spear.png";
    case 507:
      return actionIconDirectory + "/AST/divination.png";
    case 508:
      return actionIconDirectory + "/AST/lightspeed.png";
    case 509:
      return actionIconDirectory + "/AST/minor_arcana.png";
    case 510:
      return actionIconDirectory + "/AST/lord_of_crowns.png";
    case 511:
      return actionIconDirectory + "/AST/fall_malefic.png";
    case 512:
      return actionIconDirectory + "/AST/draw.png";
    case 513:
      return actionIconDirectory + "/AST/oracle.png";
    case 514:
      return actionIconDirectory + "/AST/combust_III.png";

    // SCH
    case 600:
      return actionIconDirectory + "/SCH/broil_IV.png";
    case 601:
      return actionIconDirectory + "/SCH/biolysis.png";
    case 602:
      return actionIconDirectory + "/SCH/aetherflow.png";
    case 603:
      return actionIconDirectory + "/SCH/energy_drain.png";
    case 604:
      return actionIconDirectory + "/SCH/dissipation.png";
    case 605:
      return actionIconDirectory + "/SCH/chain_stratagem.png";
    case 606:
      return actionIconDirectory + "/SCH/baneful_impaction.png";
    case 607:
      return actionIconDirectory + "/SCH/biolysis.png";
    case 608:
      return actionIconDirectory + "/SCH/baneful_impaction.png";

    // SGE
    case 700:
      return actionIconDirectory + "/SGE/eukrasian_dosis_III.png";
    case 701:
      return actionIconDirectory + "/SGE/dosis_III.png";
    case 702:
      return actionIconDirectory + "/SGE/phlegma_III.png";
    case 703:
      return actionIconDirectory + "/SGE/psyche.png";
    case 704:
      return actionIconDirectory + "/SGE/eukrasia.png";
    case 705:
      return actionIconDirectory + "/SGE/eukrasian_dosis_III.png";
    case 706:
      return actionIconDirectory + "/SGE/toxikon.png";

    // DRG
    case 800:
      return actionIconDirectory + "/DRG/life_surge.png";
    case 801:
      return actionIconDirectory + "/DRG/true_thrust.png";
    case 802:
      return actionIconDirectory + "/DRG/raiden_thrust.png";
    case 803:
      return actionIconDirectory + "/DRG/lance_barrage.png";
    case 804:
      return actionIconDirectory + "/DRG/spiral_blow.png";
    case 805:
      return actionIconDirectory + "/DRG/heavens'_thrust.png";
    case 806:
      return actionIconDirectory + "/DRG/chaotic_spring.png";
    case 807:
      return actionIconDirectory + "/DRG/fang_and_claw.png";
    case 808:
      return actionIconDirectory + "/DRG/wheeling_thrust.png";
    case 809:
      return actionIconDirectory + "/DRG/drakesbane.png";
    case 810:
      return actionIconDirectory + "/DRG/rise_of_the_dragon.png";
    case 811:
      return actionIconDirectory + "/DRG/lance_charge.png";
    case 812:
      return actionIconDirectory + "/DRG/heavens'_thrust.png";
    case 813:
      return actionIconDirectory + "/DRG/drakesbane.png";
    case 814:
      return actionIconDirectory + "/DRG/starcross.png";
    case 815:
      return actionIconDirectory + "/DRG/high_jump.png";
    case 816:
      return actionIconDirectory + "/DRG/mirage_dive.png";
    case 819:
      return actionIconDirectory + "/DRG/dragonfire_dive.png";
    case 820:
      return actionIconDirectory + "/DRG/battle_litany.png";
    case 821:
      return actionIconDirectory + "/DRG/geirskogul.png";
    case 823:
      return actionIconDirectory + "/DRG/nastrond.png";
    case 824:
      return actionIconDirectory + "/DRG/stardiver.png";
    case 825:
      return actionIconDirectory + "/DRG/wyrmwind_thrust.png";
    case 826:
      return actionIconDirectory + "/DRG/chaotic_spring.png";

    // MNK
    case 900:
      return actionIconDirectory + "/MNK/leaping_opo.png";
    case 901:
      return actionIconDirectory + "/MNK/rising_raptor.png";
    case 902:
      return actionIconDirectory + "/MNK/pouncing_coeurl.png";
    case 903:
      return actionIconDirectory + "/MNK/twin_snakes.png";
    case 904:
      return actionIconDirectory + "/MNK/demolish.png";
    case 905:
      return actionIconDirectory + "/MNK/dragon_kick.png";
    case 906:
      return actionIconDirectory + "/MNK/elixir_burst.png";
    case 907:
      return actionIconDirectory + "/MNK/rising_phoenix.png";
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
      return actionIconDirectory + "/MNK/leaping_opo.png";
    case 915:
      return actionIconDirectory + "/MNK/rising_raptor.png";
    case 916:
      return actionIconDirectory + "/MNK/pouncing_coeurl.png";
    case 917:
      return actionIconDirectory + "/MNK/twin_snakes.png";
    case 918:
      return actionIconDirectory + "/MNK/demolish.png";
    case 919:
      return actionIconDirectory + "/MNK/dragon_kick.png";
    case 920:
      return actionIconDirectory + "/MNK/wind's_reply.png";
    case 921:
      return actionIconDirectory + "/MNK/fire's_reply.png";

    // NIN
    case 1000:
      return actionIconDirectory + "/NIN/zesho_meppo.png";
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
      return actionIconDirectory + "/NIN/dokumori.png";
    case 1010:
      return actionIconDirectory + "/NIN/kunai's_bane.png";
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
    case 1024:
      return actionIconDirectory + "/NIN/tenri_jindo.png";
    case 1025:
      return actionIconDirectory + "/NIN/zesho_meppo.png";

    // SAM
    case 1100:
      return actionIconDirectory + "/SAM/gyofu.png";
    case 1101:
      return actionIconDirectory + "/SAM/jinpu.png";
    case 1102:
      return actionIconDirectory + "/SAM/gekko.png";
    case 1103:
      return actionIconDirectory + "/SAM/shifu.png";
    case 1104:
      return actionIconDirectory + "/SAM/kasha.png";
    case 1105:
      return actionIconDirectory + "/SAM/yukikaze.png";
    case 1106:
      return actionIconDirectory + "/SAM/hissatsu_shinten.png";
    case 1107:
      return actionIconDirectory + "/SAM/hissatsu_senei.png";
    case 1108:
      return actionIconDirectory + "/SAM/meikyo_shisui.png";
    case 1109:
      return actionIconDirectory + "/SAM/tendo_kaeshi_setsugekka.png";
    case 1110:
      return actionIconDirectory + "/SAM/gekko.png";
    case 1111:
      return actionIconDirectory + "/SAM/kasha.png";
    case 1112:
      return actionIconDirectory + "/SAM/yukikaze.png";
    case 1113:
      return actionIconDirectory + "/SAM/ikishoten.png";
    case 1114:
      return actionIconDirectory + "/SAM/hagakure.png";
    case 1115:
      return actionIconDirectory + "/SAM/midare_setsugekka.png";
    case 1116:
      return actionIconDirectory + "/SAM/ogi_namikiri.png";
    case 1117:
      return actionIconDirectory + "/SAM/kaeshi_namikiri.png";
    case 1118:
      return actionIconDirectory + "/SAM/higanbana.png";
    case 1119:
      return actionIconDirectory + "/SAM/higanbana.png";
    case 1120:
      return actionIconDirectory + "/SAM/higanbana.png";
    case 1121:
      return actionIconDirectory + "/SAM/shoha.png";
    case 1122:
      return actionIconDirectory + "/SAM/zanshin.png";
    case 1123:
      return actionIconDirectory + "/SAM/tendo_setsugekka.png";
    case 1124:
      return actionIconDirectory + "/SAM/kaeshi_setsugekka.png";
    case 1125:
      return actionIconDirectory + "/SAM/higanbana.png";
    case 1126:
      return actionIconDirectory + "/SAM/tengentsu.png";

    // RPR
    case 1200:
      return actionIconDirectory + "/RPR/slice.png";
    case 1201:
      return actionIconDirectory + "/RPR/waxing_slice.png";
    case 1202:
      return actionIconDirectory + "/RPR/infernal_slice.png";
    case 1203:
      return actionIconDirectory + "/RPR/shadow_of_death.png";
    case 1204:
      return actionIconDirectory + "/RPR/soul_slice.png";
    case 1205:
      return actionIconDirectory + "/RPR/blood_stalk.png";
    case 1206:
      return actionIconDirectory + "/RPR/gluttony.png";
    case 1207:
      return actionIconDirectory + "/RPR/gallows.png";
    case 1208:
      return actionIconDirectory + "/RPR/gallows.png";
    case 1209:
      return actionIconDirectory + "/RPR/gibbet.png";
    case 1210:
      return actionIconDirectory + "/RPR/harvest_moon.png";
    case 1211:
      return actionIconDirectory + "/RPR/arcane_circle.png";
    case 1212:
      return actionIconDirectory + "/RPR/plentiful_harvest.png";
    case 1213:
      return actionIconDirectory + "/RPR/enshroud.png";
    case 1214:
      return actionIconDirectory + "/RPR/cross_reaping.png";
    case 1215:
      return actionIconDirectory + "/RPR/void_reaping.png";
    case 1216:
      return actionIconDirectory + "/RPR/lemure's_slice.png";
    case 1217:
      return actionIconDirectory + "/RPR/communio.png";
    case 1218:
      return actionIconDirectory + "/RPR/executioner's_gallows.png";
    case 1219:
      return actionIconDirectory + "/RPR/executioner's_gibbet.png";
    case 1220:
      return actionIconDirectory + "/RPR/sacrificium.png";
    case 1221:
      return actionIconDirectory + "/RPR/perfectio.png";
    case 1222:
      return actionIconDirectory + "/RPR/enshroud.png";
    case 1223:
      return actionIconDirectory + "/RPR/slicefill.png";
    case 1224:
      return actionIconDirectory + "/RPR/waxing_slicefill.png";
    case 1225:
      return actionIconDirectory + "/RPR/infernal_slicefill.png";

    // VPR
    case 2100:
      return actionIconDirectory + "/VPR/steel_fangs.png";
    case 2101:
      return actionIconDirectory + "/VPR/dread_fangs.png";
    case 2102:
      return actionIconDirectory + "/VPR/dreadwinder.png";
    case 2103:
      return actionIconDirectory + "/VPR/hunter's_sting.png";
    case 2104:
      return actionIconDirectory + "/VPR/swiftskin's_sting.png";
    case 2105:
      return actionIconDirectory + "/VPR/flanksting_strike.png";
    case 2106:
      return actionIconDirectory + "/VPR/flanksbane_fang.png";
    case 2107:
      return actionIconDirectory + "/VPR/hindsting_strike.png";
    case 2108:
      return actionIconDirectory + "/VPR/hindsbane_fang.png";
    case 2109:
      return actionIconDirectory + "/VPR/normal_filler1.png";
    case 2110:
      return actionIconDirectory + "/VPR/normal_filler2.png";
    case 2111:
      return actionIconDirectory + "/VPR/hunter's_coil.png";
    case 2112:
      return actionIconDirectory + "/VPR/swiftskin's_coil.png";
    case 2113:
      return actionIconDirectory + "/VPR/serpent's_ire.png";
    case 2114:
      return actionIconDirectory + "/VPR/reawaken.png";
    case 2115:
      return actionIconDirectory + "/VPR/first_generation.png";
    case 2116:
      return actionIconDirectory + "/VPR/second_generation.png";
    case 2117:
      return actionIconDirectory + "/VPR/third_generation.png";
    case 2118:
      return actionIconDirectory + "/VPR/fourth_generation.png";
    case 2119:
      return actionIconDirectory + "/VPR/ouroboros.png";
    case 2120:
      return actionIconDirectory + "/VPR/reawaken_filler.png";
    case 2121:
      return actionIconDirectory + "/VPR/reawaken.png";
    case 2122:
      return actionIconDirectory + "/VPR/uncoiled_fury.png";
    case 2123:
      return actionIconDirectory + "/VPR/death_rattle.png";

    // BRD
    case 1300:
      return actionIconDirectory + "/BRD/burst_shot.png";
    case 1301:
      return actionIconDirectory + "/BRD/refulgent_arrow.png";
    case 1302:
      return actionIconDirectory + "/BRD/raging_strikes.png";
    case 1303:
      return actionIconDirectory + "/BRD/heartbreak_shot.png";
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
      return actionIconDirectory + "/BRD/mage's_ballad.png";
    case 1314:
      return actionIconDirectory + "/BRD/army's_paeon.png";
    case 1315:
      return actionIconDirectory + "/BRD/barrage.png";
    case 1316:
      return actionIconDirectory + "/BRD/blast_arrow.png";
    case 1317:
      return actionIconDirectory + "/BRD/radiant_finale.png";
    case 1318:
      return actionIconDirectory + "/BRD/resonant_arrow.png";
    case 1319:
      return actionIconDirectory + "/BRD/pitch_perfect.png";
    case 1320:
      return actionIconDirectory + "/BRD/pitch_perfect.png";
    case 1321:
      return actionIconDirectory + "/BRD/radiant_encore.png";
    case 1322:
      return actionIconDirectory + "/BRD/radiant_encore.png";
    case 1323:
      return actionIconDirectory + "/BRD/refulgent_arrow_barrage.png";
    case 1324:
      return actionIconDirectory + "/BRD/caustic_bite.png";
    case 1325:
      return actionIconDirectory + "/BRD/stormbite.png";

    // MCH
    case 1400:
      return actionIconDirectory + "/MCH/heated_split_shot.png";
    case 1401:
      return actionIconDirectory + "/MCH/heated_slug_shot.png";
    case 1402:
      return actionIconDirectory + "/MCH/heated_clean_shot.png";
    case 1403:
      return actionIconDirectory + "/MCH/drill.png";
    case 1404:
      return actionIconDirectory + "/MCH/air_anchor.png";
    case 1405:
      return actionIconDirectory + "/MCH/chain_saw.png";
    case 1406:
      return actionIconDirectory + "/MCH/reassemble.png";
    case 1407:
      return actionIconDirectory + "/MCH/double_check.png";
    case 1408:
      return actionIconDirectory + "/MCH/checkmate.png";
    case 1409:
      return actionIconDirectory + "/MCH/blazing_shot.png";
    case 1410:
      return actionIconDirectory + "/MCH/wildfire.png";
    case 1411:
      return actionIconDirectory + "/MCH/hypercharge.png";
    case 1412:
      return actionIconDirectory + "/MCH/barrel_stabilizer.png";
    case 1413:
      return actionIconDirectory + "/MCH/automaton_queen.png";
    case 1414:
      return actionIconDirectory + "/MCH/drill.png";
    case 1415:
      return actionIconDirectory + "/MCH/air_anchor.png";
    case 1416:
      return actionIconDirectory + "/MCH/chain_saw.png";
    case 1417:
      return actionIconDirectory + "/MCH/excavator.png";
    case 1418:
      return actionIconDirectory + "/MCH/full_metal_field.png";
    case 1419:
      return actionIconDirectory + "/MCH/excavator.png";
    case 1420:
      return actionIconDirectory + "/MCH/hypercharge.png";
    case 1421:
      return actionIconDirectory + "/MCH/reassemble.png";


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
    case 1516:
      return actionIconDirectory + "/DNC/last_dance.png";
    case 1517:
      return actionIconDirectory + "/DNC/finishing_move.png";
    case 1518:
      return actionIconDirectory + "/DNC/dance_of_the_dawn.png";
    case 1519:
      return actionIconDirectory + "/DNC/standard_step.png";

    // SMN
    case 1600:
      return actionIconDirectory + "/SMN/ruin_III.png";
    case 1601:
      return actionIconDirectory + "/SMN/ruin_IV.png";
    case 1602:
      return actionIconDirectory + "/SMN/searing_light.png";
    case 1603:
      return actionIconDirectory + "/SMN/energy_drain.png";
    case 1604:
      return actionIconDirectory + "/SMN/necrotize.png";
    case 1605:
      return actionIconDirectory + "/SMN/summon_bahamut.png";
    case 1606:
      return actionIconDirectory + "/SMN/enkindle_bahamut.png";
    case 1607:
      return actionIconDirectory + "/SMN/deathflare.png";
    case 1608:
      return actionIconDirectory + "/SMN/astral_impulse.png";
    case 1609:
      return actionIconDirectory + "/SMN/wyrmwave.png";
    case 1610:
      return actionIconDirectory + "/SMN/summon_ifrit_II.png";
    case 1611:
      return actionIconDirectory + "/SMN/crimson_cyclone.png";
    case 1612:
      return actionIconDirectory + "/SMN/crimson_strike.png";
    case 1613:
      return actionIconDirectory + "/SMN/ruby_rite.png";
    case 1614:
      return actionIconDirectory + "/SMN/summon_titan_II.png";
    case 1615:
      return actionIconDirectory + "/SMN/topaz_rite.png";
    case 1616:
      return actionIconDirectory + "/SMN/mountain_buster.png";
    case 1617:
      return actionIconDirectory + "/SMN/summon_garuda_II.png";
    case 1618:
      return actionIconDirectory + "/SMN/emerald_rite.png";
    case 1619:
      return actionIconDirectory + "/SMN/slipstream.png";
    case 1620:
      return actionIconDirectory + "/SMN/summon_phoenix.png";
    case 1621:
      return actionIconDirectory + "/SMN/fountain_of_fire.png";
    case 1622:
      return actionIconDirectory + "/SMN/enkindle_phoenix.png";
    case 1623:
      return actionIconDirectory + "/SMN/scarlet_flame.png";
    case 1625:
      return actionIconDirectory + "/SMN/searing_flash.png";
    case 1626:
      return actionIconDirectory + "/SMN/summon_solar_bahamut.png";
    case 1627:
      return actionIconDirectory + "/SMN/umbral_impulse.png";
    case 1628:
      return actionIconDirectory + "/SMN/sunflare.png";
    case 1629:
      return actionIconDirectory + "/SMN/enkindle_solar_bahamut.png";
    case 1630:
      return actionIconDirectory + "/SMN/luxwave.png";
    case 1631:
      return actionIconDirectory + "/SMN/slipstream.png";

    // BLM
    case 1700:
      return actionIconDirectory + "/BLM/transpose.png";
    case 1701:
      return actionIconDirectory + "/BLM/high_thunder.png";
    case 1702:
      return actionIconDirectory + "/BLM/transpose.png";
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
      return actionIconDirectory + "/BLM/manafont.png";
    case 1717:
      return actionIconDirectory + "/BLM/amplifier.png";
    case 1718:
      return actionIconDirectory + "/BLM/fire_III.png";
    case 1719:
      return actionIconDirectory + "/BLM/fire_IV.png";
    case 1720:
      return actionIconDirectory + "/BLM/flare_star.png";
    case 1721:
      return actionIconDirectory + "/BLM/blizzard_III.png";
    case 1722:
      return actionIconDirectory + "/BLM/flare_star.png";
    case 1723:
      return actionIconDirectory + "/BLM/blizzard_III.png";
    case 1724:
      return actionIconDirectory + "/BLM/fire_III.png";
    case 1725:
      return actionIconDirectory + "/BLM/high_thunder.png";

    // RDM
    case 1800:
      return actionIconDirectory + "/RDM/jolt_III.png";
    case 1801:
      return actionIconDirectory + "/RDM/veraero_III.png";
    case 1802:
      return actionIconDirectory + "/RDM/veraero_III.png";
    case 1803:
      return actionIconDirectory + "/RDM/veraero_III.png";
    case 1804:
      return actionIconDirectory + "/RDM/veraero_III.png";
    case 1805:
      return actionIconDirectory + "/RDM/verthunder_III.png";
    case 1806:
      return actionIconDirectory + "/RDM/verthunder_III.png";
    case 1807:
      return actionIconDirectory + "/RDM/verthunder_III.png";
    case 1808:
      return actionIconDirectory + "/RDM/verstone.png";
    case 1809:
      return actionIconDirectory + "/RDM/verfire.png";
    case 1810:
      return actionIconDirectory + "/RDM/fleche.png";
    case 1811:
      return actionIconDirectory + "/RDM/contre_sixte.png";
    case 1812:
      return actionIconDirectory + "/RDM/corps-a-corps.png";
    case 1813:
      return actionIconDirectory + "/RDM/engagement.png";
    case 1814:
      return actionIconDirectory + "/RDM/embolden.png";
    case 1815:
      return actionIconDirectory + "/RDM/enchanted_riposte.png";
    case 1816:
      return actionIconDirectory + "/RDM/enchanted_zwerchhau.png";
    case 1817:
      return actionIconDirectory + "/RDM/enchanted_redoublement.png";
    case 1818:
      return actionIconDirectory + "/RDM/verholy.png";
    case 1819:
      return actionIconDirectory + "/RDM/verflare.png";
    case 1820:
      return actionIconDirectory + "/RDM/acceleration.png";
    case 1821:
      return actionIconDirectory + "/RDM/manafication.png";
    case 1822:
      return actionIconDirectory + "/RDM/scorch.png";
    case 1823:
      return actionIconDirectory + "/RDM/resolution.png";
    case 1824:
      return actionIconDirectory + "/RDM/vice_of_thorns.png";
    case 1825:
      return actionIconDirectory + "/RDM/grand_impact.png";
    case 1826:
      return actionIconDirectory + "/RDM/prefulgence.png";
    case 1827:
      return actionIconDirectory + "/RDM/enchanted_riposte.png";
    case 1828:
      return actionIconDirectory + "/RDM/enchanted_zwerchhau.png";
    case 1829:
      return actionIconDirectory + "/RDM/enchanted_redoublement.png";
    case 1830:
      return actionIconDirectory + "/RDM/verflare.png";
    case 1831:
      return actionIconDirectory + "/RDM/verholy.png";
    case 1832:
      return actionIconDirectory + "/RDM/scorch.png";
    case 1833:
      return actionIconDirectory + "/RDM/resolution.png";

    // PCT
    case 2000:
      return actionIconDirectory + "/PCT/fire_in_red.png";
    case 2001:
      return actionIconDirectory + "/PCT/aero_in_green.png";
    case 2002:
      return actionIconDirectory + "/PCT/water_in_blue.png";
    case 2003:
      return actionIconDirectory + "/PCT/creature_muse.png";
    case 2005:
      return actionIconDirectory + "/PCT/mog_of_the_ages.png";
    case 2006:
      return actionIconDirectory + "/PCT/pom_motif.png";
    case 2007:
      return actionIconDirectory + "/PCT/pom_muse.png";
    case 2008:
      return actionIconDirectory + "/PCT/wing_motif.png";
    case 2009:
      return actionIconDirectory + "/PCT/winged_muse.png";
    case 2010:
      return actionIconDirectory + "/PCT/hammer_motif.png";
    case 2011:
      return actionIconDirectory + "/PCT/subtractive_pallete.png";
    case 2012:
      return actionIconDirectory + "/PCT/striking_muse.png";
    case 2013:
      return actionIconDirectory + "/PCT/blizzard_in_cyan.png";
    case 2014:
      return actionIconDirectory + "/PCT/stone_in_yellow.png";
    case 2015:
      return actionIconDirectory + "/PCT/thunder_in_magenta.png";
    case 2016:
      return actionIconDirectory + "/PCT/starry_sky_motif.png";
    case 2017:
      return actionIconDirectory + "/PCT/starry_muse.png";
    case 2018:
      return actionIconDirectory + "/PCT/holy_in_white.png";
    case 2019:
      return actionIconDirectory + "/PCT/hammer_stamp.png";
    case 2020:
      return actionIconDirectory + "/PCT/hammer_brush.png";
    case 2021:
      return actionIconDirectory + "/PCT/polishing_hammer.png";
    case 2022:
      return actionIconDirectory + "/PCT/comet_in_black.png";
    case 2023:
      return actionIconDirectory + "/PCT/rainbow_drip.png";
    case 2024:
      return actionIconDirectory + "/PCT/rainbow_drip.png";
    case 2025:
      return actionIconDirectory + "/PCT/claw_motif.png";
    case 2026:
      return actionIconDirectory + "/PCT/clawed_muse.png";
    case 2027:
      return actionIconDirectory + "/PCT/maw_motif.png";
    case 2028:
      return actionIconDirectory + "/PCT/fanged_muse.png";
    case 2029:
      return actionIconDirectory + "/PCT/retribution_of_the_madeem.png";
    case 2030:
      return actionIconDirectory + "/PCT/star_prism.png";
    case 2031:
      return actionIconDirectory + "/PCT/subtractive_pallete.png";
    case 2032:
      return actionIconDirectory + "/PCT/comet_in_black.png";
    case 2033:
      return actionIconDirectory + "/PCT/blizzard_in_cyan.png";
    case 2034:
      return actionIconDirectory + "/PCT/stone_in_yellow.png";
    case 2035:
      return actionIconDirectory + "/PCT/thunder_in_magenta.png";

    case 0:
      return actionIconDirectory + "/BLM/MagicalRangedRollAction/swiftcast.png";
    case 1:
      return actionIconDirectory + "/pot.png";
    case 10001:
      return actionIconDirectory + "/auto-attack.png";
    default:
      return `unknown skill id: ${skillId}`;
  }
};
