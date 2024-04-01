"""loads FFXIV data from etro API and converts it to a format that can be used by the bot.

1. Use shell commands to load data from etro API
2. Convert the data to a more readable format

>>> ex)
>>> on ffxivsimbot directory
>>> python -m py.load_data
"""

import os
import json

_STAT_ID_NAME_MAP = {
    1: 'Strength',
    2: 'Dexterity',
    3: 'Vitality',
    4: 'Intelligence',
    5: 'Mind',
    6: 'Piety',
    19: 'Tenacity',
    22: 'Direct Hit Rate',
    27: 'Critical Hit',
    44: 'Determination',
    45: 'Skill Speed',
    46: 'Spell Speed',
}

_MAIN_STAT_ABBREV_NAME_MAP = {
    'STR': 'Strength',
    'DEX': 'Dexterity',
    'VIT': 'Vitality',
    'INT': 'Intelligence',
    'MND': 'Mind',
    'PIE': 'Piety',
    'HP': 'Hp'
}

_MAX_MATERIA_TIER = 10


def initialize_all_stat_dict():
    init_dict = {}

    for stat_name in _STAT_ID_NAME_MAP.values():
        init_dict[stat_name] = 0

    return init_dict


def initialize_sub_stat_dict():
    return {
        'Piety': 0,
        'Tenacity': 0,
        'Direct Hit Rate': 0,
        'Critical Hit': 0,
        'Determination': 0,
        'Skill Speed': 0,
        'Spell Speed': 0,
    }


def load_etro_json_data_if_not_loaded(py_directory, file_name):
    json_file_path = os.path.join(py_directory, f'{file_name}.json')
    if not os.path.exists(json_file_path):
        os.system(f'coreapi action {file_name} list > {json_file_path}')


def filter_low_jobs(job_name):
    jobs_list = job_name.split(' ')

    valid_jobs = ["PLD", "WAR", "GNB", "DRK", "WHM", "SCH", "AST", "SGE", "MNK", "DRG", "NIN", "SAM", "RPR", "BRD",
                  "MCH", "DNC", "BLM", "RDM", "SMN"]
    return [job for job in jobs_list if job in valid_jobs]


def convert_equipment(equipment):
    """extract and convert needed equipment data

    Returns:
        Dictionary with
            id,
            name,
            level,
            jobName,
            slotCategory,
            weapon,
            damageMag,
            damagePhys,
            defenseMag,
            defensePhys,
            slotName,
            Strength,
            Dexterity,
            Vitality,
            Intelligence,
            Mind,
            Piety,
            Tenacity,
            Direct Hit,
            Critical Hit,
            Determination,
            Skill Speed,
            Spell Speed,
            materiaSlotCount,
            pentameldable
    """
    equipment_entity = {
        'id': equipment['id'],
        'name': equipment['name'],
        'level': equipment['level'],
        'jobName': filter_low_jobs(equipment['jobName']),
        'slotCategory': equipment['slotCategory'],
        'weapon': equipment['weapon'],
        'damageMag': equipment['damageMag'],
        'damagePhys': equipment['damagePhys'],
        'defenseMag': equipment['defenseMag'],
        'defensePhys': equipment['defensePhys'],
        'slotName': equipment['slotName'],
        'materiaSlotCount': equipment['materiaSlotCount']
    }

    equipment_entity['pentameldable'] = 'Diadochos' in equipment['name']

    stat_data = convert_stat_data(equipment)
    equipment_entity.update(stat_data)

    for stat_name in _STAT_ID_NAME_MAP.values():
        assert (stat_name in equipment_entity)

    return equipment_entity


def convert_stat_data(equipment):
    # convert param1-6 to readable stat name - value

    stat_data = initialize_all_stat_dict()

    for param_id in range(0, 4):
        stat_id = equipment[f'param{param_id}']
        stat_name = _STAT_ID_NAME_MAP[stat_id]
        stat_data[stat_name] = equipment[f'param{param_id}Value']

    return stat_data


def is_combat_item(equipment):
    """Check if the equipment is a combat item

    FFXIV PVE Combat Items
    1) Have only 4 Stats(1 Main Stat, Vitality, 2 Sub Stats)
    2) All stat are one of the combat stats stored in _STAT_ID_NAME_MAP
    """

    if equipment['param4'] is not None:
        return False

    for i in range(0, 4):
        stat_id = equipment[f'param{i}']
        if stat_id not in _STAT_ID_NAME_MAP:
            return False

    return True


