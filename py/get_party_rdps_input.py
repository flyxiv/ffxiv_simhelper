import requests
import json

"""Iterate over all possible party compositions and get the RDPS for each one.
    weaponDamage: 0,
    mainStat: 0,
    criticalStrike: DEFAULT_CRITICAL_STRIKE,
    directHit: DEFAULT_DIRECT_HIT,
    determination: DEFAULT_DETERMINATION,
    skillSpeed: DEFAULT_SPEED,
    spellSpeed: DEFAULT_SPEED,
    tenacity: DEFAULT_TENACITY,
    piety: 0,
    weaponDamageMultiplier: 0,
    mainStatMultiplier: 0,
    criticalStrikeRate: 0,
    criticalStrikeDamage: 0,
    directHitRate: 0,
    determinationMultiplier: 0,
    speedMultiplier: 0,
    tenacityMultiplier: 0,
    autoDirectHitIncrease: 0,
    autoAttackDelays: 0,
    gcd: DEFAULT_GCD,
"""

class PlayerPower:
    def __init__(self, weapon_damage_multiplier=0, main_stat_multiplier=0, critical_strike_rate=0, critical_strike_damage=0, direct_hit_rate=0, determination_multiplier=0, speed_multiplier=0, tenacity_multiplier=0, auto_direct_hit_increase=0, auto_attack_delays=0):
        self.weapon_damage = 0 
        self.main_stat = 0 
        self.critical_strike =0  
        self.direct_hit = 0 
        self.determination = 0 
        self.skill_speed = 0 
        self.spell_speed = 0 
        self.tenacity = 0 
        self.piety = 0  
        self.weapon_damage_multiplier = weapon_damage_multiplier
        self.main_stat_multiplier = main_stat_multiplier
        self.critical_strike_rate = critical_strike_rate
        self.critical_strike_damage = critical_strike_damage
        self.direct_hit_rate = direct_hit_rate
        self.determination_multiplier = determination_multiplier
        self.speed_multiplier = speed_multiplier
        self.tenacity_multiplier = tenacity_multiplier
        self.auto_direct_hit_increase = auto_direct_hit_increase
        self.auto_attack_delays = auto_attack_delays
        self.gcd = 0 

    def __str__(self):
        return f'{self.player}, {self.power}, {self.rdps}'

    def __repr__(self):
        return f'{self.player}, {self.power}, {self.rdps}'

    def __dict__(self):
        return {
            'weaponDamage': self.weapon_damage,
            'mainStat': self.main_stat,
            'criticalStrike': self.critical_strike,
            'directHit': self.direct_hit,
            'determination': self.determination,
            'skillSpeed': self.skill_speed,
            'spellSpeed': self.spell_speed,
            'tenacity': self.tenacity,
            'piety': self.piety,
            'weaponDamageMultiplier': self.weapon_damage_multiplier,
            'mainStatMultiplier': self.main_stat_multiplier,
            'criticalStrikeRate': self.critical_strike_rate,
            'criticalStrikeDamage': self.critical_strike_damage,
            'directHitRate': self.direct_hit_rate,
            'determinationMultiplier': self.determination_multiplier,
            'speedMultiplier': self.speed_multiplier,
            'tenacityMultiplier': self.tenacity_multiplier,
            'autoDirectHitIncrease': self.auto_direct_hit_increase,
            'autoAttackDelays': self.auto_attack_delays,
            'gcd': self.gcd
        }


PLD_BIS = PlayerPower(weapon_damage_multiplier=1.90, main_stat_multiplier=20.94, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.207, determination_multiplier=1.094, speed_multiplier=1, tenacity_multiplier=1.018, auto_direct_hit_increase=0.052, auto_attack_delays=2.24)
WAR_BIS = PlayerPower(weapon_damage_multiplier=1.92, main_stat_multiplier=20.94, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.207, determination_multiplier=1.094, speed_multiplier=1, tenacity_multiplier=1.018, auto_direct_hit_increase=0.052, auto_attack_delays=3.36)
DRK_BIS = PlayerPower(weapon_damage_multiplier=1.92, main_stat_multiplier=21.04, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.207, determination_multiplier=1.094, speed_multiplier=1, tenacity_multiplier=1.018, auto_direct_hit_increase=0.052, auto_attack_delays=2.96)
GNB_BIS = PlayerPower(weapon_damage_multiplier=1.90, main_stat_multiplier=20.94, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.207, determination_multiplier=1.094, speed_multiplier=1, tenacity_multiplier=1.018, auto_direct_hit_increase=0.052, auto_attack_delays=2.8)

