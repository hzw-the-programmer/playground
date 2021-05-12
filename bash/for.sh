# https://opensource.com/article/18/5/you-dont-know-bash-intro-bash-arrays
dirs=("output/bin" "output/conf" "output/templates" "output/translates")
for dir in ${dirs[@]}; do
    echo "dir $dir"
done
