<?php
ini_set('memory_limit','3G');

class Elf {
  public $id;
  public $next = null;

  public function __construct($v) {
    $this->id = $v;
  }
}

$size = 3005290;
//$size = 5;
$i = 0;
$first = new Elf($i+1);
$prev = $first;
for ($i=1; $i<$size; $i++) {
  $t = new Elf($i+1);
  $prev->next = $t;
  $prev = $t;
}
$t->next = $first;

$c = $first;
$left = $size;
while (true) {
  $to_walk = floor($left / 2);
  $o = $c;
  $before = null;
  for ($i=0; $i<$to_walk; $i++) {
    $before = $o;
    $o = $o->next;
    if ($o->id == $c->id) {
      $o = $o->next;
    }
    //echo "o: ", $o->id, "\n";
  }
  echo "elf: ", $c->id, " steals from ", $o->id, "\n";

  // remove $o
  $before->next = $o->next;

  //echo "here: ", $o->prev->id, "\n";
//  $o->prev->next = $o->next;
//  $o->next->prev = $o->prev;

  $left--;
  if ($left == 1) {
    break;
  }
  $c = $c->next;
}
echo "done: ";
echo $c->id;
//print_r($c);
