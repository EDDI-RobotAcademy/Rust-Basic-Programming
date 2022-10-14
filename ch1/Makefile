PWD = $(shell pwd)

TARGET = hello_rust variable number_literal number_base_literal type_cast

all:	_all $(TARGET)

_all:
	@echo "========================================="
	@echo "========= Compiling Rust Basics ========="
	@echo "========================================="

hello_rust:					hello_rust.rs
	@echo "$<"
	rustc $<

variable:					variable.rs
	@echo "$<"
	rustc $<

number_literal:				number_literal.rs
	@echo "$<"
	rustc $<

number_base_literal:		number_base_literal.rs
	@echo "$<"
	rustc $<

type_cast:					type_cast.rs
	@echo "$<"
	rustc $<

clean:
	rm -rf $(TARGET)
