#!/bin/bash
set -e

export THIS_SCRIPT=$(dirname $(dirname -- "$0"))
source $THIS_SCRIPT/constants.sh
source $ROOT/utils.sh

GCC_GIT_TMP_DIR=/tmp/gcc
GIT_REPOSITORY="git://gcc.gnu.org/git/gcc.git"
VERSIONS="7.5.0 8.5.0 9.5.0 10.4.0 11.3.0 12.2.0"

# Installing dependencies
sudo apt-get install -y flex build-essential

# Temporary install directory
if [[ ! -d $GCC_GIT_TMP_DIR ]]; then
    git clone $GIT_REPOSITORY $GCC_GIT_TMP_DIR
fi
cd $GCC_GIT_TMP_DIR

for version in $VERSIONS; do
    current_gcc_dir=$GCC_GIT_TMP_DIR/worktrees/releases/gcc-$version
    install_dir=/opt/gcc/$version
    if [[ -d $install_dir ]]; then
        continue
    fi

    display "Installing GCC version $version"

    # Checking out specific version
    if [[ ! -d $current_gcc_dir ]]; then
        git worktree add $current_gcc_dir releases/gcc-$version
    fi

    # Downloading prerequisites
    cd $current_gcc_dir
    ./contrib/download_prerequisites

    # Building temporary build directory
    mkdir -p $current_gcc_dir/build
    cd $current_gcc_dir/build

    # Building and installing GCC
    ../configure --prefix=$install_dir --disable-multilib
    make -j4
    sudo mkdir -p $(dirname $install_dir)
    sudo make all install
    cd $GCC_GIT_TMP_DIR

    notify "Installed GCC version $version in $install_dir"
done

# TODO: SETUP "MODULES" file
# Apprendre à le faire comme ça tu pourras factoriser tout ça
