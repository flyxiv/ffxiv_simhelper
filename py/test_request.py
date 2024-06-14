import requests

if __name__ == '__main__':
    url = 'http://localhost:13406/api/v1/simulate'
    request_body = {
        'mainPlayerId': 0,
        'combatTimeMillisecond': 23000,
        'party': [
            {
                'playerId': 0,
                'job': 'NIN',
                'role': 'melee',
                'stats': {
                    'weaponDamage': 132,
                    'mainStat': 3360,
                    'criticalStrike': 2554,
                    'directHit': 1582,
                    'determination': 1697,
                    'speed': 400,
                    'tenacity': 400,
                }
            }
        ]
    }

    response = requests.post(url, json=request_body)
    print(response.json())
