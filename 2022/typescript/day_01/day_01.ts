const text: string = await Deno.readTextFile("./01.txt");
const sorted: Array<number> = text.split("\n\n")
  .map((group) => group.split("\n").map((n) => Number(n)).reduce((a, b) => a + b))
  .sort((a, b) => b - a);

console.log(sorted[0])  // 75501
console.log(sorted.slice(0, 3).reduce((a, b) => a + b))  // 215594