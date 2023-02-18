#!/bin/bash
set -e
export THIS_SCRIPT=$(dirname $(dirname -- "$0"))
source $THIS_SCRIPT/constants.sh
source $ROOT/utils.sh
mkdir -p $LOGDIR

VERSIONS="8.0"

sudo apt install -y libncurses5-dev pkg-config

for version in $VERSIONS; do
    echo $version
    tmp_dir=/tmp/MySQL
    install_dir=/opt/MySQL/$version
    if [[ -d $install_dir ]]; then
        continue
    fi

    logfile_out=$LOGDIR/MySQL-$version.out
    logfile_err=$LOGDIR/MySQL-$version.err
    mkdir -p ${tmp_dir}
    cd ${tmp_dir}
    display "Output and errors displayed in $logfile_out $logfile_err"

    # Downloading MySQL
    archive=${tmp_dir}/mysql-${version}.32-linux-glibc2.17-x86_64-minimal.tar
    if [[ ! -f $archive ]]; then
        curl -OL https://dev.mysql.com/get/Downloads/MySQL-${version}/mysql-${version}.32-linux-glibc2.17-x86_64-minimal.tar
    fi

    # Extracting binaries
    tar xf ${archive} >>$logfile_out 2>>$logfile_err
    tar xf ${archive}.xz >>$logfile_out 2>>$logfile_err
    sudo mkdir -p $(dirname $install_dir)
    sudo cp -r mysql-${version}.32-linux-glibc2.17-x86_64-minimal $install_dir >>$logfile_out 2>>$logfile_err

    rm -rf ${tmp_dir}

    notify "Installed MySQL version $version in $install_dir"
done
