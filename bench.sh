#!/bin/bash

# Create directories for logs if they don't exist
mkdir -p logs

# CSV 파일 초기화
output_csv="experiment_results.csv"
echo "model,exp_num,proving_time,memory_usage,cpu_usage" > $output_csv

# MLP 모델에 대한 실험 루프
for i in {1..5}; do
    echo "Running ezkl script for MLP experiment $i..."

    # gtime을 사용하여 ezkl 명령 실행 및 로그 저장
    gtime -v ezkl prove --witness models/mlp/mlp$i/witness.json --pk-path models/mlp/mlp$i/pk.key --compiled-circuit models/mlp/mlp$i/model.compiled --proof-path models/mlp/mlp$i/proof.json &> logs/ezkl_mlp${i}_log.txt
    
    # 성능 데이터 추출
    proving_time=$(grep "User time (seconds)" logs/ezkl_mlp${i}_log.txt | awk '{print $4}')
    memory_usage=$(grep "Maximum resident set size" logs/ezkl_mlp${i}_log.txt | awk '{print $6}')
    cpu_usage=$(grep "Percent of CPU this job got" logs/ezkl_mlp${i}_log.txt | awk '{print $7}')
    
    # CSV 파일에 결과 저장
    echo "mlp,$i,$proving_time,$memory_usage,$cpu_usage" >> $output_csv

    echo "Running o1js script for MLP experiment $i..."
    
    gtime -v node dist/mlp.js $i &> logs/o1js_mlp${i}_log.txt

    # 성능 데이터 추출
    proving_time=$(grep "User time (seconds)" logs/o1js_mlp${i}_log.txt | awk '{print $4}')
    memory_usage=$(grep "Maximum resident set size" logs/o1js_mlp${i}_log.txt | awk '{print $6}')
    cpu_usage=$(grep "Percent of CPU this job got" logs/o1js_mlp${i}_log.txt | awk '{print $7}')
    
    # CSV 파일에 결과 저장
    echo "o1js,$i,$proving_time,$memory_usage,$cpu_usage" >> $output_csv
done

echo "Experiment completed. Results saved to $output_csv."