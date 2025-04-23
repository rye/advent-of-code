#!/usr/bin/env python

import os, sys

def part_one(lines):
	return sum(len(line[:-1]) - len(eval(line)) for line in lines)

def part_two(lines):
	return sum(2 + line.count('\\') + line.count('"') for line in lines)

if os.getenv('RYE_AOC_SUBPROC'):
	# Read the lines in from STDIN.
	lines = [line for line in sys.stdin]

	if os.getenv('RYE_AOC_YEAR') == '2015' and os.getenv('RYE_AOC_DAY') == '08':
		if os.getenv('RYE_AOC_PART_ONE') != None:
			print('Part One:', part_one(lines))
		if os.getenv('RYE_AOC_PART_TWO') != None:
			print('Part Two:', part_two(lines))
