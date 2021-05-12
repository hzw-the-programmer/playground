git clone https://github.com/golang/go.git

git clone -b release-branch.go1.4 go/.git go1.4
cd go1.4/src
./make.bash

cd ../..
mv go1.4 ~

cd go
git checkout go1.12rc1
cd src
./all.bash

