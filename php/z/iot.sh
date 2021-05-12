#!/usr/bin/env bash
case $1 in
    'r')
    sudo systemctl stop iot
    rm -fr data log
    sudo systemctl start iot
    ;;
    'd')
    sudo ifconfig enp0s3 down
    ;;
    'u')
    sudo ifconfig enp0s3 up
    ;;
    'n')
    sudo netstat -aonp | grep ':1433' | grep 'task'
    ;;
    'c1')
    cat log/taskworker1 | grep CRITICAL
    ;;
    'c2')
    cat log/taskworker2 | grep CRITICAL
    ;;
    'cl')
    cat log/log_file
    ;;
    'cs1')
    php avgDbProcessTime.php ChannelsStatus log/taskworker1
    ;;
    'cs2')
    php avgDbProcessTime.php ChannelsStatus log/taskworker2
    ;;
    'cd1')
    php avgDbProcessTime.php ChannelsData log/taskworker1
    ;;
    'cd2')
    php avgDbProcessTime.php ChannelsData log/taskworker2
    ;;
    'ds1')
    php avgDbProcessTime.php DeviceStatus log/taskworker1
    ;;
    'ds2')
    php avgDbProcessTime.php DeviceStatus log/taskworker2
    ;;
esac
