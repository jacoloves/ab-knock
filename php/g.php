<?php
$n = fgets(STDIN);

$x_sum = 0;
$y_sum = 0;

for ($i = 0; $i < $n; $i++) {
  fscanf(STDIN, "%d %d", $x, $y);

  $x_sum += $x;
  $y_sum += $y;
}

if ($x_sum > $y_sum) {
  echo "Takahashi" . PHP_EOL;
} elseif ($x_sum < $y_sum) {
  echo "Aoki" . PHP_EOL;
} else {
  echo "Draw" . PHP_EOL;
}