AST_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.03, critical_strike_rate=0.230, critical_strike_damage=1.580, direct_hit_rate=0.146, determination_multiplier=1.131, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.037, auto_attack_delays=0)
WHM_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.03, critical_strike_rate=0.230, critical_strike_damage=1.580, direct_hit_rate=0.146, determination_multiplier=1.131, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.037, auto_attack_delays=0)
SCH_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.03, critical_strike_rate=0.230, critical_strike_damage=1.580, direct_hit_rate=0.146, determination_multiplier=1.131, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.037, auto_attack_delays=0)
SGE_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.03, critical_strike_rate=0.230, critical_strike_damage=1.580, direct_hit_rate=0.146, determination_multiplier=1.131, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.037, auto_attack_delays=0)

SAM_BIS = PlayerPower(weapon_damage_multiplier=1.95, main_stat_multiplier=26.17, critical_strike_rate=0.243, critical_strike_damage=1.593, direct_hit_rate=0.299, determination_multiplier=1.083, speed_multiplier=1.012, tenacity_multiplier=1, auto_direct_hit_increase=0.076, auto_attack_delays=2.64)
DRG_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.25, critical_strike_rate=0.244, critical_strike_damage=1.594, direct_hit_rate=0.338, determination_multiplier=1.086, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.086, auto_attack_delays=2.8)
MNK_BIS = PlayerPower(weapon_damage_multiplier=1.94, main_stat_multiplier=26.13, critical_strike_rate=0.246, critical_strike_damage=1.596, direct_hit_rate=0.241, determination_multiplier=1.082, speed_multiplier=1.025, tenacity_multiplier=1, auto_direct_hit_increase=0.061, auto_attack_delays=2.56)
NIN_BIS = PlayerPower(weapon_damage_multiplier=1.94, main_stat_multiplier=26.11, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.281, determination_multiplier=1.098, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.071, auto_attack_delays=2.56)
VPR_BIS = PlayerPower(weapon_damage_multiplier=1.94, main_stat_multiplier=25.86, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.259, determination_multiplier=1.098, speed_multiplier=1.005, tenacity_multiplier=1, auto_direct_hit_increase=0.066, auto_attack_delays=2.64)
RPR_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.25, critical_strike_rate=0.244, critical_strike_damage=1.594, direct_hit_rate=0.328, determination_multiplier=1.086, speed_multiplier=1.002, tenacity_multiplier=1, auto_direct_hit_increase=0.083, auto_attack_delays=3.2)


DNC_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.24, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.339, determination_multiplier=1.083, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.086, auto_attack_delays=3.12)
BRD_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.24, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.339, determination_multiplier=1.083, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.086, auto_attack_delays=3.04)
MCH_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.24, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.339, determination_multiplier=1.083, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.086, auto_attack_delays=2.64)


PCT_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.245, critical_strike_damage=1.595, direct_hit_rate=0.311, determination_multiplier=1.092, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.079, auto_attack_delays=0)
RDM_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.245, critical_strike_damage=1.595, direct_hit_rate=0.311, determination_multiplier=1.092, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.079, auto_attack_delays=0)
SMN_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.240, critical_strike_damage=1.590, direct_hit_rate=0.337, determination_multiplier=1.084, speed_multiplier=1.005, tenacity_multiplier=1, auto_direct_hit_increase=0.085, auto_attack_delays=0)
BLM_BIS = PlayerPower(weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.249, critical_strike_damage=1.599, direct_hit_rate=0.246, determination_multiplier=1.056, speed_multiplier=1.037, tenacity_multiplier=1, auto_direct_hit_increase=0.062, auto_attack_delays=0)


