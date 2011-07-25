<?php

$r = new FibSqrt(2);
echo $r->compute(10), "\n";
echo sqrt(2), "\n";

/**
 * Fibonacci's algorithm to compute square roots.
 */
class FibSqrt {
  protected $n;
  protected $r;
  protected $adjustment;

  public function __construct($n) {
    $this->n = $n;
  }

  public function adjust() {
    do {
      if ($this->n > 100) {
        $this->n = $this->n / 100;
        $this->adjustment = $this->adjustment * 10;
        continue;
      }
      if ($this->n < 1) {
        $this->n = $this->n * 100;
        $this->adjustment = $this->adjustment / 10;
        continue;
      }
      break;
    } while (true);
  }

  public function largest() {
    $t = floor($this->n);

    for ($i=0; $i<=10; $i++) {
      $t2 = ($this->r * 20 + $i) * $i;
      if ($t2 > $t) {
        return $i-1;
      }
    }
    throw new Exception('Implementation contains a bug ?!');
  }

  public function compute($precision = 10) {
    if ($this->n <= 0) {
      return 0;
    }

    $this->r = 0;
    $this->adjustment = 10;

    // We need to adjust the initial input to be in the range 0-100
    $this->adjust();

    // now find the largest digit which will fit in floor($this->n)
    for ($i=0; $i<$precision; $i++) {
      $r = $this->largest();

      // now start over with a the new value of n:
      $this->n = ($this->n - (20 * $this->r + $r) *$r) * 100;
      $this->r = $this->r * 10 + $r;
      $this->adjustment = $this->adjustment / 10;
    }

    return $this->r * $this->adjustment;
  }
}
