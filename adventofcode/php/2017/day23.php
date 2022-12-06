<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

/*
snd X plays a sound with a frequency equal to the value of X.
set X Y sets register X to the value of Y.
add X Y increases register X by the value of Y.
mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
*/

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day18.txt'));

// $lines = "snd 1
// snd 2
// snd p
// rcv a
// rcv b
// rcv c
// rcv d";

$lines = explode("\n", $lines);

$prog = [];
foreach ($lines as $k => $line) {
  $matches = [];
  if (preg_match('/^(snd|rcv) ([a-z])$/', $line, $matches)) {
    $prog[] = [$matches[1], $matches[2]];
    continue;
  }
  if (preg_match('/^snd ([0-9-]+)$/', $line, $matches)) {
    $prog[] = ['sndi', $matches[1]];
    continue;
  }
  if (preg_match('/^(set|add|mul|mod|jgz|sub) ([a-z]) ([0-9-]+)$/', $line, $matches)) {
    $prog[] = [$matches[1] . 'i', $matches[2], $matches[3]];
    continue;
  }
  if (preg_match('/^(set|add|mul|mod|jgz|sub) ([a-z]) ([a-z])$/', $line, $matches)) {
    $prog[] = [$matches[1], $matches[2], $matches[3]];
    continue;
  }
  if (preg_match('/^(jgz) ([0-9-]+) ([0-9-]+)$/', $line, $matches)) {
    $prog[] = [$matches[1] . 'ii', $matches[2], $matches[3]];
    continue;
  }
  echo $line, "\n";
  die('parse error');
}

$pcs = [0, 0];
$queue = [[], []];
$regs = [[], []];
$send_counter = 0;
$dead = [0, 0];

$regs[0]['p'] = 0;
$regs[1]['p'] = 1;

while (1) {
  $pcs[0] = process(0);
  $pcs[1] = process(1);
}

$mul_counter = 0;

function process($id) {
  global $pcs, $prog, $sound, $regs, $send_counter, $dead, $queue;

  $pc = $pcs[$id];
  $reg = &$regs[$id];

  $dead[$id] = 0;
  if (($pc < 0) || ($pc >= count($prog))) {
    echo "prog ", $id, "is done";
    $dead[$id] = 1;
    if ($dead[1 - $id]) {
      echo $mul_counter, "\n";
      die("deadlock");
    }
    return $pc;
  }

  echo $id, " processing: ", $prog[$pc][0], "\n";

  if ($prog[$pc][0] == 'snd') {
    $queue[1-$id][] = idx($reg, $prog[$pc][1], 0);
    if ($id == 1) {
      $send_counter++;
      echo "send counter: ", $send_counter, "\n";
    }
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'sndi') {
    $queue[1-$id][] = $prog[$pc][1];
    if ($id == 1) {
      $send_counter++;
      echo "send counter: ", $send_counter, "\n";
    }
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'mod') {
    $r1 = idx($reg, $prog[$pc][1], 0);
    $r2 = idx($reg, $prog[$pc][2], 0);
    $reg[$prog[$pc][1]] = $r1 % $r2;
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'add') {
    $r1 = idx($reg, $prog[$pc][1], 0);
    $r2 = idx($reg, $prog[$pc][2], 0);
    $reg[$prog[$pc][1]] = $r1 + $r2;
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'sub') {
    $r1 = idx($reg, $prog[$pc][1], 0);
    $r2 = idx($reg, $prog[$pc][2], 0);
    $reg[$prog[$pc][1]] = $r1 - $r2;
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'set') {
    $r1 = idx($reg, $prog[$pc][1], 0);
    $r2 = idx($reg, $prog[$pc][2], 0);
    $reg[$prog[$pc][1]] = $r2;
    return $pc + 1;
  }

  if ($prog[$pc][0] == 'seti') {
    $reg[$prog[$pc][1]] = $prog[$pc][2];
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'muli') {
    $reg[$prog[$pc][1]] = idx($reg, $prog[$pc][1], 0) * $prog[$pc][2];
    $mul_counter++;
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'modi') {
    $reg[$prog[$pc][1]] = idx($reg, $prog[$pc][1], 0) % $prog[$pc][2];
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'addi') {
    $reg[$prog[$pc][1]] = idx($reg, $prog[$pc][1], 0) + $prog[$pc][2];
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'rcv') {
    if (count($queue[$id]) == 0) {
      echo $id, " waiting ", $dead[0], " ", $dead[1], "\n";
      $dead[$id] = 1;
      if ($dead[1 - $id]) {
        echo $mul_counter, "\n";
        die("deadlock");
      }
      return $pc;
    }
    $reg[$prog[$pc][1]] = array_shift($queue[$id]);
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'jgz') {
    $v = idx($reg, $prog[$pc][1], 0);
    if ($v > 0) {
      return $pc + idx($reg, $prog[$pc][2], 0);
    }
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'jgzi') {
    $v = idx($reg, $prog[$pc][1], 0);
    if ($v > 0) {
      return $pc + $prog[$pc][2];
    }
    return $pc + 1;
  }
  if ($prog[$pc][0] == 'jgzii') {
    $v = $prog[$pc][1];
    if ($v > 0) {
      return $pc + $prog[$pc][2];
    }
    return $pc + 1;
  }

  print_r($prog[$pc]);
  echo $pc;
  die("unimpl");
}
