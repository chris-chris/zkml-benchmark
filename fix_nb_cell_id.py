import json
import uuid

# Jupyter Notebook 파일 경로
notebook_path = './models/mlp/orion/orion.ipynb'
# notebook_path = './models/mlp/risczero/risczero.ipynb'

# 노트북 파일 열기
with open(notebook_path, 'r', encoding='utf-8') as f:
    notebook_data = json.load(f)

# 셀 ID를 고유하게 수정
cell_ids = set()
for cell in notebook_data['cells']:
    if 'id' in cell:
        # 중복된 ID 확인
        while cell['id'] in cell_ids:
            cell['id'] = str(uuid.uuid4())[:8]  # 새로운 고유 ID 생성
        cell_ids.add(cell['id'])

# 수정된 노트북 파일 저장
with open(notebook_path, 'w', encoding='utf-8') as f:
    json.dump(notebook_data, f, indent=4)

print(f"Duplicate cell IDs have been fixed in {notebook_path}")

