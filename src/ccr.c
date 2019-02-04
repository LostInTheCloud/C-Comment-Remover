#include "ccr.h"

#include <dirent.h> 
#include <stdio.h> 
#include <string.h>

int main(void) 
{
    get_files();
    return(0);
}

void get_files()
{
    DIR *d;
    struct dirent *dir;
    d = opendir(".");
    if (d) 
    {
        while ((dir = readdir(d)) != NULL) 
        {
            if (!strcmp(dir->d_name, ".") || !strcmp(dir->d_name, "..")) 
            {
                // filtering out these strange fellas
                continue;
            }
            if (dir->d_type == DT_REG) // regular file
            {
                printf("%s\n", dir->d_name);
            }            
            if (dir->d_type == DT_DIR) // directory
            {
                printf("%s\n", dir->d_name);
            }
        }
        closedir(d);
    }
}