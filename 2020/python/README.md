# Advent of Code
> https://adventofcode.com/2020
## [Day 1](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day01.py)
**Must read**  
├Overview of [built in functions](https://docs.python.org/3/library/functions.html)  
├[math.prod (product)](https://docs.python.org/3/library/math.html#math.prod)  
├[Slicing & Indexing](https://towardsdatascience.com/the-basics-of-indexing-and-slicing-python-lists-2d12c90a94cf)  
└[list comprehension](https://www.python-kurs.eu/list_comprehension.php)

**Can read**  
├[Pathlib](https://docs.python.org/3/library/pathlib.html)  
├[Exceptions](https://docs.python.org/3/tutorial/errors.html)  
├[math library](https://docs.python.org/3/library/math.html)  
└[Walrus operator](https://realpython.com/lessons/assignment-expressions/)

## [Day 2](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day02.py)
**Must read**  
├[Regex](https://docs.python.org/3/howto/regex.html)  
├[Regex Testing](https://regexr.com/)  
├[Context Manager](https://stackabuse.com/python-context-managers/)  
└[Casting](https://www.w3schools.com/python/python_casting.asp)

## [Day 3](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day03.py)
**Must read**  
└[Modulo](https://www.geeksforgeeks.org/what-is-a-modulo-operator-in-python/)

**Can read**  
├[Named Tuple](https://docs.python.org/3/library/collections.html#collections.namedtuple)  
└[Enumerate](https://docs.python.org/3/library/functions.html?highlight=enumerate#enumerate)

## [Day 4](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day04.py)
You'll have to use regular expressions here.  
**Must read (have been linked before)**  
├[Regex](https://docs.python.org/3/howto/regex.html)  
├[Regex Testing](https://regexr.com/)  
└[Lambda functions](https://www.w3schools.com/python/python_lambda.asp)

<details>
  <summary>Spoiler:I have written this function you can use</summary>
    
  ```python
import re
def re_range(prefix: str, low: int, up: int, postfix: str = ""):
    """Creates a regex for a numeric range"""  # e.g. "birth_year": re_range("byr:", 1920, 2002),
    lst_range = [str(i) for i in range(low, up + 1)]
    str_range = f"({'|'.join(lst_range)})"
    return re.compile(prefix + str_range + postfix)
  ```
</details>

## [Day 5](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day05.py)
**Must read**  
└[Python binary function](https://docs.python.org/3/library/functions.html?highlight=enumerate#bin)  

**Can read**  
└[Zahlendarstellung (DE)](https://www.inf.hs-flensburg.de/lang/informatik/zahlendarstellung.htm)


## [Days 6](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day06.py)
**Must read**  
├[Mengenlehre (DE)](https://www.mathebibel.de/mengenlehre)  
├[Python sets - Docs](https://docs.python.org/3/library/stdtypes.html#set)  
├[Python intersection](https://docs.python.org/3/library/stdtypes.html#frozenset.intersection)  
├[Python tuple](https://docs.python.org/3/library/stdtypes.html#tuple)  
├[Python zip](https://docs.python.org/3/library/functions.html#zip)  
└[Python sets - Tutorial](https://realpython.com/python-sets/)

The tutorial is missing something very crucial though...
```python
a = {1, 2}
b = {2, 3}
lst_sets = [a, b]

# All three statements below have the same result
print(a.intersection(b))  # {2}
print(set.intersection(a, b))  # {2}
print(set.intersection(*tuple(lst_sets)))  # {2}
```

<details>
  <summary>Spoiler:I have written this function you can use</summary>
    
  ```python
from typing import List, Set
import re
def intersecting_entries(group: str) -> int:
    """Counts the number of chars that are in every entry of the list"""
    entries = group.splitlines()  # Splits the group into their members
    entries: List[str] = [re.sub(r"[^a-z]", "", entry) for entry in entries]  # Deletes every char that is not a-z
    entries: List[Set[str]] = [set(entry) for entry in entries]

    return len(set.intersection(*tuple(entries)))
  ```
</details>

## [Day 7](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day07.py)
**Must read**  
├[Recursion & Iteration](https://www.refactoring.com/catalog/replaceRecursionWithIteration.html)  
└[PyTest](https://www.youtube.com/watch?v=fv259R38gqc)

## [Day 8](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day08.py)
**Must read**  
├[Generatoren](https://realpython.com/introduction-to-python-generators/)  
└[Generators will free your mind](https://www.youtube.com/watch?v=RdhoN4VVqq8)

## [Day 11](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day11.py)
> Nothing new here. 
>I have used a string as an array of chars and I'm accessing them via smart usage of modulo

## [Day 12](https://github.com/raui100/AdventOfCode/blob/master/2020/python/src/day12.py)
> I had solved part 1 and instead of adapting the class `Ship` I've implemented an Adapter class
> with the name `Navigator`

**Must read**  
├[PyTest](https://www.youtube.com/watch?v=fv259R38gqc)  
└[Adapter design pattern](https://refactoring.guru/design-patterns/adapter/python/example)

<!-- Design element
## Day 
**Must read** 
├[]()
└[]()
-->
