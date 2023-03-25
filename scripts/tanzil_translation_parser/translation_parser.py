# This script will create natiq essential translations tables
# Tables this script will create ->
# translations_text | translations
# More clearly the result of this script is the sql code
# that will create tables and insert the data (translation)
#
# (nq-team)

import sys
import os
import xml.etree.ElementTree as ET
import psycopg2
import re

INSERTABLE_TRANSLATIONS_TEXT = "translations_text(text, translation_id, ayah_id)"


def exit_err(msg):
    exit("Error: " + msg)


def insert_to_table(i_table, values):
    return f'INSERT INTO {i_table} VALUES {values};'


def translations(translations_folder_path):
    return list(os.scandir(translations_folder_path))


def remove_comments_from_xml(source):
    return re.sub("(<!--.*?-->)", "", source.decode('utf-8'), flags=re.DOTALL)


def create_translation_table(root, translation_id):
    result = []
    ayah_num = 1

    for child in root.iter('aya'):
        surah_text = child.attrib["text"].replace("'", "&quot;")
        result.append(
            f"('{surah_text}', {translation_id}, {ayah_num})")
        ayah_num += 1

    return insert_to_table(INSERTABLE_TRANSLATIONS_TEXT, ",".join(result))


def check_the_translation_file(translation):
    # Split into the name and extention
    splited_path = os.path.splitext(translation)

    # Check if file format is correct
    if splited_path[1] != ".xml":
        exit_err("Quran Source must be an xml file")


def translation_metadata(file_path):
    splited = os.path.split(file_path)
    splited_file_name = splited[1].split('.')

    return {"language": splited_file_name[0], "author": splited_file_name[1], "type": splited_file_name[2]}

# We need to get the translator id first
# We can use the name of file as a translator


def main(args):

    # Get the database information
    database = args[2]
    host = args[3]
    user = args[4]
    password = args[5]
    port = args[6]

    # Connect to the database
    conn = psycopg2.connect(database=database, host=host,
                            user=user, password=password, port=port)

    # Get the quran path
    translations_folder_path = args[1]

    translations_list = translations(translations_folder_path)

    for translation in translations_list:
        cur = conn.cursor()
        path = translation.path

        metadata = translation_metadata(path)

        print(f'Parsing {path}')

        if metadata["type"] != "xml":
            exit_err("This program can just parse the xml type of translations")

        translation_source = open(path, "r")
        tranlation_text = translation_source.read().encode('utf-8')
        translation_source.close()

        # We will create a account for every translator
        cur.execute("INSERT INTO app_accounts(username, account_type) VALUES (%s, %s) ON CONFLICT (username) DO NOTHING RETURNING id",
                    (metadata['author'], "user"))

        account_id = cur.fetchone()

        if account_id != None:
            # Also we must create a User for this account
            cur.execute(
                "INSERT INTO app_users(account_id, last_name) VALUES (%s, %s) ON CONFLICT (account_id) DO NOTHING", (account_id, metadata['author']))
        else:
            print("The translator account exists, skiping user creation")
            cur.execute(
                "SELECT id FROM app_accounts WHERE username=%s", (metadata['author'], ))
            account_id = cur.fetchone()

        conn.commit()

        # Insert a translation in translations table
        cur.execute("INSERT INTO translations(translator_id, language) VALUES (%s, %s) RETURNING id",
                    (account_id[0], metadata["language"]))

        translation_text_clean = remove_comments_from_xml(tranlation_text)

        print("parsing xml")

        root = ET.fromstring(translation_text_clean)
        translations_text_data = create_translation_table(
            root, cur.fetchone()[0])

        print("executing")
        cur.execute(translations_text_data)

        conn.commit()

        cur.close()

    conn.close()


if __name__ == "__main__":
    main(sys.argv)
