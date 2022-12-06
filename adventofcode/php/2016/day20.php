<?php
ini_set('memory_limit','3G');

function process($input) {
  $i = 0;
  $lines = split("\n", $input);
  $total = 0;
  while (true) {
    //echo "checking: ", $i, "\n";
    $found = true;
    foreach ($lines as $k => $v) {
      $t = split("-", $v);
      if ($i >= $t[0] && $i <= $t[1]) {
        //echo "here: ", $i, " ", $t[0], "xxx", $t[1], "\n";
        $found = false;
        $i = $t[1]+1;
        break;
      }
    }
    if ($found) {
      echo $i, " ", ++$total, "\n";
      $i++;
      if ($i >= 4294967295) {
        break;
      }
    }
  }
}

$lines = trim(file_get_contents('day20.txt'));
process($lines);
