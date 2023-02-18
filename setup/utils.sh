function list_packages_to_install(){
    # Recursively search in given directory for packages installed using:
    # "sudo apt-get install -y package1 package 2"
    if [ $# -lt 1 ]; then \
        echo "Usage: list_packages_to_install DIRECTORY"
        return
    fi

    pattern='apt\(-get\)\? install \(-y \)\?'
    grep -Re "$pattern.*" $1 -o \
        | sed "s/$pattern//p" \
        | cut -d":" -f2 \
        | awk '{print}' ORS='\n' \
        | sort -u \
        | awk '{print}' ORS=' '
}

function ask_for_confirmation(){
    # Continuously ask user to answer with  "yes" or "no"
    # yes returns 0
    # no returns 1
    while true; do \
        read -p "Continue ? (yes/no)" answer
        case $answer in
            yes ) return 0;;
            no ) return 1;;
            * ) echo "Please answer yes or no."
        esac
    done
}

function display(){
    echo "`basename $0`>: $@"
}

function notify(){
    display $@
    if command -v notify-send &> /dev/null; then \
        notify-send `basename $0` "$@"
    fi
}
