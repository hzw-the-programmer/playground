git update-index --assume-unchanged <file>
git update-index --no-assume-unchanged <file>
git ls-files -v | grep '^h'

### git log
git log --grep="xxx" // reachable form HEAD. search commit message
git log --stat --author="coder" origin/main // reachable form HEAD
git log --graph --stat --author="coder"
