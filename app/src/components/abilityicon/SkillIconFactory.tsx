export const SkillIdToIconPathFactory = (skillId: number) => {
  const actionIconDirectory = process.env.PUBLIC_URL + "/images/actions";
  switch (skillId) {
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
    case 10001:
      return actionIconDirectory + "/auto-attack.png";
    default:
      return `unknown skill id: ${skillId}`;
  }
};
