D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sync\mpmc\array.rs

## initial:

head: {lap: 0, mark: 0, index: 0}
tail: {lap: 0, mark: 0, index: 0}
buffer:
[
    {lap: 0, mark: 0, index: 0},
    {lap: 0, mark: 0, index: 1},
    {lap: 0, mark: 0, index: 2},
    {lap: 0, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 4}
]

## send:

head: {lap: 0, mark: 0, index: 0}
tail: {lap: 0, mark: 0, index: 1}
buffer:
[
    {lap: 0, mark: 0, index: 1},
    {lap: 0, mark: 0, index: 1},
    {lap: 0, mark: 0, index: 2},
    {lap: 0, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 4}
]

## send:

head: {lap: 0, mark: 0, index: 0}
tail: {lap: 0, mark: 0, index: 2}
buffer:
[
    {lap: 0, mark: 0, index: 1},
    {lap: 0, mark: 0, index: 2},
    {lap: 0, mark: 0, index: 2},
    {lap: 0, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 4}
]

## send:

head: {lap: 0, mark: 0, index: 0}
tail: {lap: 0, mark: 0, index: 3}
buffer:
[
    {lap: 0, mark: 0, index: 1},
    {lap: 0, mark: 0, index: 2},
    {lap: 0, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 4}
]

## send:

head: {lap: 0, mark: 0, index: 0}
tail: {lap: 0, mark: 0, index: 4}
buffer:
[
    {lap: 0, mark: 0, index: 1},
    {lap: 0, mark: 0, index: 2},
    {lap: 0, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 4},
    {lap: 0, mark: 0, index: 4}
]

## send:

head: {lap: 0, mark: 0, index: 0}
tail: {lap: 1, mark: 0, index: 0}
buffer:
[
    {lap: 0, mark: 0, index: 1},
    {lap: 0, mark: 0, index: 2},
    {lap: 0, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 4},
    {lap: 0, mark: 0, index: 5}
]

## recv

head: {lap: 0, mark: 0, index: 1}
tail: {lap: 1, mark: 0, index: 0}
buffer:
[
    {lap: 1, mark: 0, index: 0},
    {lap: 0, mark: 0, index: 2},
    {lap: 0, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 4},
    {lap: 0, mark: 0, index: 5}
]

## recv

head: {lap: 0, mark: 0, index: 2}
tail: {lap: 1, mark: 0, index: 0}
buffer:
[
    {lap: 1, mark: 0, index: 0},
    {lap: 1, mark: 0, index: 1},
    {lap: 0, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 4},
    {lap: 0, mark: 0, index: 5}
]

## recv

head: {lap: 0, mark: 0, index: 3}
tail: {lap: 1, mark: 0, index: 0}
buffer:
[
    {lap: 1, mark: 0, index: 0},
    {lap: 1, mark: 0, index: 1},
    {lap: 1, mark: 0, index: 2},
    {lap: 0, mark: 0, index: 4},
    {lap: 0, mark: 0, index: 5}
]

## recv

head: {lap: 0, mark: 0, index: 4}
tail: {lap: 1, mark: 0, index: 0}
buffer:
[
    {lap: 1, mark: 0, index: 0},
    {lap: 1, mark: 0, index: 1},
    {lap: 1, mark: 0, index: 2},
    {lap: 1, mark: 0, index: 3},
    {lap: 0, mark: 0, index: 5}
]

## recv

head: {lap: 1, mark: 0, index: 0}
tail: {lap: 1, mark: 0, index: 0}
buffer:
[
    {lap: 1, mark: 0, index: 0},
    {lap: 1, mark: 0, index: 1},
    {lap: 1, mark: 0, index: 2},
    {lap: 1, mark: 0, index: 3},
    {lap: 1, mark: 0, index: 4}
]

## send

head: {lap: 1, mark: 0, index: 0}
tail: {lap: 1, mark: 0, index: 1}
buffer:
[
    {lap: 1, mark: 0, index: 1},
    {lap: 1, mark: 0, index: 1},
    {lap: 1, mark: 0, index: 2},
    {lap: 1, mark: 0, index: 3},
    {lap: 1, mark: 0, index: 4}
]

## send

head: {lap: 1, mark: 0, index: 0}
tail: {lap: 1, mark: 0, index: 2}
buffer:
[
    {lap: 1, mark: 0, index: 1},
    {lap: 1, mark: 0, index: 2},
    {lap: 1, mark: 0, index: 2},
    {lap: 1, mark: 0, index: 3},
    {lap: 1, mark: 0, index: 4}
]

## recv

head: {lap: 1, mark: 0, index: 1}
tail: {lap: 1, mark: 0, index: 2}
buffer:
[
    {lap: 2, mark: 0, index: 0},
    {lap: 1, mark: 0, index: 2},
    {lap: 1, mark: 0, index: 2},
    {lap: 1, mark: 0, index: 3},
    {lap: 1, mark: 0, index: 4}
]

## recv

head: {lap: 1, mark: 0, index: 2}
tail: {lap: 1, mark: 0, index: 2}
buffer:
[
    {lap: 2, mark: 0, index: 0},
    {lap: 2, mark: 0, index: 1},
    {lap: 1, mark: 0, index: 2},
    {lap: 1, mark: 0, index: 3},
    {lap: 1, mark: 0, index: 4}
]
