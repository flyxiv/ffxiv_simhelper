#!/bin/bash

# loads all the data needed for FFXIV Simbot via Etro.gg APIs

python -m py.load_data
mv py/*_data.json data
