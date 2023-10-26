LIBS := -l RDKitGraphMol -l RDKitSmilesParse -l RDKitMolStandardize

.PHONY: run-main
run-main: main
	./main

main: main.cpp
	clang++ main.cpp -o main -std=c++14 -I /opt/homebrew/include -I /opt/homebrew/include/rdkit -L /opt/homebrew/lib $(LIBS)

main-leaks: main
	sudo leaks --atExit -- ./main
