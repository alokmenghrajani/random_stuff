<?php
ini_set('memory_limit','3G');
$match = [];

function process($input, $pass) {
  $lines = split("\n", $input);
  foreach ($lines as $k => $v) {
    if (preg_match("/swap position (\d+) with position (\d+)/", $v, $match)) {
      $t = $pass[$match[1]];
      $pass[$match[1]] = $pass[$match[2]];
      $pass[$match[2]] = $t;
    } else if (preg_match("/swap letter ([a-z]+) with letter ([a-z]+)/", $v, $match)) {
      for ($i=0; $i<strlen($pass); $i++) {
        if ($pass[$i] == $match[1]) {
          $pass[$i] = $match[2];
        } else if ($pass[$i] == $match[2]) {
          $pass[$i] = $match[1];
        }
      }
    } else if (preg_match("/rotate (left|right) (\d+) steps?/", $v, $match)) {
      for ($j=0; $j<$match[2]; $j++) {
        if ($match[1] == 'left') {
          $t = '';
          for ($i=1; $i<strlen($pass); $i++) {
            $t .= $pass[$i];
          }
          $t .= $pass[0];
        } else {
          $t = $pass[strlen($pass)-1];
          for ($i=0; $i<strlen($pass)-1; $i++) {
            $t .= $pass[$i];
          }
        }
        $pass = $t;
      }
    } else if (preg_match("/rotate based on position of letter ([a-z])/", $v, $match)) {
      $amount = strpos($pass, $match[1]);
      if ($amount >= 4) { $amount++; };
      $amount++;
      for ($j=0; $j<$amount; $j++) {
        $t = $pass[strlen($pass)-1];
        for ($i=0; $i<strlen($pass)-1; $i++) {
          $t .= $pass[$i];
        }
        $pass = $t;
      }
    } else if (preg_match("/reverse positions (\d+) through (\d+)/", $v, $match)) {
      $t = '';
      for ($i=0; $i<$match[1]; $i++) {
        $t .= $pass[$i];
      }
      for ($i=$match[2]; $i>=$match[1]; $i--) {
        $t .= $pass[$i];
      }
      for ($i=$match[2]+1; $i<strlen($pass); $i++) {
        $t .= $pass[$i];
      }
      $pass = $t;
    } else if (preg_match("/move position (\d+) to position (\d+)/", $v, $match)) {
      $t = '';
      for ($i=0; $i<$match[1]; $i++) {
        $t .= $pass[$i];
      }
      for ($i=$match[1]+1; $i<strlen($pass); $i++) {
        $t .= $pass[$i];
      }
      $u = '';
      for ($i=0; $i<strlen($pass); $i++) {
        if ($i == $match[2]) {
          $u .= $pass[$match[1]];
        } else if ($i < $match[2]) {
          $u .= $t[$i];
        } else {
          $u .= $t[$i-1];
        }
      }
      $pass = $u;
    } else {
      echo "x: ", $k, " ", $v, "\n";
      die("here");
    }
  }
  return $pass;
}

//$lines = "swap position 4 with position 0\nswap letter d with letter b\nreverse positions 0 through 4\nrotate left 1 step\nmove position 1 to position 4\nmove position 3 to position 0\nrotate based on position of letter b\nrotate based on position of letter d";
$lines = trim(file_get_contents('day21.txt'));

$visited = [];
$t = 'fbgdceah';
while (true) {
  echo $t, "\n";
  $t2 = process($lines, $t);
  if ($t2 == 'fbgdceah') {
    echo $t, "\n";
    //die("FOUND IT!");
  }
  if (isset($visited[$t2])) {
    die("NOPE");
  }
  $visited[$t2] = true;
  $t = $t2;
}
