<?php
$s = fgets(STDIN);

$s_cnt = 0;

for ($i=0; $i<strlen($s)-1; $i++) {
  if (ctype_upper($s[$i]) && $s_cnt == 0) {
    $s_cnt++;
  } else if (ctype_lower($s[$i]) && $s_cnt != 0) {
    $s_cnt++;
  }
}

if (strlen($s) - 1 == $s_cnt) {
  print("Yes\n");
} else {
  print("No\n");
}
