#!/usr/bin/env bash

chainx2_pid=$(ps -ef | grep 'archive --config' | grep -v grep | awk '{print $2}')

if [ $chainx2_pid ] ; then
        kill -9 $chainx2_pid
        echo "Killed substrate-archive process, pid: $chainx2_pid"
fi
