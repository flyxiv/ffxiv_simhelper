#!/bin/bash

pushd py
./download_ffxiv_data.sh
popd
mv py/*.json resources/
