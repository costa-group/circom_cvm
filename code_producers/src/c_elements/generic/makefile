CC=g++
CFLAGS=-std=c++11 -O3 -I.
DEPS_HPP = circom.hpp calcwit.hpp fr.hpp
DEPS_O = main.o calcwit.o fr.o 

all: {{run_name}}

%.o: %.cpp $(DEPS_HPP)
<<<<<<< HEAD
	$(CC) -c $< $(CFLAGS)
=======
	$(CC) -Wno-address-of-packed-member -c $< $(CFLAGS)
>>>>>>> e410b0d5cd2948a15931df0bc50d79ce56fa8c32

{{run_name}}: $(DEPS_O) {{run_name}}.o
	$(CC) -o {{run_name}} *.o -lgmp {{#if has_parallelism}}-pthread{{/if}}
