#!/usr/bin/env ruby

if ARGV.length != 2 then
  puts "Usage: ./xorfiles FILE1 FILE2"
  exit
end

file1, file2, *_ = ARGV

File.open(file1, 'r') do |f1|
  File.open(file2, 'r') do |f2|
    puts f1.each_byte
      .zip(f2.each_byte)
      .select{ |x, y| !(x.nil? || y.nil?)}
      .map { |x, y| x ^ y }
      .map(&:chr)
      .join("")
  end
end
