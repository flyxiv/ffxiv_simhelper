#!/bin/bash

# loads all the data needed for FFXIV Simbot via Etro.gg APIs

pushd py
./download_ffxiv_data.sh
popd
mv py/*.json resources/
