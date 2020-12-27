from typing import NamedTuple, List, Optional, Generator
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


def negate_instruction(instruction: Instruction) -> Instruction:
    """Changes a `jmp` to `nop` or a `nop` to a `jmp`"""
    if instruction.is_jmp:
        return Instruction(Operation.NO_OPERATION, instruction.argument)

    elif instruction.is_acc:
        return instruction

    elif instruction.is_nop:
        return Instruction(Operation.JUMP, instruction.argument)

    else:
        raise NotImplementedError(
            f"Please implement the operation {instruction.operation}"
        )


def determine_accumulator(instructions) -> Optional[int]:
    """Determines accumulator. Returns `None` if there is a loop"""
    visited_lines: List[int] = []  # Stores the lines that had been visited before
    accumulator = 0  # Stores the count of the accumulator
    int_instruction: int = 0  # Stores which instruction is executed next

    while True:
        # Checks if the instruction has been run before
        if int_instruction in visited_lines:

            return None

        # Checks if all instructions have been executed
        if int_instruction == len(instructions):

            return accumulator

        # Fetches the next instruction
        visited_lines.append(
            int_instruction
        )  # Storing the line number of the executed instruction
        instruction = instructions[
            int_instruction
        ]  # Fetching the instruction that is executed next

        # Executes the instruction
        if instruction.is_jmp:
            int_instruction += instruction.argument

        elif instruction.is_acc:
            accumulator += instruction.argument
            int_instruction += 1

        elif instruction.is_nop:
            int_instruction += 1

        else:  # Implement new operations here
            raise NotImplementedError(
                f"Please implement the operation: {Instruction.operation}"
            )


def swapped_instructions(
    instructions: List[Instruction],
) -> Generator[List[Instruction], None, None]:
    """Returns a list of instructions where one entry is swapped"""
    yield instructions  # Yields the original instructions once
    # Indices of the instructions that can be negated
    ind_negated_instructions: List[int] = [
        index
        for index, value in enumerate(instructions)
        if value.is_jmp or value.is_nop
    ]

    for index in ind_negated_instructions:

        tmp_instructions = instructions.copy()
        tmp_instructions[index] = negate_instruction(tmp_instructions[index])

        yield tmp_instructions


if __name__ == "__main__":
    from lib.paths import get_day

    # Reads data from disk
    data: List[str] = get_day(8).read_text().splitlines()

    # Parses instructions
    _instructions: List[Instruction] = [parse_instruction(line) for line in data]

    for _tmp_instructions in swapped_instructions(_instructions):
        _accumulator = determine_accumulator(_tmp_instructions)

        if _accumulator is not None:
            print(_accumulator)
            break
