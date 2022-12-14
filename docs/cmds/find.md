find . -maxdepth 2 -name "*.c" -printf "%P\n"
find . -maxdepth 2 -name "*.c" -printf "%P\n" | sort
find crypto/ -maxdepth 1 -type d
find crypto -maxdepth 1 -type d
find ssl -maxdepth 1 -name "*.cc" -printf "%f\n"
find crypto/ -maxdepth 2 -name "*.c" -printf "%P\n"
find ssl/ -maxdepth 1 -name "*.cc" -printf "%P\n"

find ssl/ -maxdepth 1 -name "*.cc" -and ! -name "*test*" -printf "%P\n"

find ssl/ -maxdepth 1 -name "*.cc" -and ! -name "*test*" -printf "%P\n" | LC_COLLATE=en_US.UTF-8 sort --debug
find ssl/ -maxdepth 1 -name "*.cc" -and ! -name "*test*" -printf "%P\n" | LC_COLLATE=en_US.UTF-8 sort
