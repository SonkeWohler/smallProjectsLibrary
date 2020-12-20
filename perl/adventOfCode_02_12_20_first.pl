use strict;
use warnings;


open(my $in, "<", "/d/temp/input.txt") or die "Failed to source input file"; 
my @lines;
my $line;
my @split;
my @formattedLine;

while (<$in>) {
  ##$line = $_;
  @split=split(" ",$_);
  $split[1] =~ s/://;
  @formattedLine = (split('-',$split[0]), @split[1..$#split]);
  ##print"$line: @formattedLine\n";
  push(@lines, "$formattedLine[0],$formattedLine[1],$formattedLine[2],$formattedLine[3]");
}
print" ------- \nslurped formatted lines\n";

my $i;
my $j=0;
foreach $line (@lines) {
  @split = split(",",$line);
  $i = $split[3] =~ tr/$split[2]//;
  print"$split[3] should contain $split[2] like: $split[0] <= $i <= $split[1]\nthe original line is `$line`\n";
  $j++ if $split[0]<=$i<=$split[1];
}
my $temp = @lines;
print"There are $temp passwords and $j of them are valid\n"
