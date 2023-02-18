set -e

if [[ -d ~/.config/VSCodium/User ]]; then
    cp settings.json ~/.config/VSCodium/User/settings.json
elif [[ -d ~/.config/VSCode/User ]]; then
    cp settings.json ~/.config/VSCode/User/settings.json
else
    >&2 echo "VSCodium/VSCode is not installed !"
    exit 1
fi
echo "Applied settings.json to user's VSCodium"
