<?php
ini_set('memory_limit','50000M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day12.txt'));
$lines = explode("\n", $lines);

$a = [];
foreach ($lines as $k => $line) {
  echo $line, "\n";
  $matches = [];
  if (preg_match('/^([0-9]+) <-> (.*)/', $line, $matches)) {
    array_shift($matches);
    $t = [];
    $t[] = $matches[0];
    foreach (explode(",", $matches[1]) as $v) {
      $t[] = $v + 0;
    }
    foreach ($t as $p1) {
      foreach ($t as $p2) {
        $a[$p1][$p2] = true;
        $a[$p2][$p1] = true;
      }
    }
  }
}

// recursively mark things
$n = 0;
while (count($a) > 0) {
  $n++;
  $processed = [];
  mark(array_keys($a)[0]);
  foreach ($processed as $p => $_) {
    unset($a[$p]);
  }
}
echo $n;

function mark($n) {
  global $a, $processed;
  if (isset($processed[$n])) {
    return;
  }
  $processed[$n] = true;
  $t = array_keys($a[$n]);
  foreach ($t as $p) {
    mark($p);
  }
}
