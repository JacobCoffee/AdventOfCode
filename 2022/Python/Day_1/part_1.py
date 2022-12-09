#!/bin/python3

with open("../../1.data/input.data") as f:
    elf = f.read().split("\n\n")
    elf_calorie_comparator = []

    for calories in elf:
        elf_calorie_comparator.append(sum([int(x) for x in calories.split()]))
    print(max(elf_calorie_comparator))
