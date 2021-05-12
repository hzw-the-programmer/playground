# feature branch

```
git checkout -b myfeature develop

git checkout develop
git merge --no-ff myfeature
git branch -d myfeature
git push origin develop
```

# release branch

```
git checkout -b release-1.2 develop

./bump-version.sh 1.2
git commit -a -m "Bumped version number to 1.2"

git checkout master
git merge --no-ff release-1.2
git tag -a 1.2

git checkout develop
git merge --no-ff release-1.2

git branch -d release-1.2
```

# hotfix branch

```
git checkout -b hotfix-1.2.1 master

./bump-version.sh 1.2.1
git commit -a -m "Bumped version number to 1.2.1"

git commit -m "Fixed severe production problem"

git checkout master
git merge --no-ff hotfix-1.2.1
git tag -a 1.2.1

git checkout develop
git merge --no-ff hotfix-1.2.1

git branch -d hotfix-1.2.1
```
