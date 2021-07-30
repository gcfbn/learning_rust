# Kruskal's algorithm
| Implementation           | nodes_count | Result                              |
| ---------------------    |:-----------:|:----------------------------------- |
| find_parent_recursion    | 1000        | 904,855 ns/iter (+/- 196,901)       |
| find_parent_no_recursion | 1000        | 881,320 ns/iter (+/- 53,714)        |
| union_find               | 1000        | 194,235 ns/iter (+/- 91,119)        |
| find_parent_recursion    | 10000       | 87,171,590 ns/iter (+/- 56,673,159) |
| find_parent_no_recursion | 10000       | 83,473,270 ns/iter (+/- 28,189,707) |
| union_find               | 10000       | 2,259,227 ns/iter (+/- 322,197)     |
| find_parent_recursion    | 100000      | Stack Overflow                      |
| find_parent_no_recursion | 100000      | Stack Overflow                      |
| union_find               | 100000      | Stack Overflow                      |
