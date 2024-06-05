# API Specification

## 1. Request

| API route        | Request Type           | Description                                                                             |
|------------------|------------------------|-----------------------------------------------------------------------------------------|
| /api/quicksim    | POST(application/json) | Only customize player + party composition, and rest of the party member is default BIS. |
| /api/advancedsim | POST(application/json) | Customize all party job + stats                                                         |

### Request Body Example

* Requested in POST json body.

```json
{
  "mainPlayerId": 5,
  "party": [
    {
      "playerId": 0,
      "job": "WAR",
      "role": "Tank",
      "stats": {
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

| Field | Description |
|-------|-------------|

| Rotation Log | time in millisecond, skill_id, target, damage dealt(target ID 100 is for enemy) |

* Example Response

```json
{
  "mainPlayerId": 5,
  "partyDamageResult": [
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
              "damage": 787.27
            },
            {
              "playerId": 5,
              "raidBuffStatusId": 1305,
              "damage": 781.27
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
