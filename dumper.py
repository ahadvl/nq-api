#!/usr/bin/python3

# (Natiq Api)
# Collects the all migrations and pack to the one file
# This script is works just with diesel.rs migration files
# v0.0.1
# by @AzerothA

import sys
import os

def get_files_path_list(migrations_folder_path):
    return list(map(
        lambda x: migrations_folder_path + x + "/" + os.listdir(migrations_folder_path + x)[1], 
        os.listdir(migrations_folder_path)
    ))

def get_files_content(files_path_list):
    # now read all Up.sql files and put content into result string
    return "\n".join(list(map(lambda x: open(x, "r").read(), files_path_list)))

def main(args):
    # Get sql files list
    files_path = get_files_path_list(args[1])

    # Get the all sql
    sql = get_files_content(files_path)

    # now create the file
    with open(args[2], "w") as result:
       # Writing data to a file
       result.write(sql)
       result.close()

if __name__ == "__main__":
    main(sys.argv)