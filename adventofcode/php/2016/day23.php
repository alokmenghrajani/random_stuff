<?php
ini_set('memory_limit','3G');

$reg = [
  'a' => 12,
  'b' => 0,
  'c' => 0,
  'd' => 0,
];
$prog = [
  ["cpy", "a", "b"], // 0
  ["dec", "b"],      //
  ["cpy", "a", "d"], //
  ["cpy", 0, "a"],
  ["cpy", "b", "c"],
  ["inc", "a"],
  ["dec", "c"],
  ["jnz", "c", -2],
  ["dec", "d"],
  ["jnz", "d", -5],
  ["dec", "b"],
  ["cpy", "b" ,"c"],
  ["cpy", "c","d"],
  ["dec", "d"],
  ["inc", "c"],
  ["jnz", "d", -2],
  ["tgl", "c"],
  ["cpy", -16, "c"],  // 17
  ["jnz", 1, "c"],    // 18
  ["cpy", 76, "c"],   // 19
  ["jnz", 80, "d"],   // 20
  ["inc", "a"],       // 21
  ["inc", "d"],       // 22
  ["jnz", "d", -2],   // 23
  ["inc", "c"],       // 24
  ["jnz", "c", -5],   // 25
];

// $prog = [
// ["cpy", 2, "a"],
// ["tgl", "a"],
// ["tgl", "a"],
// ["tgl", "a"],
// ["cpy", 1, "a"],
// ["dec", "a"],
// ["dec", "a"],
// ];

// $prog = [
//   ["cpy", 41, "a"],
//   ["inc", "a"],
//   ["inc", "a"],
//   ["dec", "a"],
//   ["jnz", "a", 2],
//   ["dec", "a"],
// ];


// $reg = [
//   'a' => 0,
//   'b' => 0,
//   'c' => 1,
//   'd' => 0,
// ];
// $prog = [
// ["cpy", 1, "a"],
// ["cpy", 1, "b"],
// ["cpy", 26, "d"],
// ["jnz", "c", 2],
// ["jnz", 1, 5],
// ["cpy", 7, "c"],
// ["inc", "d"],
// ["dec", "c"],
// ["jnz", "c", -2],
// ["cpy", "a", "c"],
// ["inc", "a"],
// ["dec", "b"],
// ["jnz", "b", -2],
// ["cpy", "c", "b"],
// ["dec", "d"],
// ["jnz", "d", -6],
// ["cpy", 17, "c"],
// ["cpy", 18, "d"],
// ["inc", "a"],
// ["dec", "d"],
// ["jnz", "d", -2],
// ["dec", "c"],
// ["jnz", "c", -5]
// ];

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

$ip = 0;
while (true) {
  if (($ip < 0) || ($ip >= count($prog))) {
    echo "done!\n";
    print_r($reg);
    die;
  }
  $i = &$prog[$ip];
  //echo "instruction: ", $ip, " ", join(" ", $i), " | reg: ", join(",", $reg), "\n";
  if ($i[0] == 'cpy') {
    if (is_reg($i[2])) {
      $reg[$i[2]] = resolve($i[1]);
    }
    $ip++;
    continue;
  }
  if ($i[0] == 'jnz') {
    // optimization time!
    if ($i[2] == -2) {
      if (($prog[$ip-2][0] == 'dec') &&
          ($prog[$ip-1][0] == 'dec') &&
          ($prog[$ip-1][1] == $i[1])) {
            if ($reg[$i[1]] > 0) {
              //echo "taking optimization 1\n";
              $reg[$prog[$ip-2][1]] -= $reg[$i[1]];
              $reg[$i[1]] = 0;
            } else {
              echo "failing optimization 1\n";
            }
          } else if (($prog[$ip-2][0] == 'inc') &&
              ($prog[$ip-1][0] == 'dec') &&
              ($prog[$ip-1][1] == $i[1])) {
                if ($reg[$i[1]] > 0) {
                  //echo "taking optimization 2\n";
                  $reg[$prog[$ip-2][1]] += $reg[$i[1]];
                  $reg[$i[1]] = 0;
                } else {
                  echo "failing optimization 2\n";
                }
              }
    }
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
  if ($i[0] == 'tgl') {
    $t = resolve($i[1]) + $ip;
    if (($t >= 0) && ($t < count($prog))) {
      if (count($prog[$t]) == 2) {
        if ($prog[$t][0] == 'inc') {
          $prog[$t][0] = 'dec';
        } else {
          $prog[$t][0] = 'inc';
        }
      } else {
        if ($prog[$t][0] == 'jnz') {
          $prog[$t][0] = 'cpy';
        } else {
          $prog[$t][0] = 'jnz';
        }
      }
    }
    $ip++;
    continue;
  }
  echo "unknown command: ", print_r($i, 1);
  die;
}
