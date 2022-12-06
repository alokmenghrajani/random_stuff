<?php
ini_set('memory_limit','500M');
gc_disable();

$step = 354;
$pos = 0;
$last = 0;
for ($i=1; $i<=50000000; $i++) {
  $pos = ($pos + $step) % $i;
  if ($pos == 0) {
    $last = $i;
  }
  $pos++;
}
echo "solution: ", $last, "\n";
