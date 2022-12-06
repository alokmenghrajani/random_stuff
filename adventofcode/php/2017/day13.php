<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);
$lines = "0: 3
1: 2
4: 4
6: 4";
$lines = trim(file_get_contents('day13.txt'));
$lines = explode("\n", $lines);

$firewall = [];
$scanner = [];
$dir = [];
$max = 7;
$max = 87;
for ($i=0; $i<$max; $i++) {
  $firewall[$i] = 0;
  $scanner[$i] = 0;
  $dir[$i] = 1;
}

foreach ($lines as $k => $line) {
  echo $line, "\n";
  $a = explode(": ", $line);
  $firewall[$a[0] + 0] = $a[1] + 0;
}

//print_r($firewall);

$delay = 0;
while (true) {
  $score = 0;
  $good = true;
  for ($p=0; $p<$max; $p++) {
    if ($firewall[$p] == 0) {
      //echo "skipping ", $p, "\n";
      continue;
    }
    //echo "at: ", $p, "\n";
    $pos = ($delay + $p) % ($firewall[$p] * 2 - 2);
    if ($pos == 0) {
      //echo "caught at: ", $p, "\n";
      $good = false;
      break;
      //$score += $firewall[$p] * $p;
    }
  }
  if ($good) {
    break;
  }
  $delay++;
}

echo $delay, "\n";
echo $score, "\n";
