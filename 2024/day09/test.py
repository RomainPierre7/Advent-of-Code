from typing import Generator, Optional


def create_memory_view_from_disk_map(raw_input: str) -> list:
    result = []
    empty_space = False
    file_id = 0
    for size in raw_input:
        size = int(size)
        if empty_space:
            for _ in range(size):
                result.append(None)
        else:
            for _ in range(size):
                result.append(file_id)
            file_id += 1
        empty_space = not empty_space
    return result


def generate_defragged_memory_block_part_one(
    memory_view: list[int],
) -> Generator[int, None, None]:
    forward_pointer = 0
    backward_pointer = len(memory_view) - 1
    while forward_pointer <= backward_pointer:
        if memory_view[forward_pointer] is not None:
            yield memory_view[forward_pointer]
            forward_pointer += 1
        elif memory_view[backward_pointer] is not None:
            yield memory_view[backward_pointer]
            forward_pointer += 1
            backward_pointer -= 1
        else:
            backward_pointer -= 1


def has_enough_space_for_size(memory_view, size_of_block, start_index):
    for i in range(size_of_block):
        if memory_view[start_index + i] is not None:
            return False
    return True


def create_defragged_memory_block_part_two(memory_view: list[Optional[int]]):
    last_seen_index_with_space_for_size_i: list[int] = [0] * 9
    back_pointer = len(memory_view) - 1
    while back_pointer >= min(last_seen_index_with_space_for_size_i):
        if memory_view[back_pointer] is None:
            back_pointer -= 1
            continue

        end_of_block = back_pointer
        while (
            back_pointer >= 0 and memory_view[back_pointer] == memory_view[end_of_block]
        ):
            back_pointer -= 1
        start_of_block = back_pointer + 1

        size_of_block = end_of_block - start_of_block + 1

        if last_seen_index_with_space_for_size_i[size_of_block - 1] >= start_of_block:
            continue

        found_space = False
        # Find next large-enough empty space
        while last_seen_index_with_space_for_size_i[size_of_block - 1] < start_of_block:
            if not has_enough_space_for_size(
                memory_view,
                size_of_block,
                last_seen_index_with_space_for_size_i[size_of_block - 1],
            ):
                last_seen_index_with_space_for_size_i[size_of_block - 1] += 1
            else:
                found_space = True
                break

        if found_space:
            # Move block to empty space
            for i in range(size_of_block):
                memory_view[
                    last_seen_index_with_space_for_size_i[size_of_block - 1] + i
                ] = memory_view[start_of_block + i]
                memory_view[start_of_block + i] = None
            last_seen_index_with_space_for_size_i[size_of_block - 1] += size_of_block


def calculate_checksum(memory_view: list[int]) -> int:
    return sum(
        [index * value for index, value in enumerate(memory_view) if value is not None]
    )


def part_one(raw_input: str) -> int:
    memory_view = create_memory_view_from_disk_map(raw_input)
    defragged = list(generate_defragged_memory_block_part_one(memory_view))
    return calculate_checksum(defragged)


def part_two(raw_input: str) -> int:
    memory_view = create_memory_view_from_disk_map(raw_input)
    create_defragged_memory_block_part_two(memory_view)
    return calculate_checksum(memory_view)


def main():
    with open("9_input.txt") as f:
        raw_input = f.read()
    print(f"Part One: {part_one(raw_input)}")
    print(f"Part Two: {part_two(raw_input)}")


if __name__ == "__main__":
    main()
