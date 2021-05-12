git log --reverse
git diff HEAD^ HEAD
git diff HEAD HEAD^
git diff --name-only firstcommit HEAD

git branch --set-upstream-to=origin/release [release]
git branch -u origin/release [release]
git push --set-upstream origin release
git push -u origin release:release

git diff first second
