import { ACTION_ICONS } from "./StatusIconFactory";

export const SkillIdToIconPath = (skillId: number) => {
  const actionIconDirectory = `/src/assets/images/actions`;
  switch (skillId) {
    // PLD
    case 1900:
      return { directory: actionIconDirectory + "/PLD/imperator.png", name: "imperator" }
    case 1901:
      return { directory: actionIconDirectory + "/PLD/goring_blade.png", name: "goring_blade" }
    case 1902:
      return { directory: actionIconDirectory + "/PLD/circle_of_scorn.png", name: "circle_of_scorn" }
    case 1903:
      return { directory: actionIconDirectory + "/PLD/royal_authority.png", name: "royal_authority" }
    case 1904:
      return { directory: actionIconDirectory + "/PLD/confiteor.png", name: "confiteor" }
    case 1905:
      return { directory: actionIconDirectory + "/PLD/holy_spirit.png", name: "holy_spirit" }
    case 1906:
      return { directory: actionIconDirectory + "/PLD/atonement.png", name: "atonement" }
    case 1907:
      return { directory: actionIconDirectory + "/PLD/expiacion.png", name: "expiacion" }
    case 1908:
      return { directory: actionIconDirectory + "/PLD/blade_of_faith.png", name: "blade_of_faith" }
    case 1909:
      return { directory: actionIconDirectory + "/PLD/blade_of_truth.png", name: "blade_of_truth" }
    case 1910:
      return { directory: actionIconDirectory + "/PLD/blade_of_valor.png", name: "blade_of_valor" }
    case 1911:
      return { directory: actionIconDirectory + "/PLD/intervene.png", name: "intervene" }
    case 1912:
      return { directory: actionIconDirectory + "/PLD/fight_or_flight.png", name: "fight_or_flight" }
    case 1913:
      return { directory: actionIconDirectory + "/PLD/fast_blade.png", name: "fast_blade" }
    case 1914:
      return { directory: actionIconDirectory + "/PLD/riot_blade.png", name: "riot_blade" }
    case 1915:
      return { directory: actionIconDirectory + "/PLD/holy_spirit.png", name: "holy_spirit" }
    case 1916:
      return { directory: actionIconDirectory + "/PLD/supplication.png", name: "supplication" }
    case 1917:
      return { directory: actionIconDirectory + "/PLD/sepulchre.png", name: "sepulchre" }
    case 1918:
      return { directory: actionIconDirectory + "/PLD/blade_of_honor.png", name: "blade_of_honor" }
    case 1919:
      return { directory: actionIconDirectory + "/PLD/circle_of_scorn.png", name: "circle_of_scorn" }

    // WAR
    case 100:
      return { directory: actionIconDirectory + "/WAR/heavy_swing.png", name: "heavy_swing" }
    case 101:
      return { directory: actionIconDirectory + "/WAR/maim.png", name: "maim" }
    case 102:
      return { directory: actionIconDirectory + "/WAR/storm's_eye.png", name: "storm's_eye" }
    case 103:
      return { directory: actionIconDirectory + "/WAR/infuriate.png", name: "infuriate" }
    case 104:
      return { directory: actionIconDirectory + "/WAR/fell_cleave.png", name: "fell_cleave" }
    case 105:
      return { directory: actionIconDirectory + "/WAR/onslaught.png", name: "onslaught" }
    case 106:
      return { directory: actionIconDirectory + "/WAR/upheaval.png", name: "upheaval" }
    case 107:
      return { directory: actionIconDirectory + "/WAR/inner_release.png", name: "inner_release" }
    case 108:
      return { directory: actionIconDirectory + "/WAR/primal_rend.png", name: "primal_rend" }
    case 109:
      return { directory: actionIconDirectory + "/WAR/storm's_path.png", name: "storm's_path" }
    case 110:
      return { directory: actionIconDirectory + "/WAR/inner_chaos.png", name: "inner_chaos" }
    case 111:
      return { directory: actionIconDirectory + "/WAR/fell_cleave.png", name: "fell_cleave" }
    case 112:
      return { directory: actionIconDirectory + "/WAR/primal_wrath.png", name: "primal_wrath" }
    case 113:
      return { directory: actionIconDirectory + "/WAR/primal_ruination.png", name: "primal_ruination" }

    // DRK
    case 200:
      return { directory: actionIconDirectory + "/DRK/hard_slash.png", name: "hard_slash" }
    case 201:
      return { directory: actionIconDirectory + "/DRK/syphon_strike.png", name: "syphon_strike" }
    case 202:
      return { directory: actionIconDirectory + "/DRK/souleater.png", name: "souleater" }
    case 203:
      return { directory: actionIconDirectory + "/DRK/edge_of_shadow.png", name: "edge_of_shadow" }
    case 204:
      return { directory: actionIconDirectory + "/DRK/disesteem.png", name: "disesteem" }
    case 205:
      return { directory: actionIconDirectory + "/DRK/salted_earth.png", name: "salted_earth" }
    case 206:
      return { directory: actionIconDirectory + "/DRK/plunge.png", name: "plunge" }
    case 207:
      return { directory: actionIconDirectory + "/DRK/carve_and_spit.png", name: "carve_and_spit" }
    case 208:
      return { directory: actionIconDirectory + "/DRK/delirium.png", name: "delirium" }
    case 209:
      return { directory: actionIconDirectory + "/DRK/shadowbringer.png", name: "shadowbringer" }
    case 210:
      return { directory: actionIconDirectory + "/DRK/bloodspiller.png", name: "bloodspiller" }
    case 211:
      return { directory: actionIconDirectory + "/DRK/living_shadow.png", name: "living_shadow" }
    case 212:
      return { directory: actionIconDirectory + "/DRK/scarlet_delirium.png", name: "scarlet_delirium" }
    case 213:
      return { directory: actionIconDirectory + "/DRK/salt_and_darkness.png", name: "salt_and_darkness" }
    case 214:
      return { directory: actionIconDirectory + "/DRK/comeuppance.png", name: "comeuppance" }
    case 215:
      return { directory: actionIconDirectory + "/DRK/torcleaver.png", name: "torcleaver" }
    case 216:
      return { directory: actionIconDirectory + "/DRK/the_blackest_night.png", name: "the_blackest_night" }
    case 217:
      return { directory: actionIconDirectory + "/DRK/edge_of_shadow.png", name: "edge_of_shadow" }

    // GNB
    case 300:
      return { directory: actionIconDirectory + "/GNB/keen_edge.png", name: "keen_edge" }
    case 301:
      return { directory: actionIconDirectory + "/GNB/brutal_shell.png", name: "brutal_shell" }
    case 302:
      return { directory: actionIconDirectory + "/GNB/solid_barrel.png", name: "solid_barrel" }
    case 303:
      return { directory: actionIconDirectory + "/GNB/sonic_break.png", name: "sonic_break" }
    case 304:
      return { directory: actionIconDirectory + "/GNB/bow_shock.png", name: "bow_shock" }
    case 305:
      return { directory: actionIconDirectory + "/GNB/no_mercy.png", name: "no_mercy" }
    case 306:
      return { directory: actionIconDirectory + "/GNB/rough_divide.png", name: "rough_divide" }
    case 307:
      return { directory: actionIconDirectory + "/GNB/gnashing_fang.png", name: "gnashing_fang" }
    case 308:
      return { directory: actionIconDirectory + "/GNB/savage_claw.png", name: "savage_claw" }
    case 309:
      return { directory: actionIconDirectory + "/GNB/wicked_talon.png", name: "wicked_talon" }
    case 310:
      return { directory: actionIconDirectory + "/GNB/jugular_rip.png", name: "jugular_rip" }
    case 311:
      return { directory: actionIconDirectory + "/GNB/abdomen_tear.png", name: "abdomen_tear" }
    case 312:
      return { directory: actionIconDirectory + "/GNB/eye_gouge.png", name: "eye_gouge" }
    case 313:
      return { directory: actionIconDirectory + "/GNB/blasting_zone.png", name: "blasting_zone" }
    case 314:
      return { directory: actionIconDirectory + "/GNB/double_down.png", name: "double_down" }
    case 315:
      return { directory: actionIconDirectory + "/GNB/bloodfest.png", name: "bloodfest" }
    case 316:
      return { directory: actionIconDirectory + "/GNB/burst_strike.png", name: "burst_strike" }
    case 317:
      return { directory: actionIconDirectory + "/GNB/hypervelocity.png", name: "hypervelocity" }
    case 318:
      return { directory: actionIconDirectory + "/GNB/reign_of_beasts.png", name: "reign_of_beasts" }
    case 319:
      return { directory: actionIconDirectory + "/GNB/noble_blood.png", name: "noble_blood" }
    case 320:
      return { directory: actionIconDirectory + "/GNB/lion_heart.png", name: "lion_heart" }
    case 321:
      return { directory: actionIconDirectory + "/GNB/sonic_break.png", name: "sonic_break" }
    case 322:
      return { directory: actionIconDirectory + "/GNB/bow_shock.png", name: "bow_shock" }

    // WHM
    case 400:
      return { directory: actionIconDirectory + "/WHM/glare_III.png", name: "glare_III" }
    case 401:
      return { directory: actionIconDirectory + "/WHM/dia.png", name: "dia" }
    case 402:
      return { directory: actionIconDirectory + "/WHM/afflatus_misery.png", name: "afflatus_misery" }
    case 403:
      return { directory: actionIconDirectory + "/WHM/afflatus_rapture.png", name: "afflatus_rapture" }
    case 404:
      return { directory: actionIconDirectory + "/WHM/assize.png", name: "assize" }
    case 405:
      return { directory: actionIconDirectory + "/WHM/presence_of_mind.png", name: "presence_of_mind" }
    case 406:
      return { directory: actionIconDirectory + "/WHM/glare_iv.png", name: "glare_iv" }
    case 407:
      return { directory: actionIconDirectory + "/WHM/dia.png", name: "dia" }

    // AST
    case 500:
      return { directory: actionIconDirectory + "/AST/fall_malefic.png", name: "fall_malefic" }
    case 501:
      return { directory: actionIconDirectory + "/AST/combust_III.png", name: "combust_III" }
    case 502:
      return { directory: actionIconDirectory + "/AST/earthly_star.png", name: "earthly_star" }
    case 503:
      return { directory: actionIconDirectory + "/AST/astral_draw.png", name: "astral_draw" }
    case 504:
      return { directory: actionIconDirectory + "/AST/umbral_draw.png", name: "umbral_draw" }
    case 505:
      return { directory: actionIconDirectory + "/AST/the_balance.png", name: "the_balance" }
    case 506:
      return { directory: actionIconDirectory + "/AST/the_spear.png", name: "the_spear" }
    case 507:
      return { directory: actionIconDirectory + "/AST/divination.png", name: "divination" }
    case 508:
      return { directory: actionIconDirectory + "/AST/lightspeed.png", name: "lightspeed" }
    case 509:
      return { directory: actionIconDirectory + "/AST/minor_arcana.png", name: "minor_arcana" }
    case 510:
      return { directory: actionIconDirectory + "/AST/lord_of_crowns.png", name: "lord_of_crowns" }
    case 511:
      return { directory: actionIconDirectory + "/AST/fall_malefic.png", name: "fall_malefic" }
    case 512:
      return { directory: actionIconDirectory + "/AST/draw.png", name: "draw" }
    case 513:
      return { directory: actionIconDirectory + "/AST/oracle.png", name: "oracle" }
    case 514:
      return { directory: actionIconDirectory + "/AST/combust_III.png", name: "combust_III" }

    // SCH
    case 600:
      return { directory: actionIconDirectory + "/SCH/broil_IV.png", name: "broil_IV" }
    case 601:
      return { directory: actionIconDirectory + "/SCH/biolysis.png", name: "biolysis" }
    case 602:
      return { directory: actionIconDirectory + "/SCH/aetherflow.png", name: "aetherflow" }
    case 603:
      return { directory: actionIconDirectory + "/SCH/energy_drain.png", name: "energy_drain" }
    case 604:
      return { directory: actionIconDirectory + "/SCH/dissipation.png", name: "dissipation" }
    case 605:
      return { directory: actionIconDirectory + "/SCH/chain_stratagem.png", name: "chain_stratagem" }
    case 606:
      return { directory: actionIconDirectory + "/SCH/baneful_impaction.png", name: "baneful_impaction" }
    case 607:
      return { directory: actionIconDirectory + "/SCH/biolysis.png", name: "biolysis" }
    case 608:
      return { directory: actionIconDirectory + "/SCH/baneful_impaction.png", name: "baneful_impaction" }

    // SGE
    case 700:
      return { directory: actionIconDirectory + "/SGE/eukrasian_dosis_III.png", name: "eukrasian_dosis_III" }
    case 701:
      return { directory: actionIconDirectory + "/SGE/dosis_III.png", name: "dosis_III" }
    case 702:
      return { directory: actionIconDirectory + "/SGE/phlegma_III.png", name: "phlegma_III" }
    case 703:
      return { directory: actionIconDirectory + "/SGE/psyche.png", name: "psyche" }
    case 704:
      return { directory: actionIconDirectory + "/SGE/eukrasia.png", name: "eukrasia" }
    case 705:
      return { directory: actionIconDirectory + "/SGE/eukrasian_dosis_III.png", name: "eukrasian_dosis_III" }
    case 706:
      return { directory: actionIconDirectory + "/SGE/toxikon.png", name: "toxikon" }

    // DRG
    case 800:
      return { directory: actionIconDirectory + "/DRG/life_surge.png", name: "life_surge" }
    case 801:
      return { directory: actionIconDirectory + "/DRG/true_thrust.png", name: "true_thrust" }
    case 802:
      return { directory: actionIconDirectory + "/DRG/raiden_thrust.png", name: "raiden_thrust" }
    case 803:
      return { directory: actionIconDirectory + "/DRG/lance_barrage.png", name: "lance_barrage" }
    case 804:
      return { directory: actionIconDirectory + "/DRG/spiral_blow.png", name: "spiral_blow" }
    case 805:
      return { directory: actionIconDirectory + "/DRG/heavens'_thrust.png", name: "heavens'_thrust" }
    case 806:
      return { directory: actionIconDirectory + "/DRG/chaotic_spring.png", name: "chaotic_spring" }
    case 807:
      return { directory: actionIconDirectory + "/DRG/fang_and_claw.png", name: "fang_and_claw" }
    case 808:
      return { directory: actionIconDirectory + "/DRG/wheeling_thrust.png", name: "wheeling_thrust" }
    case 809:
      return { directory: actionIconDirectory + "/DRG/drakesbane.png", name: "drakesbane" }
    case 810:
      return { directory: actionIconDirectory + "/DRG/rise_of_the_dragon.png", name: "rise_of_the_dragon" }
    case 811:
      return { directory: actionIconDirectory + "/DRG/lance_charge.png", name: "lance_charge" }
    case 812:
      return { directory: actionIconDirectory + "/DRG/heavens'_thrust.png", name: "heavens'_thrust" }
    case 813:
      return { directory: actionIconDirectory + "/DRG/drakesbane.png", name: "drakesbane" }
    case 814:
      return { directory: actionIconDirectory + "/DRG/starcross.png", name: "starcross" }
    case 815:
      return { directory: actionIconDirectory + "/DRG/high_jump.png", name: "high_jump" }
    case 816:
      return { directory: actionIconDirectory + "/DRG/mirage_dive.png", name: "mirage_dive" }
    case 819:
      return { directory: actionIconDirectory + "/DRG/dragonfire_dive.png", name: "dragonfire_dive" }
    case 820:
      return { directory: actionIconDirectory + "/DRG/battle_litany.png", name: "battle_litany" }
    case 821:
      return { directory: actionIconDirectory + "/DRG/geirskogul.png", name: "geirskogul" }
    case 823:
      return { directory: actionIconDirectory + "/DRG/nastrond.png", name: "nastrond" }
    case 824:
      return { directory: actionIconDirectory + "/DRG/stardiver.png", name: "stardiver" }
    case 825:
      return { directory: actionIconDirectory + "/DRG/wyrmwind_thrust.png", name: "wyrmwind_thrust" }
    case 826:
      return { directory: actionIconDirectory + "/DRG/chaotic_spring.png", name: "chaotic_spring" }

    // MNK
    case 900:
      return { directory: actionIconDirectory + "/MNK/leaping_opo.png", name: "leaping_opo" }
    case 901:
      return { directory: actionIconDirectory + "/MNK/rising_raptor.png", name: "rising_raptor" }
    case 902:
      return { directory: actionIconDirectory + "/MNK/pouncing_coeurl.png", name: "pouncing_coeurl" }
    case 903:
      return { directory: actionIconDirectory + "/MNK/twin_snakes.png", name: "twin_snakes" }
    case 904:
      return { directory: actionIconDirectory + "/MNK/demolish.png", name: "demolish" }
    case 905:
      return { directory: actionIconDirectory + "/MNK/dragon_kick.png", name: "dragon_kick" }
    case 906:
      return { directory: actionIconDirectory + "/MNK/elixir_burst.png", name: "elixir_burst" }
    case 907:
      return { directory: actionIconDirectory + "/MNK/rising_phoenix.png", name: "rising_phoenix" }
    case 908:
      return { directory: actionIconDirectory + "/MNK/phantom_rush.png", name: "phantom_rush" }
    case 909:
      return { directory: actionIconDirectory + "/MNK/brotherhood.png", name: "brotherhood" }
    case 910:
      return { directory: actionIconDirectory + "/MNK/riddle_of_fire.png", name: "riddle_of_fire" }
    case 911:
      return { directory: actionIconDirectory + "/MNK/riddle_of_wind.png", name: "riddle_of_wind" }
    case 912:
      return { directory: actionIconDirectory + "/MNK/perfect_balance.png", name: "perfect_balance" }
    case 913:
      return { directory: actionIconDirectory + "/MNK/the_forbidden_chakra.png", name: "the_forbidden_chakra" }
    case 914:
      return { directory: actionIconDirectory + "/MNK/leaping_opo.png", name: "leaping_opo" }
    case 915:
      return { directory: actionIconDirectory + "/MNK/rising_raptor.png", name: "rising_raptor" }
    case 916:
      return { directory: actionIconDirectory + "/MNK/pouncing_coeurl.png", name: "pouncing_coeurl" }
    case 917:
      return { directory: actionIconDirectory + "/MNK/twin_snakes.png", name: "twin_snakes" }
    case 918:
      return { directory: actionIconDirectory + "/MNK/demolish.png", name: "demolish" }
    case 919:
      return { directory: actionIconDirectory + "/MNK/dragon_kick.png", name: "dragon_kick" }
    case 920:
      return { directory: actionIconDirectory + "/MNK/wind's_reply.png", name: "wind's_reply" }
    case 921:
      return { directory: actionIconDirectory + "/MNK/fire's_reply.png", name: "fire's_reply" }

    // NIN
    case 1000:
      return { directory: actionIconDirectory + "/NIN/zesho_meppo.png", name: "zesho_meppo" }
    case 1001:
      return { directory: actionIconDirectory + "/NIN/raiton.png", name: "raiton" }
    case 1002:
      return { directory: actionIconDirectory + "/NIN/fleeting_raiju.png", name: "fleeting_raiju" }
    case 1003:
      return { directory: actionIconDirectory + "/NIN/hyosho_ranryu.png", name: "hyosho_ranryu" }
    case 1004:
      return { directory: actionIconDirectory + "/NIN/suiton.png", name: "suiton" }
    case 1005:
      return { directory: actionIconDirectory + "/NIN/spinning_edge.png", name: "spinning_edge" }
    case 1006:
      return { directory: actionIconDirectory + "/NIN/gust_slash.png", name: "gust_slash" }
    case 1007:
      return { directory: actionIconDirectory + "/NIN/aeolian_edge.png", name: "aeolian_edge" }
    case 1008:
      return { directory: actionIconDirectory + "/NIN/armor_crush.png", name: "armor_crush" }
    case 1009:
      return { directory: actionIconDirectory + "/NIN/dokumori.png", name: "dokumori" }
    case 1010:
      return { directory: actionIconDirectory + "/NIN/kunai's_bane.png", name: "kunai's_bane" }
    case 1011:
      return { directory: actionIconDirectory + "/NIN/kassatsu.png", name: "kassatsu" }
    case 1012:
      return { directory: actionIconDirectory + "/NIN/bhavacakra.png", name: "bhavacakra" }
    case 1013:
      return { directory: actionIconDirectory + "/NIN/ten_chi_jin.png", name: "ten_chi_jin" }
    case 1014:
      return { directory: actionIconDirectory + "/NIN/fuma_shuriken.png", name: "fuma_shuriken" }
    case 1015:
      return { directory: actionIconDirectory + "/NIN/raiton.png", name: "raiton" }
    case 1016:
      return { directory: actionIconDirectory + "/NIN/suiton.png", name: "suiton" }
    case 1017:
      return { directory: actionIconDirectory + "/NIN/bunshin.png", name: "bunshin" }
    case 1018:
      return { directory: actionIconDirectory + "/NIN/dream_within_a_dream.png", name: "dream_within_a_dream" }
    case 1019:
      return { directory: actionIconDirectory + "/NIN/phantom_kamaitachi.png", name: "phantom_kamaitachi" }
    case 1020:
      return { directory: actionIconDirectory + "/NIN/meisui.png", name: "meisui" }
    case 1021:
      return { directory: actionIconDirectory + "/NIN/bhavacakra.png", name: "bhavacakra" }
    case 1022:
      return { directory: actionIconDirectory + "/NIN/bunshin.png", name: "bunshin" }
    case 1023:
      return { directory: actionIconDirectory + "/NIN/ninjutsu.png", name: "ninjutsu" }
    case 1024:
      return { directory: actionIconDirectory + "/NIN/tenri_jindo.png", name: "tenri_jindo" }
    case 1025:
      return { directory: actionIconDirectory + "/NIN/zesho_meppo.png", name: "zesho_meppo" }

    // SAM
    case 1100:
      return { directory: actionIconDirectory + "/SAM/gyofu.png", name: "gyofu" }
    case 1101:
      return { directory: actionIconDirectory + "/SAM/jinpu.png", name: "jinpu" }
    case 1102:
      return { directory: actionIconDirectory + "/SAM/gekko.png", name: "gekko" }
    case 1103:
      return { directory: actionIconDirectory + "/SAM/shifu.png", name: "shifu" }
    case 1104:
      return { directory: actionIconDirectory + "/SAM/kasha.png", name: "kasha" }
    case 1105:
      return { directory: actionIconDirectory + "/SAM/yukikaze.png", name: "yukikaze" }
    case 1106:
      return { directory: actionIconDirectory + "/SAM/hissatsu_shinten.png", name: "hissatsu_shinten" }
    case 1107:
      return { directory: actionIconDirectory + "/SAM/hissatsu_senei.png", name: "hissatsu_senei" }
    case 1108:
      return { directory: actionIconDirectory + "/SAM/meikyo_shisui.png", name: "meikyo_shisui" }
    case 1109:
      return { directory: actionIconDirectory + "/SAM/tendo_kaeshi_setsugekka.png", name: "tendo_kaeshi_setsugekka" }
    case 1110:
      return { directory: actionIconDirectory + "/SAM/gekko.png", name: "gekko" }
    case 1111:
      return { directory: actionIconDirectory + "/SAM/kasha.png", name: "kasha" }
    case 1112:
      return { directory: actionIconDirectory + "/SAM/yukikaze.png", name: "yukikaze" }
    case 1113:
      return { directory: actionIconDirectory + "/SAM/ikishoten.png", name: "ikishoten" }
    case 1114:
      return { directory: actionIconDirectory + "/SAM/hagakure.png", name: "hagakure" }
    case 1115:
      return { directory: actionIconDirectory + "/SAM/midare_setsugekka.png", name: "midare_setsugekka" }
    case 1116:
      return { directory: actionIconDirectory + "/SAM/ogi_namikiri.png", name: "ogi_namikiri" }
    case 1117:
      return { directory: actionIconDirectory + "/SAM/kaeshi_namikiri.png", name: "kaeshi_namikiri" }
    case 1118:
      return { directory: actionIconDirectory + "/SAM/higanbana.png", name: "higanbana" }
    case 1119:
      return { directory: actionIconDirectory + "/SAM/higanbana.png", name: "higanbana" }
    case 1120:
      return { directory: actionIconDirectory + "/SAM/higanbana.png", name: "higanbana" }
    case 1121:
      return { directory: actionIconDirectory + "/SAM/shoha.png", name: "shoha" }
    case 1122:
      return { directory: actionIconDirectory + "/SAM/zanshin.png", name: "zanshin" }
    case 1123:
      return { directory: actionIconDirectory + "/SAM/tendo_setsugekka.png", name: "tendo_setsugekka" }
    case 1124:
      return { directory: actionIconDirectory + "/SAM/kaeshi_setsugekka.png", name: "kaeshi_setsugekka" }
    case 1125:
      return { directory: actionIconDirectory + "/SAM/higanbana.png", name: "higanbana" }
    case 1126:
      return { directory: actionIconDirectory + "/SAM/tengentsu.png", name: "tengentsu" }

    // RPR
    case 1200:
      return { directory: actionIconDirectory + "/RPR/slice.png", name: "slice" }
    case 1201:
      return { directory: actionIconDirectory + "/RPR/waxing_slice.png", name: "waxing_slice" }
    case 1202:
      return { directory: actionIconDirectory + "/RPR/infernal_slice.png", name: "infernal_slice" }
    case 1203:
      return { directory: actionIconDirectory + "/RPR/shadow_of_death.png", name: "shadow_of_death" }
    case 1204:
      return { directory: actionIconDirectory + "/RPR/soul_slice.png", name: "soul_slice" }
    case 1205:
      return { directory: actionIconDirectory + "/RPR/blood_stalk.png", name: "blood_stalk" }
    case 1206:
      return { directory: actionIconDirectory + "/RPR/gluttony.png", name: "gluttony" }
    case 1207:
      return { directory: actionIconDirectory + "/RPR/gallows.png", name: "gallows" }
    case 1208:
      return { directory: actionIconDirectory + "/RPR/gallows.png", name: "gallows" }
    case 1209:
      return { directory: actionIconDirectory + "/RPR/gibbet.png", name: "gibbet" }
    case 1210:
      return { directory: actionIconDirectory + "/RPR/harvest_moon.png", name: "harvest_moon" }
    case 1211:
      return { directory: actionIconDirectory + "/RPR/arcane_circle.png", name: "arcane_circle" }
    case 1212:
      return { directory: actionIconDirectory + "/RPR/plentiful_harvest.png", name: "plentiful_harvest" }
    case 1213:
      return { directory: actionIconDirectory + "/RPR/enshroud.png", name: "enshroud" }
    case 1214:
      return { directory: actionIconDirectory + "/RPR/cross_reaping.png", name: "cross_reaping" }
    case 1215:
      return { directory: actionIconDirectory + "/RPR/void_reaping.png", name: "void_reaping" }
    case 1216:
      return { directory: actionIconDirectory + "/RPR/lemure's_slice.png", name: "lemure's_slice" }
    case 1217:
      return { directory: actionIconDirectory + "/RPR/communio.png", name: "communio" }
    case 1218:
      return { directory: actionIconDirectory + "/RPR/executioner's_gallows.png", name: "executioner's_gallows" }
    case 1219:
      return { directory: actionIconDirectory + "/RPR/executioner's_gibbet.png", name: "executioner's_gibbet" }
    case 1220:
      return { directory: actionIconDirectory + "/RPR/sacrificium.png", name: "sacrificium" }
    case 1221:
      return { directory: actionIconDirectory + "/RPR/perfectio.png", name: "perfectio" }
    case 1222:
      return { directory: actionIconDirectory + "/RPR/enshroud.png", name: "enshroud" }
    case 1223:
      return { directory: actionIconDirectory + "/RPR/slicefill.png", name: "slicefill" }
    case 1224:
      return { directory: actionIconDirectory + "/RPR/waxing_slicefill.png", name: "waxing_slicefill" }
    case 1225:
      return { directory: actionIconDirectory + "/RPR/infernal_slicefill.png", name: "infernal_slicefill" }

    // VPR
    case 2100:
      return { directory: actionIconDirectory + "/VPR/steel_fangs.png", name: "steel_fangs" }
    case 2101:
      return { directory: actionIconDirectory + "/VPR/dread_fangs.png", name: "dread_fangs" }
    case 2102:
      return { directory: actionIconDirectory + "/VPR/dreadwinder.png", name: "dreadwinder" }
    case 2103:
      return { directory: actionIconDirectory + "/VPR/hunter's_sting.png", name: "hunter's_sting" }
    case 2104:
      return { directory: actionIconDirectory + "/VPR/swiftskin's_sting.png", name: "swiftskin's_sting" }
    case 2105:
      return { directory: actionIconDirectory + "/VPR/flanksting_strike.png", name: "flanksting_strike" }
    case 2106:
      return { directory: actionIconDirectory + "/VPR/flanksbane_fang.png", name: "flanksbane_fang" }
    case 2107:
      return { directory: actionIconDirectory + "/VPR/hindsting_strike.png", name: "hindsting_strike" }
    case 2108:
      return { directory: actionIconDirectory + "/VPR/hindsbane_fang.png", name: "hindsbane_fang" }
    case 2109:
      return { directory: actionIconDirectory + "/VPR/normal_filler1.png", name: "normal_filler1" }
    case 2110:
      return { directory: actionIconDirectory + "/VPR/normal_filler2.png", name: "normal_filler2" }
    case 2111:
      return { directory: actionIconDirectory + "/VPR/hunter's_coil.png", name: "hunter's_coil" }
    case 2112:
      return { directory: actionIconDirectory + "/VPR/swiftskin's_coil.png", name: "swiftskin's_coil" }
    case 2113:
      return { directory: actionIconDirectory + "/VPR/serpent's_ire.png", name: "serpent's_ire" }
    case 2114:
      return { directory: actionIconDirectory + "/VPR/reawaken.png", name: "reawaken" }
    case 2115:
      return { directory: actionIconDirectory + "/VPR/first_generation.png", name: "first_generation" }
    case 2116:
      return { directory: actionIconDirectory + "/VPR/second_generation.png", name: "second_generation" }
    case 2117:
      return { directory: actionIconDirectory + "/VPR/third_generation.png", name: "third_generation" }
    case 2118:
      return { directory: actionIconDirectory + "/VPR/fourth_generation.png", name: "fourth_generation" }
    case 2119:
      return { directory: actionIconDirectory + "/VPR/ouroboros.png", name: "ouroboros" }
    case 2120:
      return { directory: actionIconDirectory + "/VPR/reawaken_filler.png", name: "reawaken_filler" }
    case 2121:
      return { directory: actionIconDirectory + "/VPR/reawaken.png", name: "reawaken" }
    case 2122:
      return { directory: actionIconDirectory + "/VPR/uncoiled_fury.png", name: "uncoiled_fury" }
    case 2123:
      return { directory: actionIconDirectory + "/VPR/death_rattle.png", name: "death_rattle" }

    // BRD
    case 1300:
      return { directory: actionIconDirectory + "/BRD/burst_shot.png", name: "burst_shot" }
    case 1301:
      return { directory: actionIconDirectory + "/BRD/refulgent_arrow.png", name: "refulgent_arrow" }
    case 1302:
      return { directory: actionIconDirectory + "/BRD/raging_strikes.png", name: "raging_strikes" }
    case 1303:
      return { directory: actionIconDirectory + "/BRD/heartbreak_shot.png", name: "heartbreak_shot" }
    case 1304:
      return { directory: actionIconDirectory + "/BRD/caustic_bite.png", name: "caustic_bite" }
    case 1305:
      return { directory: actionIconDirectory + "/BRD/stormbite.png", name: "stormbite" }
    case 1306:
      return { directory: actionIconDirectory + "/BRD/apex_arrow.png", name: "apex_arrow" }
    case 1307:
      return { directory: actionIconDirectory + "/BRD/sidewinder.png", name: "sidewinder" }
    case 1308:
      return { directory: actionIconDirectory + "/BRD/iron_jaws.png", name: "iron_jaws" }
    case 1309:
      return { directory: actionIconDirectory + "/BRD/empyreal_arrow.png", name: "empyreal_arrow" }
    case 1310:
      return { directory: actionIconDirectory + "/BRD/pitch_perfect.png", name: "pitch_perfect" }
    case 1311:
      return { directory: actionIconDirectory + "/BRD/battle_voice.png", name: "battle_voice" }
    case 1312:
      return { directory: actionIconDirectory + "/BRD/the_wanderer's_minuet.png", name: "the_wanderer's_minuet" }
    case 1313:
      return { directory: actionIconDirectory + "/BRD/mage's_ballad.png", name: "mage's_ballad" }
    case 1314:
      return { directory: actionIconDirectory + "/BRD/army's_paeon.png", name: "army's_paeon" }
    case 1315:
      return { directory: actionIconDirectory + "/BRD/barrage.png", name: "barrage" }
    case 1316:
      return { directory: actionIconDirectory + "/BRD/blast_arrow.png", name: "blast_arrow" }
    case 1317:
      return { directory: actionIconDirectory + "/BRD/radiant_finale.png", name: "radiant_finale" }
    case 1318:
      return { directory: actionIconDirectory + "/BRD/resonant_arrow.png", name: "resonant_arrow" }
    case 1319:
      return { directory: actionIconDirectory + "/BRD/pitch_perfect.png", name: "pitch_perfect" }
    case 1320:
      return { directory: actionIconDirectory + "/BRD/pitch_perfect.png", name: "pitch_perfect" }
    case 1321:
      return { directory: actionIconDirectory + "/BRD/radiant_encore.png", name: "radiant_encore" }
    case 1322:
      return { directory: actionIconDirectory + "/BRD/radiant_encore.png", name: "radiant_encore" }
    case 1323:
      return { directory: actionIconDirectory + "/BRD/refulgent_arrow_barrage.png", name: "refulgent_arrow_barrage" }
    case 1324:
      return { directory: actionIconDirectory + "/BRD/caustic_bite.png", name: "caustic_bite" }
    case 1325:
      return { directory: actionIconDirectory + "/BRD/stormbite.png", name: "stormbite" }

    // MCH
    case 1400:
      return { directory: actionIconDirectory + "/MCH/heated_split_shot.png", name: "heated_split_shot" }
    case 1401:
      return { directory: actionIconDirectory + "/MCH/heated_slug_shot.png", name: "heated_slug_shot" }
    case 1402:
      return { directory: actionIconDirectory + "/MCH/heated_clean_shot.png", name: "heated_clean_shot" }
    case 1403:
      return { directory: actionIconDirectory + "/MCH/drill.png", name: "drill" }
    case 1404:
      return { directory: actionIconDirectory + "/MCH/air_anchor.png", name: "air_anchor" }
    case 1405:
      return { directory: actionIconDirectory + "/MCH/chain_saw.png", name: "chain_saw" }
    case 1406:
      return { directory: actionIconDirectory + "/MCH/reassemble.png", name: "reassemble" }
    case 1407:
      return { directory: actionIconDirectory + "/MCH/double_check.png", name: "double_check" }
    case 1408:
      return { directory: actionIconDirectory + "/MCH/checkmate.png", name: "checkmate" }
    case 1409:
      return { directory: actionIconDirectory + "/MCH/blazing_shot.png", name: "blazing_shot" }
    case 1410:
      return { directory: actionIconDirectory + "/MCH/wildfire.png", name: "wildfire" }
    case 1411:
      return { directory: actionIconDirectory + "/MCH/hypercharge.png", name: "hypercharge" }
    case 1412:
      return { directory: actionIconDirectory + "/MCH/barrel_stabilizer.png", name: "barrel_stabilizer" }
    case 1413:
      return { directory: actionIconDirectory + "/MCH/automaton_queen.png", name: "automaton_queen" }
    case 1414:
      return { directory: actionIconDirectory + "/MCH/drill.png", name: "drill" }
    case 1415:
      return { directory: actionIconDirectory + "/MCH/air_anchor.png", name: "air_anchor" }
    case 1416:
      return { directory: actionIconDirectory + "/MCH/chain_saw.png", name: "chain_saw" }
    case 1417:
      return { directory: actionIconDirectory + "/MCH/excavator.png", name: "excavator" }
    case 1418:
      return { directory: actionIconDirectory + "/MCH/full_metal_field.png", name: "full_metal_field" }
    case 1419:
      return { directory: actionIconDirectory + "/MCH/excavator.png", name: "excavator" }
    case 1420:
      return { directory: actionIconDirectory + "/MCH/hypercharge.png", name: "hypercharge" }
    case 1421:
      return { directory: actionIconDirectory + "/MCH/reassemble.png", name: "reassemble" }

    // DNC
    case 1500:
      return { directory: actionIconDirectory + "/DNC/cascade.png", name: "cascade" }
    case 1501:
      return { directory: actionIconDirectory + "/DNC/fountain.png", name: "fountain" }
    case 1502:
      return { directory: actionIconDirectory + "/DNC/standard_step.png", name: "standard_step" }
    case 1503:
      return { directory: actionIconDirectory + "/DNC/technical_step.png", name: "technical_step" }
    case 1504:
      return { directory: actionIconDirectory + "/DNC/devilment.png", name: "devilment" }
    case 1505:
      return { directory: actionIconDirectory + "/DNC/flourish.png", name: "flourish" }
    case 1506:
      return { directory: actionIconDirectory + "/DNC/fan_dance.png", name: "fan_dance" }
    case 1507:
      return { directory: actionIconDirectory + "/DNC/fan_dance_III.png", name: "fan_dance_III" }
    case 1508:
      return { directory: actionIconDirectory + "/DNC/fan_dance_IV.png", name: "fan_dance_IV" }
    case 1509:
      return { directory: actionIconDirectory + "/DNC/reverse_cascade.png", name: "reverse_cascade" }
    case 1510:
      return { directory: actionIconDirectory + "/DNC/fountainfall.png", name: "fountainfall" }
    case 1511:
      return { directory: actionIconDirectory + "/DNC/saber_dance.png", name: "saber_dance" }
    case 1512:
      return { directory: actionIconDirectory + "/DNC/starfall_dance.png", name: "starfall_dance" }
    case 1513:
      return { directory: actionIconDirectory + "/DNC/reverse_cascade.png", name: "reverse_cascade" }
    case 1514:
      return { directory: actionIconDirectory + "/DNC/fountainfall.png", name: "fountainfall" }
    case 1515:
      return { directory: actionIconDirectory + "/DNC/tillana.png", name: "tillana" }
    case 1516:
      return { directory: actionIconDirectory + "/DNC/last_dance.png", name: "last_dance" }
    case 1517:
      return { directory: actionIconDirectory + "/DNC/finishing_move.png", name: "finishing_move" }
    case 1518:
      return { directory: actionIconDirectory + "/DNC/dance_of_the_dawn.png", name: "dance_of_the_dawn" }
    case 1519:
      return { directory: actionIconDirectory + "/DNC/standard_step.png", name: "standard_step" }

    // SMN
    case 1600:
      return { directory: actionIconDirectory + "/SMN/ruin_III.png", name: "ruin_III" }
    case 1601:
      return { directory: actionIconDirectory + "/SMN/ruin_IV.png", name: "ruin_IV" }
    case 1602:
      return { directory: actionIconDirectory + "/SMN/searing_light.png", name: "searing_light" }
    case 1603:
      return { directory: actionIconDirectory + "/SMN/energy_drain.png", name: "energy_drain" }
    case 1604:
      return { directory: actionIconDirectory + "/SMN/necrotize.png", name: "necrotize" }
    case 1605:
      return { directory: actionIconDirectory + "/SMN/summon_bahamut.png", name: "summon_bahamut" }
    case 1606:
      return { directory: actionIconDirectory + "/SMN/enkindle_bahamut.png", name: "enkindle_bahamut" }
    case 1607:
      return { directory: actionIconDirectory + "/SMN/deathflare.png", name: "deathflare" }
    case 1608:
      return { directory: actionIconDirectory + "/SMN/astral_impulse.png", name: "astral_impulse" }
    case 1609:
      return { directory: actionIconDirectory + "/SMN/wyrmwave.png", name: "wyrmwave" }
    case 1610:
      return { directory: actionIconDirectory + "/SMN/summon_ifrit_II.png", name: "summon_ifrit_II" }
    case 1611:
      return { directory: actionIconDirectory + "/SMN/crimson_cyclone.png", name: "crimson_cyclone" }
    case 1612:
      return { directory: actionIconDirectory + "/SMN/crimson_strike.png", name: "crimson_strike" }
    case 1613:
      return { directory: actionIconDirectory + "/SMN/ruby_rite.png", name: "ruby_rite" }
    case 1614:
      return { directory: actionIconDirectory + "/SMN/summon_titan_II.png", name: "summon_titan_II" }
    case 1615:
      return { directory: actionIconDirectory + "/SMN/topaz_rite.png", name: "topaz_rite" }
    case 1616:
      return { directory: actionIconDirectory + "/SMN/mountain_buster.png", name: "mountain_buster" }
    case 1617:
      return { directory: actionIconDirectory + "/SMN/summon_garuda_II.png", name: "summon_garuda_II" }
    case 1618:
      return { directory: actionIconDirectory + "/SMN/emerald_rite.png", name: "emerald_rite" }
    case 1619:
      return { directory: actionIconDirectory + "/SMN/slipstream.png", name: "slipstream" }
    case 1620:
      return { directory: actionIconDirectory + "/SMN/summon_phoenix.png", name: "summon_phoenix" }
    case 1621:
      return { directory: actionIconDirectory + "/SMN/fountain_of_fire.png", name: "fountain_of_fire" }
    case 1622:
      return { directory: actionIconDirectory + "/SMN/enkindle_phoenix.png", name: "enkindle_phoenix" }
    case 1623:
      return { directory: actionIconDirectory + "/SMN/scarlet_flame.png", name: "scarlet_flame" }
    case 1625:
      return { directory: actionIconDirectory + "/SMN/searing_flash.png", name: "searing_flash" }
    case 1626:
      return { directory: actionIconDirectory + "/SMN/summon_solar_bahamut.png", name: "summon_solar_bahamut" }
    case 1627:
      return { directory: actionIconDirectory + "/SMN/umbral_impulse.png", name: "umbral_impulse" }
    case 1628:
      return { directory: actionIconDirectory + "/SMN/sunflare.png", name: "sunflare" }
    case 1629:
      return { directory: actionIconDirectory + "/SMN/enkindle_solar_bahamut.png", name: "enkindle_solar_bahamut" }
    case 1630:
      return { directory: actionIconDirectory + "/SMN/luxwave.png", name: "luxwave" }
    case 1631:
      return { directory: actionIconDirectory + "/SMN/slipstream.png", name: "slipstream" }

    // BLM
    case 1700:
      return { directory: actionIconDirectory + "/BLM/transpose.png", name: "transpose" }
    case 1701:
      return { directory: actionIconDirectory + "/BLM/high_thunder.png", name: "high_thunder" }
    case 1702:
      return { directory: actionIconDirectory + "/BLM/transpose.png", name: "transpose" }
    case 1703:
      return { directory: actionIconDirectory + "/BLM/fire_IV.png", name: "fire_IV" }
    case 1704:
      return { directory: actionIconDirectory + "/BLM/fire_IV.png", name: "fire_IV" }
    case 1705:
      return { directory: actionIconDirectory + "/BLM/fire_III.png", name: "fire_III" }
    case 1706:
      return { directory: actionIconDirectory + "/BLM/fire_III.png", name: "fire_III" }
    case 1707:
      return { directory: actionIconDirectory + "/BLM/despair.png", name: "despair" }
    case 1708:
      return { directory: actionIconDirectory + "/BLM/despair.png", name: "despair" }
    case 1709:
      return { directory: actionIconDirectory + "/BLM/despair.png", name: "despair" }
    case 1710:
      return { directory: actionIconDirectory + "/BLM/xenoglossy.png", name: "xenoglossy" }
    case 1711:
      return { directory: actionIconDirectory + "/BLM/paradox.png", name: "paradox" }
    case 1712:
      return { directory: actionIconDirectory + "/BLM/blizzard_III.png", name: "blizzard_III" }
    case 1713:
      return { directory: actionIconDirectory + "/BLM/blizzard_IV.png", name: "blizzard_IV" }
    case 1714:
      return { directory: actionIconDirectory + "/BLM/triplecast.png", name: "triplecast" }
    case 1715:
      return { directory: actionIconDirectory + "/BLM/ley_lines.png", name: "ley_lines" }
    case 1716:
      return { directory: actionIconDirectory + "/BLM/manafont.png", name: "manafont" }
    case 1717:
      return { directory: actionIconDirectory + "/BLM/amplifier.png", name: "amplifier" }
    case 1718:
      return { directory: actionIconDirectory + "/BLM/fire_III.png", name: "fire_III" }
    case 1719:
      return { directory: actionIconDirectory + "/BLM/fire_IV.png", name: "fire_IV" }
    case 1720:
      return { directory: actionIconDirectory + "/BLM/flare_star.png", name: "flare_star" }
    case 1721:
      return { directory: actionIconDirectory + "/BLM/blizzard_III.png", name: "blizzard_III" }
    case 1722:
      return { directory: actionIconDirectory + "/BLM/flare_star.png", name: "flare_star" }
    case 1723:
      return { directory: actionIconDirectory + "/BLM/blizzard_III.png", name: "blizzard_III" }
    case 1724:
      return { directory: actionIconDirectory + "/BLM/fire_III.png", name: "fire_III" }
    case 1725:
      return { directory: actionIconDirectory + "/BLM/high_thunder.png", name: "high_thunder" }

    // RDM
    case 1800:
      return { directory: actionIconDirectory + "/RDM/jolt_III.png", name: "jolt_III" }
    case 1801:
      return { directory: actionIconDirectory + "/RDM/veraero_III.png", name: "veraero_III" }
    case 1802:
      return { directory: actionIconDirectory + "/RDM/veraero_III.png", name: "veraero_III" }
    case 1803:
      return { directory: actionIconDirectory + "/RDM/veraero_III.png", name: "veraero_III" }
    case 1804:
      return { directory: actionIconDirectory + "/RDM/veraero_III.png", name: "veraero_III" }
    case 1805:
      return { directory: actionIconDirectory + "/RDM/verthunder_III.png", name: "verthunder_III" }
    case 1806:
      return { directory: actionIconDirectory + "/RDM/verthunder_III.png", name: "verthunder_III" }
    case 1807:
      return { directory: actionIconDirectory + "/RDM/verthunder_III.png", name: "verthunder_III" }
    case 1808:
      return { directory: actionIconDirectory + "/RDM/verstone.png", name: "verstone" }
    case 1809:
      return { directory: actionIconDirectory + "/RDM/verfire.png", name: "verfire" }
    case 1810:
      return { directory: actionIconDirectory + "/RDM/fleche.png", name: "fleche" }
    case 1811:
      return { directory: actionIconDirectory + "/RDM/contre_sixte.png", name: "contre_sixte" }
    case 1812:
      return { directory: actionIconDirectory + "/RDM/corps-a-corps.png", name: "corps-a-corps" }
    case 1813:
      return { directory: actionIconDirectory + "/RDM/engagement.png", name: "engagement" }
    case 1814:
      return { directory: actionIconDirectory + "/RDM/embolden.png", name: "embolden" }
    case 1815:
      return { directory: actionIconDirectory + "/RDM/enchanted_riposte.png", name: "enchanted_riposte" }
    case 1816:
      return { directory: actionIconDirectory + "/RDM/enchanted_zwerchhau.png", name: "enchanted_zwerchhau" }
    case 1817:
      return { directory: actionIconDirectory + "/RDM/enchanted_redoublement.png", name: "enchanted_redoublement" }
    case 1818:
      return { directory: actionIconDirectory + "/RDM/verholy.png", name: "verholy" }
    case 1819:
      return { directory: actionIconDirectory + "/RDM/verflare.png", name: "verflare" }
    case 1820:
      return { directory: actionIconDirectory + "/RDM/acceleration.png", name: "acceleration" }
    case 1821:
      return { directory: actionIconDirectory + "/RDM/manafication.png", name: "manafication" }
    case 1822:
      return { directory: actionIconDirectory + "/RDM/scorch.png", name: "scorch" }
    case 1823:
      return { directory: actionIconDirectory + "/RDM/resolution.png", name: "resolution" }
    case 1824:
      return { directory: actionIconDirectory + "/RDM/vice_of_thorns.png", name: "vice_of_thorns" }
    case 1825:
      return { directory: actionIconDirectory + "/RDM/grand_impact.png", name: "grand_impact" }
    case 1826:
      return { directory: actionIconDirectory + "/RDM/prefulgence.png", name: "prefulgence" }
    case 1827:
      return { directory: actionIconDirectory + "/RDM/enchanted_riposte.png", name: "enchanted_riposte" }
    case 1828:
      return { directory: actionIconDirectory + "/RDM/enchanted_zwerchhau.png", name: "enchanted_zwerchhau" }
    case 1829:
      return { directory: actionIconDirectory + "/RDM/enchanted_redoublement.png", name: "enchanted_redoublement" }
    case 1830:
      return { directory: actionIconDirectory + "/RDM/verflare.png", name: "verflare" }
    case 1831:
      return { directory: actionIconDirectory + "/RDM/verholy.png", name: "verholy" }
    case 1832:
      return { directory: actionIconDirectory + "/RDM/scorch.png", name: "scorch" }
    case 1833:
      return { directory: actionIconDirectory + "/RDM/resolution.png", name: "resolution" }

    // PCT
    case 2000:
      return { directory: actionIconDirectory + "/PCT/fire_in_red.png", name: "fire_in_red" }
    case 2001:
      return { directory: actionIconDirectory + "/PCT/aero_in_green.png", name: "aero_in_green" }
    case 2002:
      return { directory: actionIconDirectory + "/PCT/water_in_blue.png", name: "water_in_blue" }
    case 2003:
      return { directory: actionIconDirectory + "/PCT/creature_muse.png", name: "creature_muse" }
    case 2005:
      return { directory: actionIconDirectory + "/PCT/mog_of_the_ages.png", name: "mog_of_the_ages" }
    case 2006:
      return { directory: actionIconDirectory + "/PCT/pom_motif.png", name: "pom_motif" }
    case 2007:
      return { directory: actionIconDirectory + "/PCT/pom_muse.png", name: "pom_muse" }
    case 2008:
      return { directory: actionIconDirectory + "/PCT/wing_motif.png", name: "wing_motif" }
    case 2009:
      return { directory: actionIconDirectory + "/PCT/winged_muse.png", name: "winged_muse" }
    case 2010:
      return { directory: actionIconDirectory + "/PCT/hammer_motif.png", name: "hammer_motif" }
    case 2011:
      return { directory: actionIconDirectory + "/PCT/subtractive_pallete.png", name: "subtractive_pallete" }
    case 2012:
      return { directory: actionIconDirectory + "/PCT/striking_muse.png", name: "striking_muse" }
    case 2013:
      return { directory: actionIconDirectory + "/PCT/blizzard_in_cyan.png", name: "blizzard_in_cyan" }
    case 2014:
      return { directory: actionIconDirectory + "/PCT/stone_in_yellow.png", name: "stone_in_yellow" }
    case 2015:
      return { directory: actionIconDirectory + "/PCT/thunder_in_magenta.png", name: "thunder_in_magenta" }
    case 2016:
      return { directory: actionIconDirectory + "/PCT/starry_sky_motif.png", name: "starry_sky_motif" }
    case 2017:
      return { directory: actionIconDirectory + "/PCT/starry_muse.png", name: "starry_muse" }
    case 2018:
      return { directory: actionIconDirectory + "/PCT/holy_in_white.png", name: "holy_in_white" }
    case 2019:
      return { directory: actionIconDirectory + "/PCT/hammer_stamp.png", name: "hammer_stamp" }
    case 2020:
      return { directory: actionIconDirectory + "/PCT/hammer_brush.png", name: "hammer_brush" }
    case 2021:
      return { directory: actionIconDirectory + "/PCT/polishing_hammer.png", name: "polishing_hammer" }
    case 2022:
      return { directory: actionIconDirectory + "/PCT/comet_in_black.png", name: "comet_in_black" }
    case 2023:
      return { directory: actionIconDirectory + "/PCT/rainbow_drip.png", name: "rainbow_drip" }
    case 2024:
      return { directory: actionIconDirectory + "/PCT/rainbow_drip.png", name: "rainbow_drip" }
    case 2025:
      return { directory: actionIconDirectory + "/PCT/claw_motif.png", name: "claw_motif" }
    case 2026:
      return { directory: actionIconDirectory + "/PCT/clawed_muse.png", name: "clawed_muse" }
    case 2027:
      return { directory: actionIconDirectory + "/PCT/maw_motif.png", name: "maw_motif" }
    case 2028:
      return { directory: actionIconDirectory + "/PCT/fanged_muse.png", name: "fanged_muse" }
    case 2029:
      return { directory: actionIconDirectory + "/PCT/retribution_of_the_madeem.png", name: "retribution_of_the_madeem" }
    case 2030:
      return { directory: actionIconDirectory + "/PCT/star_prism.png", name: "star_prism" }
    case 2031:
      return { directory: actionIconDirectory + "/PCT/subtractive_pallete.png", name: "subtractive_pallete" }
    case 2032:
      return { directory: actionIconDirectory + "/PCT/comet_in_black.png", name: "comet_in_black" }
    case 2033:
      return { directory: actionIconDirectory + "/PCT/blizzard_in_cyan.png", name: "blizzard_in_cyan" }
    case 2034:
      return { directory: actionIconDirectory + "/PCT/stone_in_yellow.png", name: "stone_in_yellow" }
    case 2035:
      return { directory: actionIconDirectory + "/PCT/thunder_in_magenta.png", name: "thunder_in_magenta" }

    case 0:
      return { directory: actionIconDirectory + "/BLM/MagicalRangedRollAction/swiftcast.png", name: "swiftcast" }
    case 1:
      return { directory: actionIconDirectory + "/pot.png", name: "pot" }
    case 10001:
      return { directory: actionIconDirectory + "/auto-attack.png", name: "auto-attack" }
    default:
      return { directory: "", name: `unknown skill id: ${skillId}` };
  }
};

export const skillIdToIcon = (statusId: number) => {
  const { directory: iconPath, name: name } = SkillIdToIconPath(statusId);
  const actionIconFullPath = ACTION_ICONS[iconPath] as { default: string } | undefined;
  const refinedSkillName = name.replace(/_/g, " ");

  if (!actionIconFullPath) {
    return { icon: "unknown", name: refinedSkillName };
  }

  return { icon: actionIconFullPath.default, name: refinedSkillName };
}
