# API Specification

| API route        | Request Type           | Description                                                                             |
|------------------|------------------------|-----------------------------------------------------------------------------------------|
| /api/quicksim    | POST(application/json) | Only customize player + party composition, and rest of the party member is default BIS. |
| /api/advancedsim | POST(application/json) | Customize all party job + stats                                                         |

## Request Body Example

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