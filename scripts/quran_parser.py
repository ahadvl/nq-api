# This script will create natiq essential quran tables
# Tables this script will create ->
# quran_ayahs | quran_words | quran_surahs
# More clearly the result of this script is the sql code
# that will create tables and insert the data (quran)
#
# (nq-team)


import hashlib
import sys
import os
import xml.etree.ElementTree as ET

TANZIL_QURAN_SOURCE_HASH = "e7ab47ae9267ce6a3979bf60031b7c40c9701cb2c1d899bbc6e56c67058b17e2"


# TODO This table also has period column that we need
#      to insert
INSERTABLE_QURAN_SURAH_TABLE = "quran_surahs(name)"

INSERTABLE_QURAN_WORDS_TABLE = "quran_words(ayah_id, word)"

# TODO This table also has a sajdeh cloumn that we need
#      to insert in next update
INSERTABLE_QURAN_AYAHS_TABLE = "quran_ayahs(surah_id, ayah_number)"

def exit_err(msg):
    exit("Error: " + msg)

def validate_tanzil_quran(source):
    m = hashlib.sha256()
    m.update(source)

    return m.hexdigest() == TANZIL_QURAN_SOURCE_HASH

def insert_to_table(i_table, values):
    return f'INSERT INTO {i_table} VALUES {values};'

def parse_quran_suarhs_table(root):
    result = []

    for child in root:
        surah_name = child.attrib['name']
        result.append(f'("{surah_name}")')

    return insert_to_table(INSERTABLE_QURAN_SURAH_TABLE, ",\n".join(result))

def parse_quran_words_table(root):
    result = []
    ayah_number = 1

    for aya in root.iter('aya'):
        # Get the array of aya words
        words = aya.attrib['text'].split(" ")

        # Map and change every word to a specific format
        values = list(map(lambda word: f'({ayah_number}, "{word}")', words))

        # Join the values with ,\n
        result.append(",\n".join(values))

        # Next
        ayah_number += 1

    return insert_to_table(INSERTABLE_QURAN_WORDS_TABLE , ",\n".join(result))

def parse_quran_ayahs_table(root):
    result = []
    sura_number = 0

    # We just need surah_id and ayah number and sajdeh enum
    for aya in root.iter('aya'):
        aya_index = aya.attrib['index']

        if aya_index == "1":
            sura_number += 1

        result.append(f'({sura_number}, {aya_index})')

    return insert_to_table(INSERTABLE_QURAN_AYAHS_TABLE, ",\n".join(result))

def main(args):
    # Get the quran path
    quran_xml_path = args[1]

    # Split into the name and extention
    splited_path = os.path.splitext(quran_xml_path)

    # Check if file format is correct
    if splited_path[1] != ".xml":
        exit_err("Quran Source must be an xml file")

    # Open file
    quran_source = open(quran_xml_path, "r")
    
    # Read to string
    quran_source_as_string = quran_source.read().encode('utf-8')

    # We dont need file anymore
    quran_source.close()

    # Validate the source
    if validate_tanzil_quran(quran_source_as_string) == False:
        exit_err("Please use the orginal Tanzil Quran Source")

    # Parse the quran xml file string
    # To a XML object so we can use it in generating sql
    root = ET.fromstring(quran_source_as_string)

    # parse the first table  : quran_ayahs
    quran_surahs_table = parse_quran_suarhs_table(root)

    # parse the second table : quran_words
    quran_words_table  = parse_quran_words_table(root)

    # parse the third table  : quran_surahs
    quran_ayahs_table  = parse_quran_ayahs_table(root)

    # Collect all the data to one string
    final_sql_code = f'{quran_surahs_table}\n{quran_words_table}\n{quran_ayahs_table}'

    # Not create the final sql file
    sql_file = open("result.sql", "w")

    sql_file.write(final_sql_code)

    sql_file.close()

if __name__ == "__main__":
    main(sys.argv)
