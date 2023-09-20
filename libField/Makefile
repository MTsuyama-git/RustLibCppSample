all: main

main: main.o libfield.a
	clang++ -o $@ main.o -L. -lfield

test: test.o libfield.a
	clang++ -o $@ test.o -L. -lfield

libfield.a: loadField.o
	ar rcus $@ $^

%.o: %.cpp
	clang++ -c -o $@ -std=c++17 -I. $<

%.o: %.c
	clang -c -o $@ -std=c17 -I. $<

.PHONY: clean
clean:
	rm -rfv main.o main loadField.o libfield libfield.a
