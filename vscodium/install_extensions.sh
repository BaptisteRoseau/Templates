if [[ ! `command -v code` && ! `command -v codium` ]]; then
    >&2 echo "VSCodium/VSCode is not installed !"
    exit 1
fi
cat extentions | xargs codium --install-extension
