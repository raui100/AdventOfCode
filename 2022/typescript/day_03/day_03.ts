const text: string = await Deno.readTextFile("03.txt", "utf8");

function char_to_num(char: string): number {
	let code: number = char.charCodeAt(0);
	if (code >= 97) { return code - 96 } else { return code - 38 }

  }

function part_a(line: string): number {
	let half = line.length / 2
	for (let char_0 of line.slice(0, half)) {
		for (let char_1 of line.slice(half)) {
			if (char_0 === char_1) {
				return char_to_num(char_0)
			}
		}
	}
	console.log("ERROR!")
	return 0;
}

function part_b(strings: [string, string, string]): number {
	for (let char_0 of strings[0]) {
		for (let char_1 of strings[1]) {
			if (char_0 === char_1) {
				for (let char_2 of strings[2]) {
					if (char_0 === char_2) {
						return char_to_num(char_0)
					}
				}
			}
			
		}
	}
	console.log("ERROR!")
	return 0;
}

var score_a = 0
var score_b = 0
var strings: [string, string, string] = ["", "", ""]

text.split("\n").forEach((line, ind) => {
	score_a += part_a(line)
	let ind_mod = ind % 3
	strings[ind_mod] = line
	if (ind_mod === 2) {
		score_b += part_b(strings)
	}
});

console.log(score_a)
console.log(score_b)