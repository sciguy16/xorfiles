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

begin
  while true do
    print f1.readpartial(2048).bytes
      .zip(f2.readpartial(2048).bytes)
      .select{ |x, y| !(x.nil? || y.nil?)}
      .map { |x, y| x ^ y }
      .map(&:chr)
      .join("")
  end
rescue EOFError
end

# Close the file descriptors
f1.close
f2.close
