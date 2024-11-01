import json
import tqdm

def main():
    with open('party_rdps_table.json', 'r') as file:
        table = json.load(file)

    visited_keys = []
    parsed_table = []

    for key, rdps in tqdm.tqdm(table.items()):
        key_parsed = key.split(', ')
        tank1 = key_parsed[0]
        tank2 = key_parsed[1]
        healer1 = key_parsed[2]
        healer2 = key_parsed[3]
        melee = key_parsed[4]
        ranged = key_parsed[5]
        caster = key_parsed[6]
        other = key_parsed[7]

        key_list = sorted([tank1, tank2, healer1, healer2, melee, ranged, caster, other])
        key = "".join(key_list)

        if key in visited_keys:
            print("skipped!@@@@@@@@@@@@@")
            continue 

        visited_keys.append(key)
        entry = {"tank1" : tank1, "tank2" : tank2, "healer1" : healer1, "healer2" : healer2, "melee" : melee, "ranged" : ranged, "caster" : caster, "other" : other, "rdps" : rdps}
        parsed_table.append(entry)
    
    parsed_table_sorted = sorted(parsed_table, key = lambda row: row["rdps"], reverse=True)
    
    with open('party_rdps_table_parsed.json', 'w') as f:
        json.dump(parsed_table_sorted, f)





if __name__ == '__main__':
    main()