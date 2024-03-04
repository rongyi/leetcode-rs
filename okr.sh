#!/bin/bash


git log --since=midnight --name-only --pretty=format:'' | grep -v '^$'


cnt=`git log --since=midnight --name-only --pretty=format:'' | grep -v '^$' |grep '.rs$' | wc -l`
echo "today you finished $cnt leetcode in rust, happy coding ry!"

#git log --since='7 days ago' --name-only --pretty=format:'' | grep -v '^$' |grep '.rs$'


cnt=`git log --since='7 days ago' --name-only --pretty=format:'' | grep -v '^$' |grep '.rs$'| wc -l`
echo "last 7 days you finished $cnt leetcode in rust, happy coding ry!"
