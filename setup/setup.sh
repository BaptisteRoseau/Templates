#!/bin/bash

export THIS_SCRIPT=$(dirname $(dirname -- "$0"))
source $THIS_SCRIPT/constants.sh
source $ROOT/utils.sh

INSTALLERS_DIR=$ROOT/installers
HOME_DIRECTORY_ITEMS_DIR=$ROOT/home
mkdir -p $LOGDIR

# Warning prompt
echo "It is STRONGLY recommended to temporarily allow sudo sessions to last longer when running this script.
To do so, add the following line in /etc/sudoers (the unit is in minutes).

    Defaults timestamp_timeout=1440

WARNING: This script will replace ~/.bashrc
The 'module' package will be installed to manage packages during a shell session.
The following apt packages will also be installed:
    $(list_packages_to_install $INSTALLERS_DIR)"
ask_for_confirmation
if [[ $? -ne 0 ]]; then
    echo "Exiting"
    exit
fi

# Caching sudo to allow installers to use it directly without prompt
sudo echo "sudo password cached"

# Starting all installers in parallel
echo "Launching installers:"
for script in $(ls -A $INSTALLERS_DIR); do
    echo "  - $script"
    $INSTALLERS_DIR/$script &
done

# Wait for all jobs to finish
echo "Waiting for all installers to finish. This may take several hours..."
wait
echo "Installers job done"

echo ""
echo "Done."
echo "You should source your ~/.bashrc"
echo "You should remove the 'timestamp_timeout' variable from /etc/sudoers if added previously."
