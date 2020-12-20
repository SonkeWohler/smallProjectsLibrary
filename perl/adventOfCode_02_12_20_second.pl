use strict;
##use warnings;


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
close($in);

my $i;
my $j=0;
my $temp;
my $firstChar;
my $firstBool;
my $secondChar;
my $secondBool;
foreach (@lines) {
  $firstChar = "";
  $secondChar = "";
  @formattedLine = split(",",$_);
  @split = split //, $formattedLine[3];
  $firstChar = $split[$formattedLine[0]-1];
  $firstBool = $firstChar eq $formattedLine[2];
  $secondChar = $split[$formattedLine[1]-1];
  $secondBool = $secondChar eq $formattedLine[2];
  $temp = ($firstBool && !$secondBool) || (!$firstBool && $secondBool);
  $j++ if $temp;
  ##print"$firstChar:$formattedLine[0] --- $secondChar:$formattedLine[1] --- $formattedLine[3] --- $formattedLine[2]:$temp \n";
}
$temp = @lines;
print"There are $temp passwords and $j of them are valid\n";
