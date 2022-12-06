<?php
ini_set('memory_limit','500M');
gc_disable();

class Node {
  public $val;
  public $next;

  public function __construct($val) {
    $this->val = $val;
  }

  public function insert($at) {
    $t = $at->next;
    $at->next = $this;
    $this->next = $t;
  }
}

$step = 354;
$a = new Node(0);
$a->next = $a;
$pos = $a;
$start = time();
$zero = $a;

for ($i=1; $i<=2017; $i++) {
  for ($j=0; $j<$step; $j++) {
    $pos = $pos->next;
  }

  // insert at $pos
  $t = new Node($i);
  $t->insert($pos);

  // if ($pos->val == 0) {
  //   echo "new: ", $i, "\n";
  // }

  $pos = $t;
}

echo "we are done! Looking for 2017\n";
$t = $zero;
$n = 0;
while ($t->val != 2017) {
  $t = $t->next;
  $n++;
}
echo "solution: ", $t->next->val;
echo "n=", $n, "\n";
