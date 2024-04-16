function double(x) {
    return x * 2
}

{
    if (($1 % 2) == 0) {
        print "Number is greater than 10";
    } else {
        print "Number is less than or equal to 10";
    }
    print "Double of", $1, "is", double($1)
}
