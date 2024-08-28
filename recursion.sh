#!/bin/bash

# Create directories for logs if they don't exist
mkdir -p logs

# CSV 파일 초기화
output_csv="experiment_results.csv"
echo "framework,model,exp_num,proving_time,memory_usage,cpu_usage" > $output_csv

# 함수: 성능 데이터를 추출하는 기능
extract_performance_data() {
    local log_file=$1
    proving_time=$(grep "User time (seconds)" "$log_file" | awk '{print $4}')
    memory_usage=$(grep "Maximum resident set size" "$log_file" | awk '{print $6}')
    cpu_usage=$(grep "Percent of CPU this job got" "$log_file" | awk '{print $7}')
}

# 함수: 벤치마크 실행 및 결과 저장
run_benchmark() {
    local framework=$1
    local exp_num=$2
    local command=$3
    local log_file="logs/${framework}_mlp${exp_num}_log.txt"

    # if [ "$framework" == "ezkl" ]; then
    #     EXP_NUM=$exp_num jupyter nbconvert --to notebook --execute models/mlp/mlp.ipynb
    # fi

    echo "Running ${framework} script for MLP experiment $exp_num..."

    if [ "$framework" == "risczero" ]; then
        # risczero의 경우 해당 디렉토리로 이동해서 실행
        (cd mlp_risczero && gtime -v $command &> "../$log_file")
    else
        # 명령어 실행 및 로그 저장
        gtime -v $command &> "$log_file"
    fi   
    
    # 성능 데이터 추출
    extract_performance_data "$log_file"
    
    # CSV 파일에 결과 저장
    echo "${framework},mlp,$exp_num,$proving_time,$memory_usage,$cpu_usage" >> $output_csv
}

# MLP 모델에 대한 실험 루프
# for i in {1..5}; do

for i in {1..10}; do
    run_benchmark "ezkl" $i "ezkl prove --witness models/recursion/mlp$i/witness.json --pk-path models/recursion/mlp$i/pk.key --compiled-circuit models/recursion/mlp$i/model.compiled --proof-path models/recursion/mlp$i/proof.json"
    # run_benchmark "o1js" $i "node dist/mlp.js $i"
    # run_benchmark "orion" $i "scarb run --path models/linear_regression/orion"
    # run_benchmark "orion" $i "jupyter nbconvert --to notebook --execute ./models/mlp/orion/orion.ipynb --output orion_output"
    # run_benchmark "risczero" $i "cd mlp_risczero && ./zkvm"
done

echo "Experiment completed. Results saved to $output_csv."

