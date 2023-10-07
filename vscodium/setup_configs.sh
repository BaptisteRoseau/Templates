#!/bin/bash
set -e

CONFIG_DIR='VSCodium'
CONFIG_FILES=`ls *.json | tr "\n" " "`

if [[ -d ~/.config/VSCodium/User ]]; then
    CONFIG_DIR='VSCodium'
elif [[ -d ~/.config/VSCode/User ]]; then
    CONFIG_DIR='VSCode'
else
    >&2 echo "VSCodium/VSCode is not installed !"
    exit 1
fi

cp ${CONFIG_FILES} ~/.config/${CONFIG_DIR}/User/
echo "Applied ${CONFIG_FILES} to user's ${CONFIG_DIR}"
