: led-on 7 3 SYS 0 DROP 7 1 SYS 1 DROP ;

: led-off 7 3 SYS 0 DROP 7 0 SYS 1 DROP ;

: led-blink led-on 500 SYS 34 led-off 500 SYS 34 ;

: led-blink-fast led-on 100 SYS 34 led-off 100 SYS 34 ;

: led-blink-slow led-on 1000 SYS 34 led-off 1000 SYS 34 ;
