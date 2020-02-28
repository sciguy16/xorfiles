#!/usr/bin/env ruby

if ARGV.length != 2 then
  puts "Usage: ./xorfiles FILE1 FILE2"
  exit
end

file1, file2, *_ = ARGV

# Get file descriptors for the files
f1desc = IO.sysopen(file1)
f2desc = IO.sysopen(file2)

# Open IO streams for the files
f1 = IO.new(f1desc, 'r')
f2 = IO.new(f2desc, 'r')

BUFSIZE = 2048

# Zip will attempt to assign an element of its argument array to each
# element of the target array, and if there aren't enough the remainder
# will be filled in with nil. This is bad because nil can't be coerced
# into Integer, so we have to first read from the argument array and
# if fewer than BUFSIZE are read in then we only read that much into the
# target array.
#
# This is measurably faster than running a select() on the zip to pull
# out non-nil entries (intuitively).
begin
  while true do
    buf2 = f2.readpartial(BUFSIZE).bytes
    print f1.readpartial([buf2.length, BUFSIZE].min).bytes
      .zip(buf2)
      .map { |x, y| x ^ y }
      .map(&:chr)
      .join("")
  end
rescue EOFError
end

# Close the file descriptors
f1.close
f2.close
