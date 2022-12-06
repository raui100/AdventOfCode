const data: string = await Deno.readTextFile("06.txt", "utf8");
const solve = (data: string, windowSize: number) => {
    return data.split('').findIndex((_, index) => {
      const charsWindow = data.slice(index, index + windowSize);
  
      return charsWindow.length === new Set(charsWindow).size;
    }) + windowSize
  }

console.log(solve(data, 4))  // 1034
console.log(solve(data, 14))  // 2472