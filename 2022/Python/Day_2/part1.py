#!/bin/python3

choice_map = {"A": "Rock", "B": "Paper", "C": "Scissors", "X": "Rock", "Y": "Paper", "Z": "Scissors"}
choice_points = {"Rock": 1, "Paper": 2, "Scissors": 3}
round_points = {"Win": 6, "Lose": 0, "Draw": 3}
my_points = []


def round_outcome(round_choice):
    elf_choice = choice_map[round_choice[0]]
    my_choice = choice_map[round_choice[2]]

    if elf_choice == my_choice:
        return round_points["Draw"] + choice_points[my_choice]
    elif (elf_choice, my_choice) in [("Rock", "Paper"), ("Paper", "Scissors"), ("Scissors", "Rock")]:
        return round_points["Win"] + choice_points[my_choice]
    else:
        return round_points["Lose"] + choice_points[my_choice]


def game_results():
    with open("../../Data/Day_2/input.data") as f:
        lines = f.readlines()
        rounds = [x.strip() for x in lines]
    print(f"You scored {sum([round_outcome(round_string) for round_string in rounds])} points")


if __name__ == "__main__":
    game_results()
