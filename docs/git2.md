ssh-keygen -t rsa -b 4096 -C coder@hzw.com
cat ~/.ssh/id_rsa.pub

git clone -b dev --recurse-submodules ssh://git@host/project.git

git branch --contains tags/v0.1.2
git log --since="1 week ago"
git remote set-url origin git@github.com:hzw-the-programmer/playground.git

ssh-keygen -o -t rsa -C "email" -b 4096
