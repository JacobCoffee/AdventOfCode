#!/bin/python3

choice_map = {"A": "Rock", "B": "Paper", "C": "Scissors", "X": "Lose", "Y": "Draw", "Z": "Win"}
choice_points = {"Rock": 1, "Paper": 2, "Scissors": 3}
round_points = {"Win": 6, "Lose": 0, "Draw": 3}
my_points = []


def round_outcome(round_choice):
    elf_choice = choice_map[round_choice[0]]
    goal = choice_map[round_choice[2]]

    if (elf_choice, goal) in [("Rock", "Draw"), ("Paper", "Lose"), ("Scissors", "Win")]:
        return round_points[goal] + choice_points["Rock"]
    elif (elf_choice, goal) in [("Rock", "Win"), ("Paper", "Draw"), ("Scissors", "Lose")]:
        return round_points[goal] + choice_points["Paper"]
    else:
        return round_points[goal] + choice_points["Scissors"]


def game_results():
    with open("../../Data/Day_2/input.data") as f:
        lines = f.readlines()
        rounds = [x.strip() for x in lines]
    print(f"You scored {sum([round_outcome(round_string) for round_string in rounds])} points")


if __name__ == "__main__":
    game_results()
