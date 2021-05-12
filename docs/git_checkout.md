```shell
git stash save
git checkout dev
git checkout -b mydev
git stash pop
git checkout --theirs path/to/conflict/file
#git checkout --ours path/to/conflict/file
#git add path/to/conflict/file
git restore --staged path/to/conflict/file
```