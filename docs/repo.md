```
curl https://storage.googleapis.com/git-repo-downloads/repo > ~/bin/repo
chmod a+x ~/bin/repo
mkdir android
cd android/
repo init -u https://android.googlesource.com/platform/manifest
repo sync
```

```
cat ~/.gitconfig
cat ~/.git-credentials
git config --global http.postBuffer 2000000000
git config --global http.maxRequestBuffer 100M
git config --global core.compression 0
git config --global credential.https://git.hzw.com.username hzw
git config --global credential.helper store
git ls-remote https://git.hzw.com/gerrit/platform/manifest

mkdir s
cd s
repo init -u https://git.hzw.com/gerrit/platform/manifest -b hzw/project
repo sync

df -h
du -sh s
```
