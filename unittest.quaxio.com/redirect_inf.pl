if ($ENV{"HTTPS"}) {
  print("Location: https://unittest.quaxio.com/redirect_inf.pl\n\n");
} else {
  print("Location: http://unittest.quaxio.com/redirect_inf.pl\n\n");
}

