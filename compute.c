#include <stdint.h>
#include "compute.h"

extern int computations_count;
extern struct Instruction computations[];

int32_t run(struct Instruction *computation, uint16_t input[6])
{
    int32_t tape[12];
    tape[0] = 0;
    tape[1] = input[0];
    tape[2] = input[1];
    tape[3] = input[2];
    tape[4] = input[3];
    tape[5] = input[4];
    tape[6] = input[5];

    for (int i = 0; i < 5; i++) {
        int32_t lhs = tape[computation[i].lhs];
        int32_t rhs = tape[computation[i].rhs];
        int32_t output;

        switch (computation[i].op) {
          case Add: output = lhs + rhs; break;
          case Sub: output = lhs - rhs; break;
          case Mul: output = lhs * rhs; break;
          case Div: output = lhs / rhs;
            if (output * rhs != lhs) return 0;
            break;
        }
        if (output <= 0) return 0; // catch overflow and negative numbers

        tape[7 + i] = output;
    }

    return tape[11];
}

int main()
{
    uint16_t nums[6] = {4, 5, 25, 50, 75, 100};

    int sum = 0; // so the compiler doesn't throw the results away
    for (int j = 0; j < 100; j++) { // loop for benchmarking
        for (int i = 0; i < computations_count; i++) {
            int32_t n = run(&computations[i*5], nums);
            sum += n;
        }
    }

    return sum;
}
