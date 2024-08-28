const fs = require('fs');

// o1js용 입력 데이터 생성 함수
function generateO1jsInput(batchSize, inputSize, chunkSize) {
  const inputData = [];

  // 입력 데이터 생성
  for (let i = 0; i < batchSize; i++) {
    const singleInput = [];
    for (let j = 0; j < inputSize; j++) {
      // 0에서 10 사이의 임의의 정수를 생성하여 소수점 문제를 방지
      const randomValue = Math.floor(Math.random() * 10); 
      if (isNaN(randomValue)) {
        throw new Error(`Invalid number generated: ${randomValue}`);
      }
      singleInput.push(randomValue);
    }
    inputData.push(singleInput);
  }

  // 입력 데이터를 작은 청크로 나눔
  const chunks = [];
  for (let i = 0; i < inputData.length; i += chunkSize) {
    const chunk = inputData.slice(i, i + chunkSize);
    chunks.push(chunk);
  }

  return chunks;
}

// 설정
const batchSize = 1024; // 전체 입력 개수 (그림에 맞춰 설정)
const inputSize = 5;   // 각 입력의 크기 (예: 5차원 벡터)
const chunkSize = 10;  // 각 청크의 크기 (병렬 처리할 데이터 수)

// o1js용 입력 데이터 생성
const o1jsInput = generateO1jsInput(batchSize, inputSize, chunkSize);

// JSON 파일로 저장
const o1jsInputJson = { input_data: o1jsInput };
fs.writeFileSync('input_o1js.json', JSON.stringify(o1jsInputJson, null, 2));

console.log('input_o1js.json 파일이 생성되었습니다.');

