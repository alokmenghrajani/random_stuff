<?php

$lines = explode("\n", "ytr inc 5 if xzn > 1
xzn inc 1 if ytr < 5
cpq dec -10 if xzn >= 1
cpq inc -20 if cpq == 10");

$lines = explode("\n", trim(file_get_contents('day08.txt')));
echo '<?php $reg=[]; $max=0; function id($arr, $el) { return isset($arr[$el]) ? $arr[$el] : 0; }', "\n";
echo '$max = 0;';
foreach ($lines as $k => $line) {
  $matches = [];
  if (preg_match('/^([a-z]+) (inc|dec) ([0-9-]+) if ([a-z]+) ([!<=>]+) ([0-9-]+)$/', $line, $matches)) {
    array_shift($matches);
    $op = ($matches[1] == 'dec') ? '-=' : '+=';
    echo 'if (id($reg, ', $matches[3], ') ', $matches[4], ' ', $matches[5], ') { $reg[', $matches[0], ']', $op, $matches[2], '; ';
    echo '$max = max($max, $reg[', $matches[0], ']);';
    echo '}', "\n";
  } else {
    echo "failed: ", $line, "\n";
  }
  echo 'asort($reg); print_r($reg); echo $max;', "\n";
}
echo 'asort($reg); print_r($reg); echo $max;', "\n";
