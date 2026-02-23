REM ============================================
REM  11 - GW-BASIC Graphics
REM  Learn: LINE, CIRCLE, PSET, DRAW, COLOR
REM ============================================

10 PRINT "GW-BASIC Style Graphics Demo"
20 CLS

30 REM Draw a house using LINE and filled boxes
40 REM House body (filled blue box)
50 COLOR BLUE
60 LINE (100, 50)-(200, 150), BLUE, BF

70 REM Roof (red lines forming triangle)
80 COLOR RED
90 LINE (90, 50)-(150, -20)
100 LINE (150, -20)-(210, 50)
110 LINE (90, 50)-(210, 50)

120 REM Door (brown filled box)
130 COLOR BROWN
140 LINE (135, 80)-(165, 150), BROWN, BF

150 REM Windows (yellow filled boxes)
160 COLOR YELLOW
170 LINE (108, 65)-(130, 85), YELLOW, BF
180 LINE (170, 65)-(192, 85), YELLOW, BF

190 REM Draw a sun
200 COLOR YELLOW
210 CIRCLE (250, -40), 25

220 REM Sun rays using DRAW macro
230 PENUP
240 HOME
250 PENDOWN
260 COLOR GOLD
270 FOR I = 1 TO 8
280    DRAW "U15"
290    DRAW "D15"
300    RIGHT 45
310 NEXT I

320 REM Ground
330 COLOR GREEN
340 LINE (-300, 150)-(300, 170), GREEN, BF

350 REM Flowers using circles and dots
360 COLOR RED
370 CIRCLE (-100, 140), 8
380 COLOR PINK
390 CIRCLE (-60, 130), 6
400 COLOR ORANGE
410 CIRCLE (-140, 135), 7

420 REM Sky dots (stars would be at night)
430 COLOR WHITE
440 PSET (-200, -80)
450 PSET (-180, -100)
460 PSET (-150, -70)
470 PSET (200, -90)
480 PSET (180, -60)

490 PRINT "House scene complete!"
500 END
