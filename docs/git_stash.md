git stash
git stash list
git stash show -p stash@{0}
git stash pop --index stash@{0}
git stash apply --index stash@{0}
git stash drop stash@{0}
git stash branch test stash@{0}
