<?php
ini_set('memory_limit','3G');

$reg = [
  'a' => 0,
  'b' => 0,
  'c' => 0,
  'd' => 0,
];

$prog = [
  ["cpy", "a", "d"],
  ["cpy", 11, "c"],
  ["cpy", 231, "b"],
  ["inc", "d"],
  ["dec", "b"],
  ["jnz", "b", -2],
  ["dec", "c"],
  ["jnz", "c", -5],
  ["cpy", "d", "a"],
  ["jnz", 0, 0],
  ["cpy", "a", "b"],
  ["cpy", 0, "a"],
  ["cpy", 2, "c"],
  ["jnz", "b", 2],
  ["jnz", 1, 6],
  ["dec", "b"],
  ["dec", "c"],
  ["jnz", "c", -4],
  ["inc", "a"],
  ["jnz", 1, -7],
  ["cpy", 2, "b"],
  ["jnz", "c", 2],
  ["jnz", 1, 4],
  ["dec", "b"],
  ["dec", "c"],
  ["jnz", 1, -4],
  ["jnz", 0, 0],
  ["out", "b"],
  ["jnz", "a", -19],
  ["jnz", 1, -21],
];

function is_reg($v) {
  if (($v == 'a') ||
  ($v == 'b') ||
  ($v == 'c') ||
  ($v == 'd')) {
    return true;
  }
  if (!is_numeric($v)) {
    die("WTF");
  }
  return false;
}

function resolve($v) {
  global $reg;

  if (is_reg($v)) {
    return $reg[$v];
  }
  return $v;
}

function run($a) {
  global $prog, $reg;
  $last = 1;

  $reg['a'] = $a;

  $ip = 0;
  while (true) {
    if (($ip < 0) || ($ip >= count($prog))) {
      echo "program terminated!\n";
      return;
    }
    $i = &$prog[$ip];
    //echo "instruction: ", $ip, " ", join(" ", $i), " | reg: ", join(",", $reg), "\n";
    if ($i[0] == 'out') {
      $t = resolve($i[1]);
      //echo $t, " ";
      if (($last == 1) && ($t != 0)) {
        echo "\n";
        return;
      }
      if (($last == 0) && ($t != 1)) {
        //echo "last: ", $last, ", this: ", $t, "\n";
        echo "\n";
        return;
      }
      $last = $t;
      $ip++;
      continue;
    }
    if ($i[0] == 'cpy') {
      if (is_reg($i[2])) {
        $reg[$i[2]] = resolve($i[1]);
      }
      $ip++;
      continue;
    }
    if ($i[0] == 'jnz') {
      // optimization time!
      // if ($i[2] == -2) {
      //   if (($prog[$ip-2][0] == 'dec') &&
      //       ($prog[$ip-1][0] == 'dec') &&
      //       ($prog[$ip-1][1] == $i[1])) {
      //         if ($reg[$i[1]] > 0) {
      //           //echo "taking optimization 1\n";
      //           $reg[$prog[$ip-2][1]] -= $reg[$i[1]];
      //           $reg[$i[1]] = 0;
      //         } else {
      //           echo "failing optimization 1\n";
      //         }
      //       } else if (($prog[$ip-2][0] == 'inc') &&
      //           ($prog[$ip-1][0] == 'dec') &&
      //           ($prog[$ip-1][1] == $i[1])) {
      //             if ($reg[$i[1]] > 0) {
      //               //echo "taking optimization 2\n";
      //               $reg[$prog[$ip-2][1]] += $reg[$i[1]];
      //               $reg[$i[1]] = 0;
      //             } else {
      //               echo "failing optimization 2\n";
      //             }
      //           }
      // }
      if (resolve($i[1]) == 0) {
        $ip++;
      } else {
        $ip += resolve($i[2]);
      }
      continue;
    }
    if ($i[0] == 'inc') {
      if (is_reg($i[1])) {
        $reg[$i[1]]++;
      }
      $ip++;
      continue;
    }
    if ($i[0] == 'dec') {
      if (is_reg($i[1])) {
        $reg[$i[1]]--;
      }
      $ip++;
      continue;
    }
    // if ($i[0] == 'tgl') {
    //   $t = resolve($i[1]) + $ip;
    //   if (($t >= 0) && ($t < count($prog))) {
    //     if (count($prog[$t]) == 2) {
    //       if ($prog[$t][0] == 'inc') {
    //         $prog[$t][0] = 'dec';
    //       } else {
    //         $prog[$t][0] = 'inc';
    //       }
    //     } else {
    //       if ($prog[$t][0] == 'jnz') {
    //         $prog[$t][0] = 'cpy';
    //       } else {
    //         $prog[$t][0] = 'jnz';
    //       }
    //     }
    //   }
    //   $ip++;
    //   continue;
    // }
    echo "unknown command: ", print_r($i, 1);
    die;
  }
}
for ($a = 180; $a<2000; $a++) {
  echo "running $a\n";
  $t = run($a);
//  echo "result: ", $t ? 'true' : 'false', "\n";
}
echo "failed!\n";
