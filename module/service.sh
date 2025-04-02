#!/system/bin/sh
MODDIR=${0%/*}
LOG=$MODDIR/log.txt

wait_until_login() {
    # in case of /data encryption is disabled
    while [ "$(getprop sys.boot_completed)" != "1" ]; do sleep 1; done
    # we doesn't have the permission to rw "/sdcard" before the user unlocks the screen
    until [ -d /sdcard/Android ]; do sleep 1; done
}

if [ "$(getprop sys.boot_completed)" != "1" ]; then
    wait_until_login
    if [ ! -L $MODDIR/cosa_apps.toml ]; then
        rm $MODDIR/cosa_apps.toml
        ln -s /storage/emulated/0/Android/cosa_apps.toml $MODDIR/cosa_apps.toml
    fi
fi

killall -15 cosa_helper; rm $LOG
chmod +x ${0%/*}/cosa_helper
RUST_BACKTRACE=1 nohup $MODDIR/cosa_helper >$LOG 2>&1 &
