import requests
import json
import asyncio
import aiohttp
import tqdm

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
    def __init__(self, main_stat=0, weapon_damage_multiplier=0, main_stat_multiplier=0, critical_strike_rate=0, critical_strike_damage=0, direct_hit_rate=0, determination_multiplier=0, speed_multiplier=0, tenacity_multiplier=0, auto_direct_hit_increase=0, auto_attack_delays=0):
        self.weapon_damage = 140 
        self.main_stat = main_stat
        self.critical_strike = 3033 
        self.direct_hit = 2002 
        self.determination = 2000 
        self.skill_speed = 420 
        self.spell_speed = 420 
        self.tenacity = 420 
        self.piety = 420 
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
        self.gcd = 1.0 

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


PLD_BIS = PlayerPower(main_stat=5056, weapon_damage_multiplier=1.90, main_stat_multiplier=20.96, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.207, determination_multiplier=1.094, speed_multiplier=1, tenacity_multiplier=1.018, auto_direct_hit_increase=0.052, auto_attack_delays=2.24)
WAR_BIS = PlayerPower(main_stat=5084, weapon_damage_multiplier=1.92, main_stat_multiplier=21.05, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.207, determination_multiplier=1.094, speed_multiplier=1, tenacity_multiplier=1.018, auto_direct_hit_increase=0.052, auto_attack_delays=3.36)
DRK_BIS = PlayerPower(main_stat=5084, weapon_damage_multiplier=1.92, main_stat_multiplier=21.05, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.207, determination_multiplier=1.094, speed_multiplier=1, tenacity_multiplier=1.018, auto_direct_hit_increase=0.052, auto_attack_delays=2.96)
GNB_BIS = PlayerPower(main_stat=5061, weapon_damage_multiplier=1.90, main_stat_multiplier=20.95, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.207, determination_multiplier=1.094, speed_multiplier=1, tenacity_multiplier=1.018, auto_direct_hit_increase=0.052, auto_attack_delays=2.8)

AST_BIS = PlayerPower(main_stat=5091, weapon_damage_multiplier=1.96, main_stat_multiplier=26.05, critical_strike_rate=0.230, critical_strike_damage=1.580, direct_hit_rate=0.146, determination_multiplier=1.131, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.037, auto_attack_delays=0)
WHM_BIS = PlayerPower(main_stat=5091, weapon_damage_multiplier=1.96, main_stat_multiplier=26.05, critical_strike_rate=0.230, critical_strike_damage=1.580, direct_hit_rate=0.146, determination_multiplier=1.131, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.037, auto_attack_delays=0)
SCH_BIS = PlayerPower(main_stat=5091, weapon_damage_multiplier=1.96, main_stat_multiplier=26.05, critical_strike_rate=0.230, critical_strike_damage=1.580, direct_hit_rate=0.146, determination_multiplier=1.131, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.037, auto_attack_delays=0)
SGE_BIS = PlayerPower(main_stat=5091, weapon_damage_multiplier=1.96, main_stat_multiplier=26.05, critical_strike_rate=0.230, critical_strike_damage=1.580, direct_hit_rate=0.146, determination_multiplier=1.131, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.037, auto_attack_delays=0)

SAM_BIS = PlayerPower(main_stat=5115, weapon_damage_multiplier=1.95, main_stat_multiplier=26.18, critical_strike_rate=0.243, critical_strike_damage=1.593, direct_hit_rate=0.299, determination_multiplier=1.083, speed_multiplier=1.012, tenacity_multiplier=1, auto_direct_hit_increase=0.076, auto_attack_delays=2.64)
DRG_BIS = PlayerPower(main_stat=5130, weapon_damage_multiplier=1.96, main_stat_multiplier=26.25, critical_strike_rate=0.244, critical_strike_damage=1.594, direct_hit_rate=0.338, determination_multiplier=1.086, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.086, auto_attack_delays=2.8)
MNK_BIS = PlayerPower(main_stat=5107, weapon_damage_multiplier=1.94, main_stat_multiplier=26.13, critical_strike_rate=0.246, critical_strike_damage=1.596, direct_hit_rate=0.241, determination_multiplier=1.082, speed_multiplier=1.025, tenacity_multiplier=1, auto_direct_hit_increase=0.061, auto_attack_delays=2.56)
NIN_BIS = PlayerPower(main_stat=5107, weapon_damage_multiplier=1.94, main_stat_multiplier=26.13, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.281, determination_multiplier=1.098, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.071, auto_attack_delays=2.56)
VPR_BIS = PlayerPower(main_stat=5061, weapon_damage_multiplier=1.94, main_stat_multiplier=25.89, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.259, determination_multiplier=1.098, speed_multiplier=1.005, tenacity_multiplier=1, auto_direct_hit_increase=0.066, auto_attack_delays=2.64)
RPR_BIS = PlayerPower(main_stat=5130, weapon_damage_multiplier=1.96, main_stat_multiplier=26.25, critical_strike_rate=0.244, critical_strike_damage=1.594, direct_hit_rate=0.328, determination_multiplier=1.086, speed_multiplier=1.002, tenacity_multiplier=1, auto_direct_hit_increase=0.083, auto_attack_delays=3.2)


