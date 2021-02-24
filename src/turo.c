/**************************************************************************
 * TURO COMPILER
 * 
 * turo.c - The top-level loop of the compiler.
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/ 

#include "turo.h"

int main(int argc, char *argv[]) {
    handle_args(argc, argv);
    init_interface();
    //init_lexer();
    finalize();
    return EXIT_SUCCESS;
}
