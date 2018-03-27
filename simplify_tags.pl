#!/usr/bin/perl

while ($radek = <STDIN>) {
    if ($radek =~ /<MMt[^>]*>.{14}8/) {
	$radek =~ s/(<MMt[^>]*>..).{12}8/$1------------8/;
    }
    if ($radek =~ /<MMt[^>]*>P[67]/) {
	$radek =~ s/(<MMt[^>]*>)P7-.([34]---)/$1P7--$2/g;
	$radek =~ s/(<MMt[^>]*>)P6-.([23467])/$1P6--$2/g;
    }
    elsif ($radek =~ /<MMt[^>]*>P[1S]/) {
	$radek =~ s/(<MMt[^>]*>P[1S]...)[XZMIN]([SP]3)/$1-$2/g;
	$radek =~ s/(<MMt[^>]*>P[1S]...)F(P3)/$1-$2/g;
    }
    elsif ($radek =~ /<MMt[^>]*>Vs/) {
	$radek =~ s/(<MMt[^>]*>Vs......)[FPRX]/$1-/g;
    }
    elsif ($radek =~ /<MMt[^>]*>Vc/) {
	$radek =~ s/(<MMt[^>]*>Vc.)X(...)3/$1-$2-/g;
    }
    elsif ($radek =~ /<MMt[^>]*>C[nlr]/) {
	$radek =~ s/(>([\-\+])?[0-9][^<]*<MMl[^>]*>[^<]+<MMt[^>]*>)C[nlr]............./$1C=-------------/;
    }
    elsif ($radek =~ /<MMt[^>]*>C}/) {
	$radek =~ s/(<MMt[^>]*>)C}/$1C=/;
    }
    elsif ($radek =~ /<MMt[^>]*>VB-X/) {
	$radek =~ s/(<MMt[^>]*>)VB-X---X......./$1XX-------------/g;
    }
    elsif ($radek =~ /<MMl[^>]*>co</) {
	$radek =~ s/(<MMt[^>]*>P4).../$1---/g;
    }
    print STDOUT "$radek";
}
