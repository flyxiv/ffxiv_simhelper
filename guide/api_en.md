# API Specification

## 0. Request Overview

| API route           | Request Type           | Description                                                                             |
|---------------------|------------------------|-----------------------------------------------------------------------------------------|
| /api/v1/quicksim    | POST(application/json) | Very quickly simulate player's detailed damage profile. (1000 iterations)               |
| /api/v1/gearcompare | POST(application/json) | Detailed simulation of expected DPS for two different gearsets. (4000 iterations each)  |
| /api/v1/bestpartner | POST(application/json) | Finds out which jobs contributes the most to the main player's raidbuffs (500 iteration each) |
| /api/v1/statweights | POST(application/json) | Finds out the expected DPS increase per stat point for each character stats (500 iteration each) | 


## 1. QuickSim
* Fastest simulation + In-detail analysis of player DPS
* DPS Summary/Party Member Contribution/My Contribution to Party Members/Sample of Damage Rotation
* **The root of all simulation. Other simulations are done using the QuickSim in a specific way.**

#### Request Parameters: SimulationApiRequest
| Parameter | Type | Description |
--|--|--
| mainPlayerId | int | id of the main character in the "party" parameter array |
| combatTimeMillisecond | int | duration of simulation in millisecond |
| party | PartyInfoRequest | Status of the party members |
| partyIlvlAdjustment | float | tuning parameter depending on the input party iLvl. The value is a float < 1.0| 
| usePot | boolean | decides whether the simulation is done using pots or not |


#### PartyRequest
| Parameter | Type | Description |
--|--|--
| playerId | int | id of the data |
| partnerId1 | int or null | id of the first partner player in the party. This value is null for every job except DNC and AST. For AST, it is the ID of the melee card target, and for DNC it is the ID of the dance partner. If the value is null for DNC or AST, the partner is assigned automatically based on internal algorithm. |
| partnerId2 | int or null | id of the second partner player in the party. This value is null for every job except AST. For AST, it is the ID of the ranged card target. If the value is null for AST, the partner is assigned automatically based on internal algorithm. |
| jobAbbrev | string | 3-letter abbreviation of the player's combat job |
| power | PlayerPower | Specific stats and stat ladder values of player |


#### PlayerPower
| Parameter | Type | Description |
--|--|--
| weaponDamage | int | Weapon Damage Stat of the weapon |
| mainStat | int | Main stat(STR, DEX, INT, MND) related to the player's job |
| criticalStrike | int | Critical Strike substat of player |
| directHit | int | Direct Hit substat of player | 
| determination | int | Determination substat of player |
| skillSpeed | int | Skill Speed substat of player |
| spellSpeed | int | Spell Speed substat of player |
| tenacity | int | Tenacity substat of player |
| piety | int | Piety substat of player |
| weaponDamageMultiplier | float | the actual weapon damage increase value. Value must be greater than 1. ex) 1% damage increase = 1.01
| mainStatMultiplier | float | the main stat damage increase value. Value must be greater than 1. ex) 1% damage increase = 1.01
| criticalStrikeRate | float | critical 


### Request Example

