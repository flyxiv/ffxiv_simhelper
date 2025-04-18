export const ACTION_ICONS = import.meta.glob('/src/assets/images/actions/**/*', { eager: true });

// Only support raid/personal buff/debuff icons for now
const StatusIdToIconPath = (statusId: number) => {
  const actionIconDirectory = `/src/assets/images/actions`;
  switch (statusId) {
    // PLD
    case 1900:
      return actionIconDirectory + "/PLD/imperator.png";
    case 1901:
      return actionIconDirectory + "/PLD/fight_or_flight.png";

    // DRK
    case 201:
      return actionIconDirectory + "/DRK/delirium.png";

    // WAR
    case 100:
      return actionIconDirectory + "/WAR/storm's_eye.png";

    // GNB
    case 300:
      return actionIconDirectory + "/GNB/no_mercy.png";

    // WHM
    case 401:
      return actionIconDirectory + "/WHM/presence_of_mind.png";

    // AST
    case 500:
      return actionIconDirectory + "/AST/divination.png";
    case 501:
      return actionIconDirectory + "/AST/the_balance.png";
    case 502:
      return actionIconDirectory + "/AST/the_spear.png";

    // SCH
    case 601:
      return actionIconDirectory + "/SCH/chain_stratagem.png";

    // DRG
    case 800:
      return actionIconDirectory + "/DRG/life_surge.png";
    case 804:
      return actionIconDirectory + "/DRG/lance_charge.png";
    case 807:
      return actionIconDirectory + "/DRG/battle_litany.png";

    // MNK
    case 900:
      return actionIconDirectory + "/MNK/bootshine.png";
    case 904:
      return actionIconDirectory + "/MNK/riddle_of_fire.png";
    case 905:
      return actionIconDirectory + "/MNK/riddle_of_wind.png";
    case 906:
      return actionIconDirectory + "/MNK/brotherhood.png";

    // NIN
    case 1003:
      return actionIconDirectory + "/NIN/dokumori.png";
    case 1004:
      return actionIconDirectory + "/NIN/kunai's_bane.png";
    case 1005:
      return actionIconDirectory + "/NIN/kassatsu.png";

    // RPR
    case 1200:
      return actionIconDirectory + "/RPR/enshroud.png";
    case 1203:
      return actionIconDirectory + "/RPR/arcane_circle.png";

    // BRD
    case 1302:
      return actionIconDirectory + "/BRD/the_wanderer's_minuet.png";
    case 1304:
      return actionIconDirectory + "/BRD/mage's_ballad.png";
    case 1306:
      return actionIconDirectory + "/BRD/army's_paeon.png";
    case 1308:
      return actionIconDirectory + "/BRD/barrage.png";
    case 1309:
      return actionIconDirectory + "/BRD/radiant_finale.png";
    case 1312:
      return actionIconDirectory + "/BRD/battle_voice.png";
    case 1314:
      return actionIconDirectory + "/BRD/raging_strikes.png";

    // DNC
    case 1500:
      return actionIconDirectory + "/DNC/standard_finish.png";
    case 1502:
      return actionIconDirectory + "/DNC/technical_finish.png";
    case 1504:
      return actionIconDirectory + "/DNC/devilment.png";

    // SMN
    case 1601:
      return actionIconDirectory + "/SMN/searing_light.png";

    // BLM
    case 1700:
      return actionIconDirectory + "/BLM/triplecast.png";
    case 1703:
      return actionIconDirectory + "/BLM/ley_lines.png";

    // RDM
    case 1800:
      return actionIconDirectory + "/RDM/embolden.png";
    case 1801:
      return actionIconDirectory + "/RDM/manafication.png";
    case 1802:
      return actionIconDirectory + "/RDM/acceleration.png";
    case 1803:
      return actionIconDirectory + "/RDM/dualcast.png";

    // PCT
    case 2000:
      return actionIconDirectory + "/PCT/starry_muse.png";
    case 2002:
      return actionIconDirectory + "/PCT/hyperphantasia.png";

    case 0:
      return actionIconDirectory + "/BLM/MagicalRangedRollAction/swiftcast.png";
    case 1:
      return actionIconDirectory + "/pot.png";

    default:
      return `unknown status id: ${statusId}`;
  }
};

export const StatusIdToIcon = (statusId: number) => {
  const iconPath = StatusIdToIconPath(statusId);
  const actionIconFullPath = ACTION_ICONS[iconPath] as { default: string } | undefined;

  if (!actionIconFullPath) {
    return "unknown";
  }

  return actionIconFullPath.default;
}