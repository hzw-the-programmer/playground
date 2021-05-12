```shell
git clone --depth 1 --branch release-branch.go1.4 https://github.com/golang/go.git go1.4
cd go1.4/
git status
cd src/
CGO_ENABLED=0 ./make.bash

git clone https://github.com/golang/go.git
cd go
git status
# git describe
# git describe --tags
git branch -a
git describe --tags origin/release-branch.go1.14
git checkout go1.14.2
cd src/
GOROOT_BOOTSTRAP=/home/hezhiwen/bin/go1.4/ ./all.bash
```
