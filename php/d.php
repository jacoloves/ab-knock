<?php
fscanf(STDIN, "%d %d %d", $h, $w, $n);

$grid = [];
$grid[] = [];

for ($i = 0; $i < $h; $i++) {
  for ($j = 0; $j < $w; $j++) {
    $grid[$i][$j] = '.';
  }
}

$x = 0;
$y = 0;
$dir = 0;

while ($n--) {
  if ($grid[$x][$y] == '.') {
    $grid[$x][$y] = '#';
    $dir = ($dir + 1) % 4;
  } else {
    $grid[$x][$y] = '.';
    $dir = ($dir + 3) % 4;
  }

  if ($dir == 0) {
    $x = ($x - 1 + $h) % $h;
  } elseif ($dir == 1) {
    $y = ($y + 1) % $w;
  } elseif ($dir == 2) {
    $x = ($x + 1) % $h;
  } else {
    $y = ($y - 1 + $w) % $w;
  }
}

for ($i = 0; $i < $h; $i++) {
  for ($j = 0; $j < $w; $j++) {
    print($grid[$i][$j]);
  }
  print("\n");
}

