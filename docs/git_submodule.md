git clone https://parent.git
git submodule add https://lib.git lib/child
cd lib/child
git checkout working_branch
cd ../..
git add .
git commit -m "add child project."

git reset --hard HEAD^

```
git mv old/submod new/submod
```
