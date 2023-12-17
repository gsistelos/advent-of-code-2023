const path = require("path")
const fs = require("fs")

function hasChar(string, pos) {
    if (pos > 0 && pos < string.length && string[pos] != ".") {
        return true
    }
    return false
}

function hasCharInRange(string, start_pos, end_pos) {
    for (let i = start_pos; i <= end_pos; i++) {
        if (hasChar(string, i)) {
            return true
        }
    }
    return false
}

function hasCharSurrounding(lines, y, start_pos, end_pos) {
    start_pos--;
    end_pos++;

    if (y > 0) {
        if (hasCharInRange(lines[y - 1], start_pos, end_pos)) {
            return true
        }
    }

    if (hasChar(lines[y], start_pos) || hasChar(lines[y], end_pos)) {
        return true
    }

    if (y < lines.length - 1) {
        if (hasCharInRange(lines[y + 1], start_pos, end_pos)) {
            return true
        }
    }
    return false
}

if (process.argv.length != 3) {
    console.log("Usage: nodejs", path.relative(process.cwd(), process.argv[1]), "<filename>")
    process.exit(1)
}

try {
    const lines = fs.readFileSync(process.argv[2], "utf8").split("\n")

    let numbers = []

    for (let y = 0; y < lines.length; y++) {
        const line = lines[y]

        for (let x = 0; x < line.length; x++) {
            if (/[0-9]/.test(line[x])) {
                const first_digit_pos = x
                while (/[0-9]/.test(line[x])) {
                    x++
                }
                const last_digit_pos = x - 1 // -1 because x is at the first non-digit character

                if (hasCharSurrounding(lines, y, first_digit_pos, last_digit_pos)) {
                    numbers.push(Number(line.substring(first_digit_pos, last_digit_pos + 1))) // +1 because substring is exclusive
                }
            }
        }
    }

    sum = 0
    for (let i = 0; i < numbers.length; i++) {
        sum += numbers[i]
    }
    console.log(sum)
} catch (e) {
    console.log(e.message);
}
