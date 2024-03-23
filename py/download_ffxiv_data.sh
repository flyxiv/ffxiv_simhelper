#!/bin/bash

. venv/bin/activate

coreapi action equipment list > equipment.json
coreapi action jobs list > jobs.json
coreapi action food list > food.json
coreapi action materia list > materia.json
coreapi action clans list > clans.json
coreapi action medicine list > medicine.json

deactivate
