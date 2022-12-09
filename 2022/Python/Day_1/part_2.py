#!/bin/python3

with open("../../Day_1/input.data") as f:
    elf = f.read().split("\n\n")
    elf_calorie_comparator = []

    for calories in elf:
        elf_calorie_comparator.append(sum([int(x) for x in calories.split()]))
    print(sum(sorted(elf_calorie_comparator, reverse=True)[:3]))
