#include <stdint.h>

enum Op { Add, Sub, Mul, Div };

struct Instruction {
  uint8_t lhs : 4;
  uint8_t rhs : 4;
  uint8_t op;
};
