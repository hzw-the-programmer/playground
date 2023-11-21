git remote
git remote add hzw wrongurl
git remote get-url hzw
git remote set-url hzw git@github.com:hzw-the-programmer/mbedtls.git
git push origin master

```
$ cat .git/config
[core]
        repositoryformatversion = 0
        filemode = false
        bare = false
        logallrefupdates = true
        symlinks = false
        ignorecase = true
[remote "origin"]
        url = https://github.com/Mbed-TLS/mbedtls.git
        fetch = +refs/heads/*:refs/remotes/origin/*
[branch "development"]
        remote = origin
        merge = refs/heads/development
[remote "hzw"]
        url = git@github.com:hzw-the-programmer/mbedtls.git
        fetch = +refs/heads/*:refs/remotes/hzw/*
```

git push --set-upstream hzw hzw/dev

```
$ cat .git/config
[core]
        repositoryformatversion = 0
        filemode = false
        bare = false
        logallrefupdates = true
        symlinks = false
        ignorecase = true
[remote "origin"]
        url = https://github.com/Mbed-TLS/mbedtls.git
        fetch = +refs/heads/*:refs/remotes/origin/*
[branch "development"]
        remote = origin
        merge = refs/heads/development
[remote "hzw"]
        url = git@github.com:hzw-the-programmer/mbedtls.git
        fetch = +refs/heads/*:refs/remotes/hzw/*
[branch "hzw/dev"]
        remote = hzw
        merge = refs/heads/hzw/dev
```
