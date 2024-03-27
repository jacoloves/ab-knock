<?php
$n = fgets(STDIN);
$ans = [];
$query = [];

for  ($i = 0; $i < $n; $i++) {
  fscanf(STDIN, "%d %d", $q, $a);
  if ($q === 1) {
    $query[] = $a;
  } else {
    $tmp_q = array_reverse($query);
    $ans[] = $tmp_q[$a - 1];
  }
}

foreach ($ans as $a) {
  print($a . "\n");
}
