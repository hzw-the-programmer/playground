```
$ cat lsquic/.gitmodules
[submodule "src/liblsquic/ls-qpack"]
        path = src/liblsquic/ls-qpack
        url = https://github.com/litespeedtech/ls-qpack
[submodule "src/lshpack"]
        path = src/lshpack
        url = https://github.com/litespeedtech/ls-hpack
```

git submodule set-url src/lshpack git@github.com:hzw-the-programmer/ls-hpack.git
git submodule set-url src/liblsquic/ls-qpack git@github.com:hzw-the-programmer/ls-qpack.git

```
$ cat .gitmodules
[submodule "src/liblsquic/ls-qpack"]
        path = src/liblsquic/ls-qpack
        url = git@github.com:hzw-the-programmer/ls-qpack.git
[submodule "src/lshpack"]
        path = src/lshpack
        url = git@github.com:hzw-the-programmer/ls-hpack.git
```

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
        url = git@github.com:hzw-the-programmer/lsquic.git
        fetch = +refs/heads/*:refs/remotes/origin/*
[branch "master"]
        remote = origin
        merge = refs/heads/master
```

git submodule init

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
        url = git@github.com:hzw-the-programmer/lsquic.git
        fetch = +refs/heads/*:refs/remotes/origin/*
[branch "master"]
        remote = origin
        merge = refs/heads/master
[submodule "src/liblsquic/ls-qpack"]
        active = true
        url = git@github.com:hzw-the-programmer/ls-qpack.git
[submodule "src/lshpack"]
        active = true
        url = git@github.com:hzw-the-programmer/ls-hpack.git
```

git submodule update
