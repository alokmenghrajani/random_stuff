<?php

// $prog = [
//   "cpy 41 a",
//   "inc a",
//   "inc a",
//   "dec a",
//   "jnz a 2",
//   "dec a",
//   "done"
// ];

$prog = [
"cpy 1 a",
"cpy 1 b",
"cpy 26 d",
"jnz c 2",
"jnz 1 5",
"cpy 7 c",
"inc d",
"dec c",
"jnz c -2",
"cpy a c",
"inc a",
"dec b",
"jnz b -2",
"cpy c b",
"dec d",
"jnz d -6",
"cpy 17 c",
"cpy 18 d",
"inc a",
"dec d",
"jnz d -2",
"dec c",
"jnz c -5",
"done"];

$reg = [
  'a' => 0,
  'b' => 0,
  'c' => 1,
  'd' => 0,
];

$ip = 0;
while ($prog[$ip] != 'done') {
  echo "ip: ", $ip, "\n";
  print_r($reg);
  $i = $prog[$ip];
  if (preg_match('/cpy (\d+) ([a-d])/', $i, $match)) {
    $reg[$match[2]] = $match[1];
    $ip++;
    continue;
  }
  if (preg_match('/cpy ([a-d]) ([a-d])/', $i, $match)) {
    $reg[$match[2]] = $reg[$match[1]];
    $ip++;
    continue;
  }
  if (preg_match('/jnz ([a-d]) (\d+)/', $i, $match)) {
    if ($reg[$match[1]] == 0) {
      $ip++;
    } else {
      $ip += $match[2];
    }
    continue;
  }
  if (preg_match('/jnz ([a-d]) -(\d+)/', $i, $match)) {
    if ($reg[$match[1]] == 0) {
      $ip++;
    } else {
      $ip -= $match[2];
    }
    continue;
  }

  if (preg_match('/jnz (\d+) (\d+)/', $i, $match)) {
    if ($match[1] == 0) {
      $ip++;
    } else {
      $ip += $match[2];
    }
    continue;
  }
  if (preg_match('/inc ([a-d])/', $i, $match)) {
    $reg[$match[1]]++;
    $ip++;
    continue;
  }
  if (preg_match('/dec ([a-d])/', $i, $match)) {
    $reg[$match[1]]--;
    $ip++;
    continue;
  }
  echo "unknown command: ", $i;
  die;
}
print_r($reg);
