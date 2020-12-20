use strict;
use warnings;


open(my $in, "<", "/d/temp/input.txt") or die "Failed to source input file"; 
my @lines;
my $line;
my @split;
my @formattedLine;

while (<$in>) {
  @split=split(" ",$_);
  $split[1] =~ s/://;
  @formattedLine = (split('-',$split[0]), @split[1..$#split]);
  push(@lines, "$formattedLine[0],$formattedLine[1],$formattedLine[2],$formattedLine[3]");
}

my $i;
my $j=0;
foreach (@lines) {
  @split = split(",",$_);
  $i = 0;
  for (split //, $split[3]) {
    $i++ if $_ eq $split[2];
  }
  $j++ if $split[0]<=$i<=$split[1];
}
my $temp = @lines;
print"There are $temp passwords and $j of them are valid\n"
