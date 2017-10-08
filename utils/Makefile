all: stm32.rs

stm32.rs: def.c
	cd ./ast && $(MAKE) all
	./ast/ast def.c stm.rs

def.c:
	patch -p2 <stm.patch
	cd ./def && $(MAKE) all
	./def/def
