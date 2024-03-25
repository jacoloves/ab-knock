<?php
fscanf(STDIN, "%d %d %d", $a, $b, $c);

for ($i = $a; $i <= $b; $i += $c) {
  printf("%d ", $i);
}

print("\n");