def convert_equipments(equipments):
    """Load and convert PVE Combat Items

    Args:
        equipments: List of equipments data from Etro API

    Returns:
        List of converted combat items
    """

    equipments_only_max_level_equipments = [
        equipment for equipment in equipments if equipment['level'] == 90
    ]

    equipments_only_combat_items = []

    for equipment in equipments_only_max_level_equipments:
        if is_combat_item(equipment):
            equipments_only_combat_items.append(equipment)

    return [convert_equipment(equipment) for equipment in equipments_only_combat_items]


def convert_jobs(jobs):
    """convert jobs data

    Args:
        jobs: List of jobs data from Etro API

    Returns:
        Dictionary
           id
           abbrev
           name
           Strength
           Dexterity
           Mind
           Vitality
           Piety
           Strength
           Hp
           IsTank
    """
    jobs_only_combat = [job for job in jobs if (not job['isCrafting']) and (not job['isGathering'])]

    converted_jobs = []

    for job in jobs_only_combat:
        converted_job = {
            'id': job['id'],
            'abbrev': job['abbrev'],
            'name': job['name'],
        }

        if converted_job['abbrev'] in ['PLD', 'WAR', 'GNB', 'DRK']:
            converted_job['IsTank'] = True
        else:
            converted_job['IsTank'] = False

        for stat_abbrev, stat_value in job.items():
            if stat_abbrev in _MAIN_STAT_ABBREV_NAME_MAP:
                stat_full_name = _MAIN_STAT_ABBREV_NAME_MAP[stat_abbrev]
                converted_job[stat_full_name] = stat_value

        converted_jobs.append(converted_job)

    return converted_jobs


def is_combat_food(food):
    """Check if the food is a combat food

    FFXIV PVE Combat Food has vitality and two of the combat sub stats in _STAT_ID_NAME_MAP
    """

    if not food['isFood']:
        return False

    for i in range(0, 3):
        stat_id = food[f'param{i}']
        if stat_id not in _STAT_ID_NAME_MAP:
            return False

    return True


def convert_food(food):
    converted_food = {
        'id': food['id'],
        'name': food['name'],
        'itemLevel': food['itemLevel'],
    }

    converted_food.update(initialize_sub_stat_dict())

    for i in range(0, 3):
        stat_id = food[f'param{i}']
        stat_name = _STAT_ID_NAME_MAP[stat_id]
        converted_food[stat_name] = food[f'maxHQ{i}']

    return converted_food


def convert_foods(foods):
    """convert combat food data

    Args:
        foods: List of foods data from Etro API

    Returns:
       Dictionary
            id
            name
            itemLevel
            Piety,
            Tenacity,
            Direct Hit,
            Critical Hit,
            Determination,
            Skill Speed,
            Spell Speed
    """

    combat_foods = [food for food in foods if is_combat_food(food)]
    return [convert_food(food) for food in combat_foods]


def convert_clans(clans):
    """convert clan data

    Args:
        clans: List of clans data from Etro API

    Returns:
        dictionary with the following columns:
            id,
            name,
            Strength,
            Dexterity,
            Vitality,
            Intelligence,
            Mind,
            Piety,
    """

    converted_clans = []

    for clan in clans:
        converted_clan = {
            'id': clan['id'],
            'name': clan['name'],
        }

        for stat_abbrev, stat_value in clan.items():
            if stat_abbrev in _MAIN_STAT_ABBREV_NAME_MAP:
                stat_full_name = _MAIN_STAT_ABBREV_NAME_MAP[stat_abbrev]
                converted_clan[stat_full_name] = stat_value

        converted_clans.append(converted_clan)

    return converted_clans


def convert_data(py_directory, file_name, convert_function):
    json_file_path = os.path.join(py_directory, f'{file_name}.json')

    with open(json_file_path, 'r') as f:
        data = json.load(f)
        converted_data = convert_function(data)

    with open(os.path.join(py_directory, f'{file_name}_data.json'), 'w') as f:
        json.dump(converted_data, f)


_ETRO_JSON_FILES = ['equipment', 'jobs', 'food', 'clans']
_CONVERT_FUNCTIONS = [convert_equipments, convert_jobs, convert_foods, convert_clans]


def main():
    working_directory = os.getcwd()
    py_directory = os.path.join(working_directory, 'py')

    for (file_name, convert_function) in zip(_ETRO_JSON_FILES, _CONVERT_FUNCTIONS):
        print(f'Loading {file_name} data...')
        load_etro_json_data_if_not_loaded(py_directory, file_name)

        convert_data(py_directory, file_name, convert_function)


if __name__ == '__main__':
    main()
