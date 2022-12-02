const text: string = await Deno.readTextFile("./02.txt", "utf8");
const SCORE: Record<string, [number, number]>= {
	"A X": [4, 3],
	"A Y": [8, 4],
	"A Z": [3, 8],
	"B X": [1, 1],
	"B Y": [5, 5],
	"B Z": [9, 9],
	"C X": [7, 2],
	"C Y": [2, 6],
	"C Z": [6, 7],
};

console.log(text.split("\n").reduce((acc, curr) => acc + SCORE[curr][0], 0))  // 10941
console.log(text.split("\n").reduce((acc, curr) => acc + SCORE[curr][1], 0))  // 13071