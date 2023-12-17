const path = require("path")
const fs = require("fs")

function hasChar(string, pos) {
    if (pos > 0 && pos < string.length && string[pos] == "*") {
        return true
    }
    return false
}

function hasCharInRange(string, start_pos, end_pos) {
    for (let i = start_pos; i <= end_pos; i++) {
        if (hasChar(string, i)) {
            return i
        }
    }
    return -1
}

function hasCharSurrounding(lines, y, start_pos, end_pos) {
    start_pos--;
    end_pos++;

    if (y > 0) {
        x = hasCharInRange(lines[y - 1], start_pos, end_pos)
        if (x != -1) {
            return [y - 1, x]
        }
    }

    if (hasChar(lines[y], start_pos)) {
        return [y, start_pos]
    }

    if (hasChar(lines[y], end_pos)) {
        return [y, end_pos]
    }

    if (y < lines.length - 1) {
        x = hasCharInRange(lines[y + 1], start_pos, end_pos)
        if (x != -1) {
            return [y + 1, x]
        }
    }
    return null
}

if (process.argv.length != 3) {
    console.log("Usage: nodejs", path.relative(process.cwd(), process.argv[1]), "<filename>")
    process.exit(1)
}

try {
    const lines = fs.readFileSync(process.argv[2], "utf8").split("\n")

    let numbersMap = {}

    for (let y = 0; y < lines.length; y++) {
        const line = lines[y]

        for (let x = 0; x < line.length; x++) {
            if (/[0-9]/.test(line[x])) {
                const first_digit_pos = x
                while (/[0-9]/.test(line[x])) {
                    x++
                }
                const last_digit_pos = x - 1 // -1 because x is at the first non-digit character

                const pos = hasCharSurrounding(lines, y, first_digit_pos, last_digit_pos)
                if (pos != null) {
                    const pos_y = pos[0]
                    const pos_x = pos[1]
                    if (numbersMap[pos_y] == undefined) {
                        numbersMap[pos_y] = {}
                    }
                    if (numbersMap[pos_y][pos_x] == undefined) {
                        numbersMap[pos_y][pos_x] = []
                    }
                    numbersMap[pos_y][pos_x].push(Number(line.substring(first_digit_pos, last_digit_pos + 1))) // +1 because substring is exclusive
                }
            }
        }
    }

    let sum = 0
    for (let y in numbersMap) {
        for (let x in numbersMap[y]) {
            if (numbersMap[y][x].length == 2) {
                sum += numbersMap[y][x][0] * numbersMap[y][x][1]
            }
        }
    }
    console.log(sum)
} catch (e) {
    console.log(e);
}
