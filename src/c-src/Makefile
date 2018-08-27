CC=gcc
DEPS = $(wildcard *.h)
FILES = $(basename $(DEPS))
OBJ = $(addsuffix .o,$(FILES))
%.o : %.c $(DEPS)
	$(CC) -c -o $@ $<

all : $(OBJ)
	 ar rc 'lib$(basename $^).a' $^

clear :
	rm *.o *.a
