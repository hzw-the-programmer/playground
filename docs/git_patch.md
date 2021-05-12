```
git diff > mypatch.patch
git apply mypatch.patch
```

```
git add .
git diff --cached > mypatch.patch
git apply mypatch.patch
```

```
git add .
git diff --cached --binary > mypatch.patch
git apply mypatch.patch
```

```
git stash list
git stash show -p stash@{<number>} > <name>.patch
```

git format-patch <-> git am
git diff <-> git apply
