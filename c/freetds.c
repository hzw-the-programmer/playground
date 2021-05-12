#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <errno.h>
#include <unistd.h>
#include <libgen.h>

#include <sybfront.h>
#include <sybdb.h>

int err_handler(DBPROCESS*, int, int, int, char*, char*);
int msg_handler(DBPROCESS*, DBINT, int, int, char*, char*, char*, int);

extern char *optarg;
extern int optind;

const static char syntax[] = "syntax: example -S server -D db -U user -P passwd -C charset\n";

struct {
    char *appname, *servername, *dbname, *username, *password, *charset;
} options = {0, 0, 0, 0, 0};

int main(int argc, char *argv[]) {
    int i, ch;
    LOGINREC *login;
    DBPROCESS *dbproc;
    RETCODE erc;

    options.appname = basename(argv[0]);

    while ((ch = getopt(argc, argv, "U:P:S:D:C:")) != -1) {
        switch (ch) {
            case 'S':
                options.servername = strdup(optarg);
                break;
            case 'D':
                options.dbname = strdup(optarg);
                break;
            case 'U':
                options.username = strdup(optarg);
                break;
            case 'P':
                options.password = strdup(optarg);
                break;
            case 'C':
                options.charset = strdup(optarg);
                break;
            case '?':
            default:
                fprintf(stderr, syntax);
                exit(1);
        }
    }

    argc -= optind;
    argv += optind;

    if (!(options.servername && options.username && options.password)) {
        fprintf(stderr, syntax);
        exit(1);
    }

    if (dbinit() == FAIL) {
        fprintf(stderr, "%s:%d: dbinit() failed\n",
            options.appname, __LINE__);
        exit(1);
    }

    dberrhandle(err_handler);
    dbmsghandle(msg_handler);

    if ((login = dblogin()) == NULL) {
        fprintf(stderr, "%s:%d: unable to allocate login structure\n",
            options.appname, __LINE__);
        exit(1);
    }

    DBSETLUSER(login, options.username);
    DBSETLPWD(login, options.password);
    if (options.charset) {
        DBSETLCHARSET(login, options.charset);
    }

    if ((dbproc = dbopen(login, options.servername)) == NULL) {
        fprintf(stderr, "%s:%d: unable to connect to %s as %s\n",
            options.appname, __LINE__,
            options.servername, options.username);
        exit(1);
    }

    if (options.dbname && (erc = dbuse(dbproc, options.dbname)) == FAIL) {
        fprintf(stderr, "%s:%d: unable to use to database %s\n",
            options.appname, __LINE__, options.dbname);
        exit(1);
    }

    for (i = 0; i < argc; i++) {
        assert(argv[i]);
        printf("%s ", argv[i]);
        if ((erc = dbfcmd(dbproc, "%s ", argv[i])) == FAIL) {
            fprintf(stderr, "%s:%d: dbfcmd() failed\n", options.appname, __LINE__);
            exit(1);
        }
    }
    printf("\n");

    if ((erc = dbsqlexec(dbproc)) == FAIL) {
        fprintf(stderr, "%s:%d: dbsqlexec() failed\n", options.appname, __LINE__);
        exit(1);
    }

    while ((erc = dbresults(dbproc)) != NO_MORE_RESULTS) {
        struct COL {
            char *name;
            char *buffer;
            int type, size, status;
        } *columns, *pcol;
        int ncols;
        int row_code;

        if (erc == FAIL) {
            fprintf(stderr, "%s:%d: dbresults failed\n",
                options.appname, __LINE__);
            exit(1);
        }

        ncols = dbnumcols(dbproc);

        if ((columns = calloc(ncols, sizeof(struct COL))) == NULL) {
            perror(NULL);
            exit(1);
        }

        for (pcol = columns; pcol - columns < ncols; pcol++) {
            int c = pcol - columns + 1;

            pcol->name = dbcolname(dbproc, c);
            pcol->type = dbcoltype(dbproc, c);
            pcol->size = dbcollen(dbproc, c);

            if (SYBCHAR != pcol->type) {
                pcol->size = dbprcollen(dbproc, c);
                if (pcol->size > 255) {
                    pcol->size = 255;
                }
            }

            printf("%*s", pcol->size, pcol->name);

            if ((pcol->buffer = calloc(1, pcol->size + 1)) == NULL) {
                perror(NULL);
                exit(1);
            }

            erc = dbbind(dbproc, c, NTBSTRINGBIND,
                pcol->size + 1, (BYTE*)pcol->buffer);
            if (erc == FAIL) {
                fprintf(stderr, "%s:%d: dbbind(%d) failed\n",
                    options.appname, __LINE__, c);
                exit(1);
            }

            erc = dbnullbind(dbproc, c, &pcol->status);
            if (erc == FAIL) {
                fprintf(stderr, "%s:%d: dbnullbind(%d) failed\n",
                    options.appname, __LINE__, c);
                exit(1);
            }
        }

        printf("\n");
        
        while ((row_code = dbnextrow(dbproc)) != NO_MORE_ROWS) {
            switch (row_code) {
                case REG_ROW:
                    for (pcol = columns; pcol - columns < ncols; pcol++) {
                        char *buffer = pcol->status == -1 ?
                            "NULL" : pcol->buffer;
                        printf("%*s", pcol->size, buffer);
                    }
                    printf("\n");
                    break;
                case BUF_FULL:
                    assert(row_code != BUF_FULL);
                    break;
                case FAIL:
                    fprintf(stderr, "%s:%d: dbresults failed\n",
                        options.appname, __LINE__);
                    exit(1);
                    break;
                default:
                    printf("Data for computeid %d ignored\n", row_code);
            }
        }

        for (pcol = columns; pcol - columns < ncols; pcol++) {
            free(pcol->buffer);
        }
        free(columns);

        if (DBCOUNT(dbproc) > -1) {
            fprintf(stderr, "%d rows affected\n", DBCOUNT(dbproc));
        }

        if (dbhasretstat(dbproc) == TRUE) {
            printf("Procedure returned %d\n", dbretstatus(dbproc));
        }
    }

    dbclose(dbproc);
    dbexit();
    exit(0);
}

