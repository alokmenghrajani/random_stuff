<?php

$cache = [];

function m($s) {
  global $cache;
  if (isset($cache[$s])) {
    return $cache[$s];
  }
  $r = $s;
  for ($i=0; $i<2017; $i++) {
    $r = md5($r);
  }
  $cache[$s] = $r;
  return $r;
}

$i = 0;
$salt ='qzyelonm';
//$salt = 'abc';

echo m('abc0'), "\n";


$found = 0;
while (true) {
    $x = m($salt . $i);
    for ($j=0; $j<30; $j++) {
      if (($x[$j] == $x[$j+1]) &&
        ($x[$j] == $x[$j+2])) {
          // check for 5
          //echo "checking: ", $i, "->", $x[$j], " ", $j, " ", $x, "\n";
          $t = check($i+1, $x[$j]);
          if ($t > 0) {
            $found++;
            echo "found! ", $found, " ", $i, " char: ", $x[$j], " ", $x, " at ", $t, "\n";
            if ($found == 64) {
              die("done");
            }
            break;
          } else {
            break;
          }
        }
    }
    $i++;
    //if ($i == 100) { break; }
}

function check($offset, $char) {
  global $salt;
  for ($k=0; $k<1000; $k++) {
    $x = m($salt . ($offset + $k));
    for ($j=0; $j<28; $j++) {
      if (
        ($x[$j] == $char) &&
        ($x[$j] == $x[$j+1]) &&
      ($x[$j] == $x[$j+2]) &&
      ($x[$j] == $x[$j+3]) &&
      ($x[$j] == $x[$j+4])
    ) {
        return $k + $offset;
      }
    }
  }
  return 0;
}
