mkdir project
cd project
repo init -u minifest_url -b menifest_branch
repo sync -cqd --no-tags

repo sync --help
du -sh project