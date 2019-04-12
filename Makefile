NAME = computor_v1
CC = cargo

SRC_DIR = ./src/
SRC = app.rs compute.rs equation.rs main.rs parse.rs

SRCS = $(addprefix $(SRC_DIR),$(SRC))

.PHONY: all clean re

all: $(NAME)

$(NAME): $(SRCS)
	$(CC) build
	@mv target/debug/$(NAME) .

clean:
	@if [ -f $(NAME) ]; then rm $(NAME) && echo $(NAME) binary removed; else echo no binary found; fi

re: clean all