int msg_handler(DBPROCESS *dbproc, DBINT msgno, int msgstate, int severity,
    char *msgtext, char *srvname, char *procname, int line) {

    enum {changed_database = 5701, changed_language = 5703};

    if (msgno == changed_database || msgno == changed_language) {
        return 0;
    }

    if (msgno > 0) {
        fprintf(stderr, "Msg %ld, Level %d, State %d\n",
            (long)msgno, severity, msgstate);

        if (strlen(srvname) > 0) {
            fprintf(stderr, "Server '%s', ", srvname);
        }
        if (strlen(procname) > 0) {
            fprintf(stderr, "Procedure '%s', ", procname);
        }
        if (line > 0) {
            fprintf(stderr, "Line %d", line);
        }

        fprintf(stderr, "\n\t");
    }
    fprintf(stderr, "%s\n", msgtext);

    if (severity > 10) {
        fprintf(stderr, "%s:%d: error: severity %d > 10, exiting\n",
            options.appname, __LINE__, severity);
        exit(severity);
    }
}

int err_handler(DBPROCESS *dbproc, int severity, int dberr, int oserr,
    char *dberrstr, char *oserrstr) {

    if (dberr) {
        fprintf(stderr, "%s:%d: Msg %d, Level %d\n",
            options.appname, __LINE__, dberr, severity);
        fprintf(stderr, "%s\n\n", dberrstr);
    } else {
        fprintf(stderr, "%s: DB-LIBRARY error:\n\t",
            options.appname);
        fprintf(stderr, "%s\n", dberrstr);
    }

    return INT_CANCEL;
}

/*
gcc freetds.c -I/home/zhiwenhe/work/build/freetds/include -L/home/zhiwenhe/work/build/freetds/lib -lsybdb
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/zhiwenhe/work/build/freetds/lib ./a.out -S server -D db -U user -P passwd -C UTF-8
*/
