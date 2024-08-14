

#!/bin/bash

# Create directories for logs if they don't exist
mkdir -p logs

# Run o1js script with gtime and capture output
echo "Running o1js script..."
gtime -v node dist/mlp.js &> logs/o1js_log.txt
echo "o1js script completed. Log saved to logs/o1js_log.txt."

# Run ezkl script with gtime and capture output
echo "Running ezkl script..."
gtime -v ezkl prove --witness models/mlp/witness.json --pk-path models/mlp/pk.key --compiled-circuit models/mlp/model.compiled --proof-path models/mlp/proof.json &> logs/ezkl_log.txt
echo "ezkl script completed. Log saved to logs/ezkl_log.txt."

# Extract and display relevant information from the logs
echo -e "\nPerformance Summary:"
echo "===================="
echo "o1js:"
grep "User time (seconds)" logs/o1js_log.txt
grep "Maximum resident set size" logs/o1js_log.txt
grep "Percent of CPU this job got" logs/o1js_log.txt

echo "===================="
echo "ezkl:"
grep "User time (seconds)" logs/ezkl_log.txt
grep "Maximum resident set size" logs/ezkl_log.txt
grep "Percent of CPU this job got" logs/ezkl_log.txt