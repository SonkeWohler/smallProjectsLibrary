use strict;
use warnings;


open(my $in, "<", "/d/temp/input.txt") or die "Failed to source input file"; 
my $right = 3;
##my $down = 1;

my $x = 0;
my @line;
my $i = 0;
while (<$in>) {
  @line = split //, $_;
  if ($line[$x] eq "#") {
    $line[$x]="X";
    $i++;
  } else {
    $line[$x]="O";
  }
  ##print"@line";
  $x += $right;
  $x = $x % $#line;
  ##print" $x \n";
}
print"You encountered $i trees";
