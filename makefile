BQN := BQN

.PHONY: clean

compute: compute.c compute.h computations.o
	cc -Os compute.c computations.o -o compute

computations.o: computations.c
	cc -Os -c computations.c
computations.c: computations.bqn
	$(BQN) computations.bqn > computations.c

clean:
	rm -r compute computations.o computations.c