PARTY_BOARD = {
    "PLD": PLD_BIS,
    "WAR": WAR_BIS,
    "DRK": DRK_BIS,
    "GNB": GNB_BIS,
    "AST": AST_BIS,
    "WHM": WHM_BIS,
    "SCH": SCH_BIS,
    "SGE": SGE_BIS,
    "SAM": SAM_BIS,
    "DRG": DRG_BIS,
    "MNK": MNK_BIS,
    "NIN": NIN_BIS,
    "VPR": VPR_BIS,
    "RPR": RPR_BIS,
    "DNC": DNC_BIS,
    "BRD": BRD_BIS,
    "MCH": MCH_BIS,
    "PCT": PCT_BIS,
    "RDM": RDM_BIS,
    "SMN": SMN_BIS,
    "BLM": BLM_BIS
}



TANK_SPECS = [
    "PLD",
    "WAR",
    "DRK",
    "GNB"
]

HEALER_SPECS = [
    "AST",
    "WHM",
    "SCH",
    "SGE"
]

MELEE_SPECS = [ 
    "NIN",
    "MNK",
    "DRG",
    "SAM",
    "VPR",
    "RPR"
]

RANGED_SPECS = [
    "DNC",
    "BRD",
    "MCH"
]

CASTER_SPECS = [
    "PCT",
    "RDM",
    "SMN",
    "BLM"
]

ALL_DPS = MELEE_SPECS + RANGED_SPECS + CASTER_SPECS

def create_party_info(job_abbrevs):
    party_infos = []
    for (player_id, job_abbrev) in enumerate(job_abbrevs):
        party_info = {} 
        party_info["playerId"] = player_id 
        party_info['jobAbbrev'] = job_abbrev
        party_info['power'] = PARTY_BOARD[job_abbrev].__dict__()
        party_info['partner1Id'] = None
        party_info['partner2Id'] = None

        party_infos.append(party_info)

    return json.dumps({
        "mainPlayerId": 0,
        "combatTimeMillisecond": 0,
        "party": party_infos,
        "partyIlvlAdjustment": 1.0,
        "usePot": True 
    } )

def run():
    table = {"key": [], "total dps": []}
    count = 0
    for tank in TANK_SPECS:
        for tank2 in TANK_SPECS:
            for heal1 in HEALER_SPECS:
                for heal2 in HEALER_SPECS:
                    if heal1 == heal2 and heal1 in ["AST", "SCH"]:
                        continue

                    for melee in MELEE_SPECS:
                        for ranged in RANGED_SPECS:
                            for caster in CASTER_SPECS:
                                for other in ALL_DPS:
                                    if other in ["PCT", "RDM", "SMN", "MNK", "NIN", "DRG", "RPR","BRD", "DNC"] and (other == caster or other == ranged or other == melee):
                                        continue
                                    key = "{}, {}, {}, {}, {}, {}, {}, {}".format(tank, tank2, heal1, heal2, melee, ranged, caster, other)
                                    print(key)
                                    if key in table:
                                        continue
                                    party_info = create_party_info([tank, tank2, heal1, heal2, melee, ranged, caster, other])
                                    rdps = []

                                    for _ in range(1000):
                                        response = requests.post('http://localhost:13406/api/v1/simulate', json=party_info)
                                        print(response)
                                        print(response.reason)
                                        rdpsofrun1=0
                                        rdpsofrun2=0
                                        for simulation_data in response['simulationData']:
                                            rdpsofrun1 += int((simulation_data['simulationSummary']['pdps'][0]))
                                            rdpsofrun2 += int((simulation_data['simulationSummary']['pdps'][1]))
                                        rdps.extend([rdpsofrun1, rdpsofrun2])

                                    rdps.sort()
                                    print(rdps[500])
                                    table[key] = rdps[500]
                                
                                    if count % 100 == 0:
                                        with open('party_rdps_table.json', 'w') as f:
                                            json.dump(table, f)


if __name__ == '__main__':
    run()