```json
{
  "mainPlayerId": 0,
  "combatTimeMillisecond": 300000,
  "party": [
    {
      "playerId": 0,
      "job": "WAR",
      "role": "Tank",
      "stats": {
        "weaponDamage": 100,
        "mainStat": 100,
        "criticalStrike": 100,
        "directHit": 100,
        "determination": 100,
        "speed": 100,
        "tenacity": 100
      }
    },
    {
      "playerId": 1,
      "job": "PLD",
      "role": "Tank",
      "stats": {
        "weaponDamage": 100,
        "mainStat": 399,
        "criticalStrike": 388,
        "directHit": 390,
        "determination": 288,
        "speed": 158,
        "tenacity": 100
      }
    },
    {
      "playerId": 2,
      "job": "WHM",
      "role": "Healer",
      "stats": {
        "weaponDamage": 100,
        "mainStat": 2195,
        "criticalStrike": 562,
        "directHit": 291,
        "determination": 196,
        "speed": 532,
        "tenacity": 562
      }
    },
    {
      "playerId": 3,
      "job": "SGE",
      "role": "Healer",
      "stats": {
        "weaponDamage": 100,
        "mainStat": 500,
        "criticalStrike": 291,
        "directHit": 691,
        "determination": 561,
        "speed": 1193,
        "tenacity": 188
      }
    },
    {
      "playerId": 4,
      "job": "DRG",
      "role": "Melee",
      "stats": {
        "weaponDamage": 100,
        "mainStat": 2911,
        "criticalStrike": 1638,
        "directHit": 1819,
        "determination": 592,
        "speed": 191,
        "tenacity": 100
      }
    },
    {
      "playerId": 5,
      "job": "NIN",
      "role": "Melee",
      "stats": {
        "weaponDamage": 100,
        "mainStat": 1800,
        "criticalStrike": 271,
        "directHit": 921,
        "determination": 722,
        "speed": 100,
        "tenacity": 100
      }
    },
    {
      "playerId": 6,
      "job": "BRD",
      "role": "Ranged",
      "stats": {
        "weaponDamage": 100,
        "mainStat": 1591,
        "criticalStrike": 1162,
        "directHit": 2781,
        "determination": 100,
        "speed": 500,
        "tenacity": 100
      }
    },
    {
      "playerId": 7,
      "job": "BLM",
      "role": "Caster",
      "stats": {
        "weaponDamage": 100,
        "mainStat": 119,
        "criticalStrike": 592,
        "directHit": 100,
        "determination": 100,
        "speed": 2911,
        "tenacity": 100
      }
    }
  ]
}
```

* job types

| job name | description |
|----------|-------------|
| WAR      | Warrior     |
| PLD      | Paladin     |
| GNB      | Gunbreaker  |
| DRK      | Dark Knight |
| WHM      | White Mage  |
| AST      | Astrologian |
| SGE      | Sage        |
| SCH      | Scholar     |
| DRG      | Dragoon     |
| NIN      | Ninja       |
| MNK      | Monk        |
| SAM      | Samurai     |
| RPR      | Reaper      |
| BRD      | Bard        |
| MCH      | Machinist   |
| BLM      | Black Mage  |
| SMN      | Summoner    |
| RDM      | Red Mage    |

* roles

  | role name |
        |-----------|
  | Tank      |
  | Healer    |
  | Melee     |
  | Ranged    |
  | Caster    |

## 2. Response

* Needed Criteria

| Field                | Description                                                                                                                        |
|----------------------|------------------------------------------------------------------------------------------------------------------------------------|
| mainPlayerId         | The player ID of the requested user.                                                                                               |
| rotationLog          | Time of skill use in millisecond, skill_id, target, damage dealt(target ID 100 is for enemy), damage contribution to each raidbuff |
| fightTimeMillisecond | The requested fight time in millisecond                                                                                            |

* Example Response

```json
{
  "mainPlayerId": 5,
  "partySimulationResult": [
    {
      "playerId": 0,
      "job": "WAR",
      "role": "Tank",
      "rotationLog": [
        {
          "time": -562,
          "skillId": 1,
          "target": 100,
          "rawDamage": 100,
          "rdpsContribution": [
            {
              "playerId": 3,
              "raidBuffStatusId": 802,
              "contributedDamage": 787.27
            },
            {
              "playerId": 5,
              "raidBuffStatusId": 1305,
              "contributedDamage": 781.27
            }
            //...
          ]
        }
      ]
    }
  ]
}
```

* Needed Query: From the raw data of the request, use GraphQL to query the following needed data:

| Field              | Description                                                                  |
|--------------------|------------------------------------------------------------------------------|
| RDPS               | My raw DPS + party members' contribution to my raid buffs                    |
| ADPS               | RDPS + My contribution to other party member's buffs                         |
| PDPS               | My raw DPS + My contribution to other party member's buffs                   |
| Adjusted DPS       | RDPS - my buff contribution + my contribution to party member buffs          |
| Contribution Table | RDPS contribution of each skill id, for each (playerId, raid buff Status Id) |
| Damage Table       | Damage Percent of each skill and their RDPS contribution                     |
