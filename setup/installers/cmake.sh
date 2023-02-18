#!/bin/bash
set -e
export THIS_SCRIPT=$(dirname $(dirname -- "$0"))
source $THIS_SCRIPT/constants.sh
source $ROOT/utils.sh
mkdir -p $LOGDIR

VERSIONS="3.26.0-rc1"

logfile_out=$LOGDIR/cmake-apt.out
logfile_err=$LOGDIR/cmake-apt.err
sudo apt-get install -y curl libssl-dev >>$logfile_out 2>>$logfile_err

for version in $VERSIONS; do
    logfile_out=$LOGDIR/cmake-$version.out
    logfile_err=$LOGDIR/cmake-$version.err
    mkdir -p /tmp/cmake
    cd /tmp/cmake
    display "Output and errors displayed in $logfile_out $logfile_err"

    # Downloading CMake
    if [[ ! -f cmake-$version.tar.gz ]]; then
        curl -OL https://github.com/Kitware/CMake/releases/download/v$version/cmake-$version-SHA-256.txt >>$logfile_out 2>>$logfile_err
        curl -OL https://github.com/Kitware/CMake/releases/download/v$version/cmake-$version.tar.gz >>$logfile_out 2>>$logfile_err
        sha256sum -c --ignore-missing cmake-$version-SHA-256.txt >>$logfile_out 2>>$logfile_err
        if [[ $? != 0 ]]; then
            display "Suspicous CMake package for version $version ! SHA-256 is different !"
            continue
        fi
    fi

    # Extracting archive
    tar xzf cmake-$version.tar.gz >>$logfile_out 2>>$logfile_err
    cd cmake-$version >>$logfile_out 2>>$logfile_err

    # Building and installing CMake
    install_dir=/opt/CMake/$version
    ./bootstrap --prefix=$install_dir >>$logfile_out 2>>$logfile_err
    make -j4 >>$logfile_out 2>>$logfile_err
    sudo make install >>$logfile_out 2>>$logfile_err

    notify "Installed Cmake version $version in $install_dir"
done
