/**************************************************************************
 * TURO COMPILER
 * 
 * handle_args.c - Functions to handle command-line arguments.
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/ 

#include "turo.h"

static char printbuf[100];

FILE *infile = NULL;
FILE *outfile = NULL;
FILE *errfile = NULL;

void handle_args(int argc, char *argv[]) {
    int option;
    bool file_provided;
    outfile = stdout;
    errfile = stderr;
    while ((option = getopt(argc, argv, "i:o:")) != -1) {
        switch (option) {
            case 'i':
                if ((infile = fopen(optarg, "r")) == NULL) {
                    sprintf(printbuf, "Input file %s not found.", optarg);
                    logmsg(LOG_FATAL, printbuf);
                    return;
                }
                file_provided = true;
                break;
            case 'o':
                if ((outfile = fopen(optarg, "w")) == NULL) {
                    sprintf(printbuf, "Failed to open output file %s.", optarg);
                    logmsg(LOG_FATAL, printbuf);
                    return;
                }
                break;
            default:
                sprintf(printbuf, "Ignoring unknown option %c.", optopt);
                logmsg(LOG_INFO, printbuf);
        }
    }
    if (!file_provided) {
        sprintf(printbuf, "Usage: ./turo -i infile [-o] [outfile]");
        logmsg(LOG_FATAL, printbuf);
    }
    for(; optind < argc; optind++) {
        sprintf(printbuf, "Ignoring extra argument %s", argv[optind]);
        logmsg(LOG_INFO, printbuf);
    }
    if (infile == NULL) infile = stdin;
    return;
}