PR  = def
CC  = gcc
CXX = g++
CFLAGS =-Wall -Os
AFLAGS =
MFLAGS =
CFLAGS+=-ffunction-sections -fdata-sections
MFLAGS+=-gc-sections
#MFLAGS+=-nostartfiles
MFLAGS+=-o $(PR)
#LFLAGS =-T script.ld

all: $(PR)

OBJECTS = main.o

$(PR): $(OBJECTS)
	$(CXX) $(AFLAGS) $(MFLAGS) $(OBJECTS) $(LFLAGS)
clean:
	rm -f *.o *~ $(PR)
%.o: %.c
	$(CC) -c $(AFLAGS) $(CFLAGS) $(GFLAGS) $<
%.o: %.cpp
	$(CXX) -c $(AFLAGS) $(CFLAGS) $(GFLAGS) $<

