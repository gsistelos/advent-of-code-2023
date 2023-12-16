import sys


class Round:
    def __init__(self):
        self.red = -1
        self.green = -1
        self.blue = -1

    def set_red(self, red):
        if self.red == -1:
            self.red = red
        else:
            raise Exception("Duplicate red value")

    def set_green(self, green):
        if self.green == -1:
            self.green = green
        else:
            raise Exception("Duplicate green value")

    def set_blue(self, blue):
        if self.blue == -1:
            self.blue = blue
        else:
            raise Exception("Duplicate blue value")

    def set_color(self, value, color):
        if color == "red":
            self.set_red(value)
        elif color == "green":
            self.set_green(value)
        elif color == "blue":
            self.set_blue(value)
        else:
            raise Exception("Invalid color: " + color)

    def is_valid(self):
        return (self.red < 13 and self.green < 14 and self.blue < 15)

    def __str__(self):
        return (str(self.red) + ", " +
                str(self.green) + ", " +
                str(self.blue))


class Game:
    def __init__(self, id):
        self.id = id
        self.rounds = []

    # Receives a list of words representing the rounds of the game
    # E.g. ["13", "red,", "14", "green,", "15", "blue;", "13", "red," ...]
    def set_rounds(self, words):
        i = 0
        while True:
            # Iteration for each round
            round = Round()

            while True:
                # Iteration for each value and color pair
                value = int(words[i])
                i += 1

                color = words[i]
                i += 1

                if i == len(words):
                    is_last = True
                    round.set_color(value, color)
                else:
                    is_last = False
                    round.set_color(value, color[:-1])

                self.rounds.append(round)

                if is_last:
                    break
                elif color[-1] == ";":
                    break
                elif color[-1] != ",":
                    raise Exception("Missing comma after '" + color + "'")

            if is_last:
                break

    def is_valid(self):
        for round in self.rounds:
            if not round.is_valid():
                return False
        return True

    # Returns the highest red, green and blue values in the game
    def highest_colors(self):
        highest_red = 0
        highest_green = 0
        highest_blue = 0
        for round in self.rounds:
            if round.red > highest_red:
                highest_red = round.red
            if round.green > highest_green:
                highest_green = round.green
            if round.blue > highest_blue:
                highest_blue = round.blue
        return highest_red, highest_green, highest_blue

    def __str__(self):
        return (str(self.id) + ": " +
                str(self.rounds[0]) + ", " +
                str(self.rounds[1]) + ", " +
                str(self.rounds[2]) + ": " +
                "valid: " + str(self.is_valid()))


if len(sys.argv) != 2:
    print("Usage: python3", sys.argv[0], "<filename>")
    exit(0)

try:
    with open(sys.argv[1], "r") as file:
        lines = file.readlines()

    games = []
    for line in lines:
        words = line.split()

        if words[0] != "Game":
            raise Exception("Invalid token: '" + words[0] + "'")

        game = Game(int(words[1][:-1]))
        if words[1][-1] != ":":
            raise Exception("Missing colon after '" + words[1] + "'")

        game.set_rounds(words[2:])
        games.append(game)

    ids_sum = 0
    power_sum = 0
    for game in games:
        # print(game)
        if game.is_valid():
            ids_sum += game.id

        highest_red, highest_green, highest_blue = game.highest_colors()
        power_sum += highest_red * highest_green * highest_blue

    print("Part one:", ids_sum)
    print("Part two:", power_sum)


except Exception as e:
    print(e)
