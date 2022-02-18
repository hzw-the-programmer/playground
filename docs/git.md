git log --abbrev-commit --pretty=oneline
```shell
git show 1c002d
git show topic1
git rev-parse topic1
git reflog
git show HEAD@{0}
git show HEAD@{1}
git show topic1@{0}
git show topic1@{1}
git show master@{yesterday}
git show HEAD@{2.months.ago}
git log --pretty=format:'%h %s' --graph
git show HEAD^ = git show HEAD^1
git show HEAD^2
git show HEAD~~~ = git show HEAD~3
git log mydev^^^..mydev
git cherry-pick mydev^^^..mydev
git log origin/master..HEAD = git log origin/master..
git log refA..refB = git log ^refA refB = git log refB --not refA
git log refA refB ^refC = git log refA refB --not refC
git log master...experiment
#reachable by either of two references but not by both of them
git log --left-right master...experiment

git stash save
git rebase
git status --untracked-files=no
git checkout --theirs path/to/binary/file
#git checkout --ours path/to/binary/file
git add path/to/binary/file
git checkout --theirs path/to/file
git add path/to/file
git rebase --continue
```

git status applib/mem/
git diff applib/mem/

git tag --contains <commit>
git branch -a --contains <commit>

git tag -d tag1 tag2
git push --delete origin tag1 tag2
git push --tags
git push origin :refs/tags/<tag>

git show commit:path/to/file.c > path/to/file_old.c

git checkout main
git checkout -b main_merge
git merge feature

git log --graph --author=hezhiwen not..reachable
git log --graph --author=hezhiwen ^not reachable

git branch --contains $COMMIT_ID
git tag --contains <commit>

git ls-files --others --exclude-standard >> .gitignore

git ls-files --modified >> modified
xargs --arg-file=modified git update-index --assume-unchanged

git submodule add https://github.com/chaconinc/DbConnector
git diff --cached --submodule
git commit -am 'Add DbConnector module'
git push origin master

git clone https://github.com/chaconinc/MainProject

git submodule init
git submodule update

git submodule update --init
git submodule update --init --recursive

git clone --recurse-submodules https://github.com/chaconinc/MainProject

git log --pretty=format:%h --diff-filter=A -- nucleus
git diff --name-only commitid HEAD

git describe --tags
git describe --tags --abbrev=0
