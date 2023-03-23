# (Natiq Api)
# Collects the all migrations and pack to the one file
# This script is works just with diesel.rs migration files
# v0.0.1
# by @AzerothA

import sys
import os

def get_files_content(migrations_folder_path):
    files = list(map(
        lambda x: migrations_folder_path + x + "/" + os.listdir(migrations_folder_path + x)[0], 
        os.listdir(migrations_folder_path)
    ))

    # now read all Up.sql files and put content into result string
    content = "\n".join(list(map(lambda x: open(x, "r").read(), files)))

    return content

def main(args):
    # Get the all sql
    sql = get_files_content(args[1])

    # now create the file
    with open(args[2], "w") as result:
       # Writing data to a file
       result.write(sql)
       result.close()

if __name__ == "__main__":
    main(sys.argv)
