#!/bin/bash
set -e
export THIS_SCRIPT=$(dirname $(dirname -- "$0"))
source $THIS_SCRIPT/constants.sh
source $ROOT/utils.sh
mkdir -p $LOGDIR

VERSIONS="3.11.1 3.10.9"

sudo apt install -y valgrind libffi-dev

for version in $VERSIONS; do
    echo $version
    install_dir=/opt/Python/$version
    if [[ -d $install_dir ]]; then
        continue
    fi

    logfile_out=$LOGDIR/python-$version.out
    logfile_err=$LOGDIR/python-$version.err
    mkdir -p /tmp/python
    cd /tmp/python
    display "Output and errors displayed in $logfile_out $logfile_err"

    # Downloading Python
    archive=Python-$version.tar.xz
    if [[ ! -f $archive ]]; then
        curl -OL https://www.python.org/ftp/python/$version/$archive >>$logfile_out 2>>$logfile_err
    fi

    # Extracting archive
    tar xf $archive >>$logfile_out 2>>$logfile_err
    cd Python-$version >>$logfile_out 2>>$logfile_err

    # Building and installing CMake
    ./configure --prefix=$install_dir \
        --enable-shared \
        --enable-optimizations \
        --with-valgrind >>$logfile_out 2>>$logfile_err
    make -j 4 >>$logfile_out 2>>$logfile_err
    sudo make install >>$logfile_out 2>>$logfile_err

    notify "Installed Python version $version in $install_dir"
done