DNC_BIS = PlayerPower(main_stat=5130, weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.339, determination_multiplier=1.083, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.086, auto_attack_delays=3.12)
BRD_BIS = PlayerPower(main_stat=5130, weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.339, determination_multiplier=1.083, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.086, auto_attack_delays=3.04)
MCH_BIS = PlayerPower(main_stat=5130, weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.248, critical_strike_damage=1.598, direct_hit_rate=0.339, determination_multiplier=1.083, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.086, auto_attack_delays=2.64)


PCT_BIS = PlayerPower(main_stat=5130, weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.245, critical_strike_damage=1.595, direct_hit_rate=0.311, determination_multiplier=1.092, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.079, auto_attack_delays=0)
RDM_BIS = PlayerPower(main_stat=5130, weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.245, critical_strike_damage=1.595, direct_hit_rate=0.311, determination_multiplier=1.092, speed_multiplier=1, tenacity_multiplier=1, auto_direct_hit_increase=0.079, auto_attack_delays=0)
SMN_BIS = PlayerPower(main_stat=5130, weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.240, critical_strike_damage=1.590, direct_hit_rate=0.337, determination_multiplier=1.084, speed_multiplier=1.005, tenacity_multiplier=1, auto_direct_hit_increase=0.085, auto_attack_delays=0)
BLM_BIS = PlayerPower(main_stat=5130, weapon_damage_multiplier=1.96, main_stat_multiplier=26.26, critical_strike_rate=0.249, critical_strike_damage=1.599, direct_hit_rate=0.246, determination_multiplier=1.056, speed_multiplier=1.037, tenacity_multiplier=1, auto_direct_hit_increase=0.062, auto_attack_delays=0)


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

OTHER_SPECS = MELEE_SPECS + CASTER_SPECS

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

    return {
        "mainPlayerId": 0,
        "combatTimeMillisecond": 390000,
        "party": party_infos,
        "partyIlvlAdjustment": 1.0,
        "usePot": True 
    } 

async def send_request(session, url, data):
    async with session.post(url, json=data) as response:
        if response.status == 200:
            response_data = await response.json()
            rdps = sum(int(simulation['simulationSummary']['pdps'][0]) for simulation in response_data['simulationData'])
            return rdps
        else:
            print(f"Error: {response.status}")
            return 0  # Handle errors appropriately

def generate_party_info(tank, tank2, heal1, heal2, melee, ranged, caster, other):
    return create_party_info([tank, tank2, heal1, heal2, melee, ranged, caster, other])

async def send_requests(urls, party_info):
    async with aiohttp.ClientSession() as session:
        tasks = [send_request(session, url, party_info) for url in urls]
        results = await asyncio.gather(*tasks)
        return results

async def main():
    with open('party_rdps_table.json', 'r') as file:
        table = json.load(file)
    count = 0

    left_compositions = []

    for tank in TANK_SPECS:
        for tank2 in TANK_SPECS:
            for i, heal1 in enumerate(HEALER_SPECS):
                for j in range(i + 1, len(HEALER_SPECS)):
                    heal2 = HEALER_SPECS[j]
                    if heal1 == heal2:
                        continue
                    for melee in MELEE_SPECS:
                        for ranged in RANGED_SPECS:
                            for caster in CASTER_SPECS:
                                for other in OTHER_SPECS:
                                    if other in ["PCT", "RDM", "SMN", "MNK", "NIN", "DRG", "RPR", "BRD", "DNC"] and (other == caster or other == ranged or other == melee):
                                        continue
                                    
                                    key = f"{tank}, {tank2}, {heal1}, {heal2}, {melee}, {ranged}, {caster}, {other}"
                                    key_inversed = f"{tank2}, {tank}, {heal2}, {heal1}, {other}, {ranged}, {caster}, {melee}"
                                    if key in table or key_inversed in table:
                                        continue
                                    else:
                                        left_compositions.append({"tank": tank, "tank2": tank2, "heal1": heal1, "heal2": heal2, "melee": melee, "ranged": ranged, "caster": caster, "other": other, "key": key})
    
    for composition in tqdm.tqdm(left_compositions):
        tank = composition["tank"]
        tank2 = composition["tank2"]
        heal1 = composition["heal1"]
        heal2 = composition["heal2"]
        melee = composition["melee"]
        ranged = composition["ranged"]
        caster = composition["caster"]
        other = composition["other"]
        key = composition["key"]

        print(key)

        party_info = create_party_info([tank, tank2, heal1, heal2, melee, ranged, caster, other])
        urls = ["http://localhost:13406/api/v1/simulate"] * 1000
                                    
        rdps_results = await send_requests(urls, party_info)
        rdps_results.sort() 
        print(rdps_results[500])
        table[key] = rdps_results[500]

        count += 1
        if count % 100 == 0:
            print("flushing")
            with open('party_rdps_table.json', 'w') as f:
                json.dump(table, f)

# Entry point for the async function
if __name__ == "__main__":
    asyncio.run(main())