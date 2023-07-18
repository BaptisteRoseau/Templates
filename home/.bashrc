#!/bin/bash

for file in ~/.bashrc_utils/.*.sh; do
    source "${file}"
done

if [[ -f /opt/Modules/init/bash ]]; then
    source /opt/Modules/init/bash
fi
