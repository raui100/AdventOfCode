const data: string = await Deno.readTextFile("07.txt", "utf8");
var tmp: number[] = new Array();
var directories: number[] = new Array();
for (let line of data.split("\n")) {
  if (line.startsWith("$ cd ..")) {
    directories.push(tmp.pop()!)
  } else if (line.startsWith("$ cd")) {
    tmp.push(0)
  } else {
    const size = Number(line.split(" ")[0]);
    if (!isNaN(size)) {
      for (let ind in tmp) {
        tmp[ind] += size
      }
    }
  }
}
directories = directories.concat(tmp)

// Part A: 2031851
var result = directories.filter(num => num < 100_000).reduce((acc, num) => acc + num)
console.log(result)

// Part B: 2568781
const root = directories.reduce((acc, num) => Math.max(acc, num))  // Biggest directory
const required = root + 30_000_000 - 70_000_000;
result = directories.filter((num) => num >= required).reduce((acc, num) => Math.min(acc, num));
console.log(result)
