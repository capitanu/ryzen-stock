#!/bin/sh

export DISPLAY=:0
pid=$(pgrep -u calin xmonad | head -n 1)
dbus=$(grep -z DBUS_SESSION_BUS_ADDRESS /proc/$pid/environ | sed 's/DBUS_SESSION_BUS_ADDRESS=//' )
export DBUS_SESSION_BUS_ADDRESS=$dbus

RESULT1=$(ryzen inet)
RESULT2=$(cat /home/calin/repos/github.com/capitanu/ryzen/ryzen_inet.txt)

if [ "$RESULT1" != "$RESULT2" ]
then
    /home/calin/.local/bin/ryzen inet > /home/calin/repos/github.com/capitanu/ryzen/ryzen_inet.txt &&
    /usr/bin/dunstify -u 2 "$RESULT1"
fi

RESULT3=$(ryzen proshop)
RESULT4=$(cat /home/calin/repos/github.com/capitanu/ryzen/ryzen_proshop.txt)

if [ "$RESULT3" != "$RESULT4" ]
then
    /home/calin/.local/bin/ryzen proshop > /home/calin/repos/github.com/capitanu/ryzen/ryzen_proshop.txt;
    dunstify -u 2 --timeout=10000 "$RESULT3"
fi

RESULT5=$(ryzen komplett)
RESULT6=$(cat /home/calin/repos/github.com/capitanu/ryzen/ryzen_komplett.txt)

if [ "$RESULT5" != "$RESULT6" ]
then
    /home/calin/.local/bin/ryzen komplett > /home/calin/repos/github.com/capitanu/ryzen/ryzen_komplett.txt;
    dunstify -u 2 --timeout=10000 "$RESULT5"
fi

