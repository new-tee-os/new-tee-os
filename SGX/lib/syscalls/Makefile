libtest.a: ocall.o
	$(AR) rcsD $@ $^
	mv libtest.a ../libtest.a

ocall.o: ocall.c
	gcc -c $^ -o $@

all: libtest.a
clean:
	rm -rf a.out ocall.o ../libtest.a
