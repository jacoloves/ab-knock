<?php
$s = trim(fgets(STDIN));

$mp = [];

for ($i=0; $i<strlen($s); $i++) {
  if (isset($mp[$s[$i]])) {
    $mp[$s[$i]]++;
  } else {
    $mp[$s[$i]] = 1;
  }
}

$max_num = -1;
$max_char = ' ';

foreach ($mp as $char => $num) {
  if ($max_num < $num) {
    $max_char = $char;
    $max_num = $num;
  } elseif ($max_num == $num) {
    if ($max_char > $char) {
      $max_char = $char;
    }
  }
}

echo $max_char . PHP_EOL;
