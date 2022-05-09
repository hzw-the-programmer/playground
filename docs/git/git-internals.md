$ git init test
Initialized empty Git repository in E:/work/test/.git/

$ cd test

$ find .git/objects/
.git/objects/
.git/objects/info
.git/objects/pack

$ echo 'test content' | git hash-object -w --stdin
d670460b4b4aece5915caf5c68d12f560a9fe3e4

$ find .git/objects/ -type f
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4

$ git cat-file -p d670460b4b4aece5915caf5c68d12f560a9fe3e4
test content

$ git cat-file -t d670460b4b4aece5915caf5c68d12f560a9fe3e4
blob

$ echo 'version 1' > test.txt

$ git hash-object -w test.txt
warning: LF will be replaced by CRLF in test.txt.
The file will have its original line endings in your working directory
83baae61804e65cc73a7201a7252750c76066a30

$ find .git/objects/ -type f
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4

$ git cat-file -p 83baae61804e65cc73a7201a7252750c76066a30
version 1

$ echo 'version 2' > test.txt

$ git hash-object -w test.txt
warning: LF will be replaced by CRLF in test.txt.
The file will have its original line endings in your working directory
1f7a7a472abf3dd9643fd615f6da379c4acb3e3a

$ find .git/objects/ -type f
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4

$ git cat-file -p 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a
version 2

$ git update-index --add --cacheinfo 100644 83baae61804e65cc73a7201a7252750c76066a30 test.txt

$ git write-tree
d8329fc1cc938780ffdd9f94e0d364e0ea74f579

$ find .git/objects/ -type f
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
.git/objects/d8/329fc1cc938780ffdd9f94e0d364e0ea74f579

$ git cat-file -p d8329fc1cc938780ffdd9f94e0d364e0ea74f579
100644 blob 83baae61804e65cc73a7201a7252750c76066a30    test.txt

$ git update-index --add --cacheinfo 100644 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a test.txt

$ echo 'new file' > new.txt

$ git update-index --add new.txt
warning: LF will be replaced by CRLF in new.txt.
The file will have its original line endings in your working directory

$ git write-tree
0155eb4229851634a0f03eb265b69f5a2d56f341

$ find .git/objects/ -type f
.git/objects/01/55eb4229851634a0f03eb265b69f5a2d56f341
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
.git/objects/d8/329fc1cc938780ffdd9f94e0d364e0ea74f579
.git/objects/fa/49b077972391ad58037050f2a75f74e3671e92

$ git cat-file -p 0155eb4229851634a0f03eb265b69f5a2d56f341
100644 blob fa49b077972391ad58037050f2a75f74e3671e92    new.txt
100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a    test.txt

$ git read-tree --prefix=bak d8329fc1cc938780ffdd9f94e0d364e0ea74f579

$ git write-tree
3c4e9cd789d88d8d89c1073707c3585e41b0e614

$ find .git/objects/ -type f
.git/objects/01/55eb4229851634a0f03eb265b69f5a2d56f341
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a
.git/objects/3c/4e9cd789d88d8d89c1073707c3585e41b0e614
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
.git/objects/d8/329fc1cc938780ffdd9f94e0d364e0ea74f579
.git/objects/fa/49b077972391ad58037050f2a75f74e3671e92

$ git cat-file -p 3c4e9cd789d88d8d89c1073707c3585e41b0e614
040000 tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579    bak
100644 blob fa49b077972391ad58037050f2a75f74e3671e92    new.txt
100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a    test.txt

$ echo 'First commit' | git commit-tree d8329f
8b51a3b6638b346030baacdfaeb07b0300d4e98c

$ find .git/objects/ -type f
.git/objects/01/55eb4229851634a0f03eb265b69f5a2d56f341
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a
.git/objects/3c/4e9cd789d88d8d89c1073707c3585e41b0e614
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
.git/objects/8b/51a3b6638b346030baacdfaeb07b0300d4e98c
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
.git/objects/d8/329fc1cc938780ffdd9f94e0d364e0ea74f579
.git/objects/fa/49b077972391ad58037050f2a75f74e3671e92

$ git cat-file -p 8b51a3b6638b346030baacdfaeb07b0300d4e98c
tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579
author hezhiwen <hezhiwen.coder@bytedance.com> 1652086616 +0800
committer hezhiwen <hezhiwen.coder@bytedance.com> 1652086616 +0800

First commit

$ echo 'Second commit' | git commit-tree 0155eb -p 8b51a3b6638b346030baacdfaeb07b0300d4e98c
a02d7b563fff348674dc57fe0f5f9c63e0098963

$ git cat-file -p a02d7b563fff348674dc57fe0f5f9c63e0098963
tree 0155eb4229851634a0f03eb265b69f5a2d56f341
parent 8b51a3b6638b346030baacdfaeb07b0300d4e98c
author hezhiwen <hezhiwen.coder@bytedance.com> 1652086728 +0800
committer hezhiwen <hezhiwen.coder@bytedance.com> 1652086728 +0800

Second commit

$ echo 'Third commit'  | git commit-tree 3c4e9c -p a02d7b563fff348674dc57fe0f5f9c63e0098963
7528c2ed11b11594003bb8ed5b68b00d9f8f0891

$ git cat-file -p 7528c2ed11b11594003bb8ed5b68b00d9f8f0891
tree 3c4e9cd789d88d8d89c1073707c3585e41b0e614
parent a02d7b563fff348674dc57fe0f5f9c63e0098963
author hezhiwen <hezhiwen.coder@bytedance.com> 1652086806 +0800
committer hezhiwen <hezhiwen.coder@bytedance.com> 1652086806 +0800

Third commit

$ git log 7528c2ed11b11594003bb8ed5b68b00d9f8f0891
commit 7528c2ed11b11594003bb8ed5b68b00d9f8f0891
Author: hezhiwen <hezhiwen.coder@bytedance.com>
Date:   Mon May 9 17:00:06 2022 +0800

    Third commit

commit a02d7b563fff348674dc57fe0f5f9c63e0098963
Author: hezhiwen <hezhiwen.coder@bytedance.com>
Date:   Mon May 9 16:58:48 2022 +0800

    Second commit

commit 8b51a3b6638b346030baacdfaeb07b0300d4e98c
Author: hezhiwen <hezhiwen.coder@bytedance.com>
Date:   Mon May 9 16:56:56 2022 +0800

    First commit
