#!/bin/bash


git log --since=midnight --name-only --pretty=format:'' | grep -v '^$'


cnt=`git log --since=midnight --name-only --pretty=format:'' | grep -v '^$' |wc -l`
echo "today you finished $cnt leetcode in rust, happy coding ry!"
