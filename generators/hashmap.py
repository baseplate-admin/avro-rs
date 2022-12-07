import json
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent

data = json.load(open(f'{BASE_DIR}/data.json','r',encoding='utf-8'))['data']['patterns']

data_structure = []

# ("bdh", "ব্ধ"),
for i in data:
    data_structure.append(f'''
    ("{i["find"]}" , "{i['replace']}")
    ''')


stripped_list = ['\n'+data.strip() for data in data_structure]

print(f"""
HashMap::from([
    {','.join(stripped_list)}
]);
""")