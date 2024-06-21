export const StatusIdToIconPathFactory = (statusId: number) => {
  const actionIconDirectory = process.env.PUBLIC_URL + "/images/actions";
  switch (statusId) {
    // AST
    case 500:
      return actionIconDirectory + "/AST/divination.png";
    case 501:
      return actionIconDirectory + "/AST/003110.png";
    case 502:
      return actionIconDirectory + "/AST/003115.png";

    // SCH
    case 601:
      return actionIconDirectory + "/SCH/chain_stratagem.png";

    // DRG
    case 806:
      return actionIconDirectory + "/DRG/dragon_sight.png";
    case 807:
      return actionIconDirectory + "/DRG/battle_litany.png";

    // MNK
    case 906:
      return actionIconDirectory + "/MNK/brotherhood.png";

    // NIN
    case 1003:
      return actionIconDirectory + "/NIN/mug.png";

    // RPR
    case 1203:
      return actionIconDirectory + "/RPR/arcane_circle.png";

    // BRD
    case 1302:
      return actionIconDirectory + "/BRD/the_wanderer's_minuet.png";
    case 1304:
      return actionIconDirectory + "/BRD/mage's_ballad.png";
    case 1306:
      return actionIconDirectory + "/BRD/army's_paeon.png";
    case 1309:
      return actionIconDirectory + "/BRD/radiant_finale.png";
    case 1312:
      return actionIconDirectory + "/BRD/battle_voice.png";

    // DNC
    case 1500:
      return actionIconDirectory + "/DNC/standard_finish.png";
    case 1502:
      return actionIconDirectory + "/DNC/techical_finish.png";
    case 1504:
      return actionIconDirectory + "/DNC/devilment.png";

    // SMN
    case 1601:
      return actionIconDirectory + "/SMN/searing_light.png";

    // RDM
    case 1800:
      return actionIconDirectory + "/RDM/embolden.png";

    case 0:
      return actionIconDirectory + "/BLM/MagicalRangedRollAction/swiftcast.png";
    case 10001:
      return actionIconDirectory + "/auto-attack.png";
    default:
      return `unknown skill id: ${statusId}`;
  }
};
