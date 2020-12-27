"""Tests the functions in day08"""
from typing import List
import pytest
from pytest import fixture


from src.day08 import (
    Operation,
    Instruction,
    parse_instruction,
    negate_instruction,
    determine_accumulator,
    swapped_instructions,
)


def test_parse_instruction():
    assert parse_instruction("jmp -1") == Instruction(Operation.JUMP, -1)
    assert parse_instruction("nop +1") == Instruction(Operation.NO_OPERATION, 1)
    assert parse_instruction("acc +11") == Instruction(Operation.ACCUMULATE, 11)


def test_negate_instruction():
    fn = negate_instruction
    assert fn(Instruction(Operation.JUMP, 1)) == Instruction(Operation.NO_OPERATION, 1)

    assert fn(Instruction(Operation.NO_OPERATION, 1)) == Instruction(Operation.JUMP, 1)

    assert fn(Instruction(Operation.ACCUMULATE, 1)) == Instruction(
        Operation.ACCUMULATE, 1
    )


@fixture
def instructions_loop() -> List[Instruction]:
    """Returns a list of instructions that lead to an loop"""
    return [Instruction(Operation.JUMP, 0)]


@fixture
def instructions_with_end() -> List[Instruction]:
    """Returns a list of instructions that lead to an accumulator 20"""
    return [
        Instruction(Operation.ACCUMULATE, 10),
        Instruction(Operation.JUMP, 2),
        Instruction(Operation.JUMP, 2),
        Instruction(Operation.JUMP, -1),
        Instruction(Operation.NO_OPERATION, 0),
        Instruction(Operation.ACCUMULATE, 10),
    ]


def test_determine_accumulator(instructions_loop, instructions_with_end):
    assert determine_accumulator(instructions_loop) is None
    assert determine_accumulator(instructions_with_end) == 20


def test_swapped_instructions(instructions_with_end):
    generator_instruction = swapped_instructions(instructions_with_end)

    # Testing 0th iteration. Nothing is being changed
    tmp_instructions = instructions_with_end.copy()
    assert next(generator_instruction) == tmp_instructions

    # Testing 1st iteration. First possible index is being changed
    tmp_instructions = instructions_with_end.copy()
    tmp_instructions[1] = Instruction(Operation.NO_OPERATION, 2)
    assert next(generator_instruction) == tmp_instructions

    # Testing 2nd iteration. Second possible index is being changed
    tmp_instructions = instructions_with_end.copy()
    tmp_instructions[2] = Instruction(Operation.NO_OPERATION, 2)
    assert next(generator_instruction) == tmp_instructions

    # Testing 3rd iteration. Third possible index is being changed
    tmp_instructions = instructions_with_end.copy()
    tmp_instructions[3] = Instruction(Operation.NO_OPERATION, -1)
    assert next(generator_instruction) == tmp_instructions

    # Testing 4th iteration. Fourth possible index is being changed
    tmp_instructions = instructions_with_end.copy()
    tmp_instructions[4] = Instruction(Operation.JUMP, 0)
    assert next(generator_instruction) == tmp_instructions

    with pytest.raises(StopIteration):  # The generator ends after that
        next(generator_instruction)
