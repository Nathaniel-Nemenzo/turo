/**************************************************************************
 * TURO COMPILER
 * 
 * err_handler.h - Contains error enums and functions for error handling.
 * ERROR HANDLING CODE IS COPIED FROM A 2021 UTCS ASSIGNMENT BY S. CHATTERJEE, X. SHEN, AND T. BYRD.
 * 
 * Copyright (c) 2021. N. Nemenzo. All rights reserved.
 * May not be used, modified, or copied without permission.
 **************************************************************************/ 

typedef enum {
    LOG_INFO,       // print message to console
    LOG_WARNING,    // print warning message and ignore input
    LOG_ERROR,      // print error message and ignore input
    LOG_FATAL,      // print error message and terminate program
} log_lev_t;

typedef enum {
    ERR_LEX,        // lexical error
    ERR_SYNTAX,     // syntactic error
    ERR_TYPE,       // type reference error
    ERR_EVAL,       // evaluation error
    ERR_UNDEFINED,  // undefined variable error
} err_type_t;

/* logging() - log information to the console and given a certain log level and a log string.
 * Parameters: log_level_t - signify the log level
 *             char * - give the string to log
 * Side effects: none
 * Return: return based on printing status.
 */
extern int logmsg(log_lev_t, char *);

/* handle_error() - display an error based on an error type.
 * Parameters: err_type_t - signify the error level
 * Side effects: stop reading input
 * Return: return based on printing status
 */
extern int handle_error(err_type_t);

