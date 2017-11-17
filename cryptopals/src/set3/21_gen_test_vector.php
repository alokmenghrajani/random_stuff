<?php

mt_srand(1234, MT_RAND_MT19937);
$lines = "";
for ($i=0; $i<10000; $i++) {
  $lines .= mt_rand() . "\n";
}
echo "saving to 21.test_vector\n";
file_put_contents('21.test_vector', $lines);
