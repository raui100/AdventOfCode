from typing import NamedTuple, List
from enum import Enum


class Operation(Enum):
    """Models all possible operation"""
    JUMP = "jmp"
    ACCUMULATE = "acc"
    NO_OPERATION = "nop"


class Instruction(NamedTuple):
    """Couples an operation with an argument. E.g: 'jmp -1'"""
    operation: Operation
    argument: int

    is_jmp = property(lambda self: self.operation == Operation.JUMP)
    is_acc = property(lambda self: self.operation == Operation.ACCUMULATE)
    is_nop = property(lambda self: self.operation == Operation.NO_OPERATION)


def parse_instruction(line: str) -> Instruction:
    """Parses an string to an Instruction. E.g: 'jmp -1' -> Instruction(<Operation.JUMP: 'jmp'>, -1)"""
    operation = Operation(line[:3])
    argument = int(line[4:])

    return Instruction(operation, argument)


if __name__ == "__main__":
    from lib.paths import get_day
    data = get_day(8).read_text().splitlines()
    instructions: List[Instruction] = [parse_instruction(line) for line in data]

    visited_lines: List[int] = []  # Stores the lines that had been visited before
    accumulator = 0  # Stores the count of the accumulator
    int_instruction: int = 0  # Stores which instruction is executed next

    while True:
        if int_instruction in visited_lines:
            break

        visited_lines.append(int_instruction)  # Storing the line number of the executed instruction
        instruction = instructions[int_instruction]  # Fetching the instruction that is executed next

        if instruction.is_jmp:
            int_instruction += instruction.argument

        elif instruction.is_acc:
            accumulator += instruction.argument
            int_instruction += 1

        elif instruction.is_nop:
            int_instruction += 1

        else:
            raise NotImplementedError(f"Please implement the operation: {Instruction.operation}")

    print(accumulator)
