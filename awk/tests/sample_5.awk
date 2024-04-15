BEGIN {
    print "Processing started..."
    total_lines = 12
}

{
    total_lines++
}

END {
    print "Processing finished."
    print "Total lines processed:", total_lines
}
