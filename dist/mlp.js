"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const o1js_1 = require("o1js");
const relu_1 = require("./relu");
// 선형 변환을 수행하는 모듈화된 레이어 함수
function linearLayer(input, weights, bias) {
  let z = o1js_1.UInt32.from(0);
  for (let i = 0; i < weights.length; i++) {
    z = z.add(weights[i].mul(input[i]));
  }
  z = z.add(bias);
  return z;
}
// 퍼셉트론 레이어를 처리하는 함수
function perceptron(input, weights, bias) {
  const z = linearLayer(input, weights, bias);
  return (0, relu_1.relu)(z); // ReLU 활성화 함수 적용
}
// MLP 모델 정의
function createMLPProgram(depth) {
  return (0, o1js_1.ZkProgram)({
    name: `MLP_Depth_${depth}`,
    publicOutput: o1js_1.UInt32,
    methods: {
      predict: {
        privateInputs: [o1js_1.Provable.Array(o1js_1.UInt32, 4)], // 4개의 입력값
        async method(input) {
          let a = input;
          for (let i = 0; i < depth; i++) {
            const weights = [
              o1js_1.UInt32.from(0),
              o1js_1.UInt32.from(0),
              o1js_1.UInt32.from(0),
              o1js_1.UInt32.from(0),
            ];
            const bias = o1js_1.UInt32.from(0);
            a = [
              perceptron(a, weights, bias),
              perceptron(a, weights, bias),
              perceptron(a, weights, bias),
              perceptron(a, weights, bias),
            ];
          }
          const weightsOut = [o1js_1.UInt32.from(0)];
          const biasOut = o1js_1.UInt32.from(0);
          const zOut = linearLayer(a, weightsOut, biasOut);
          return zOut;
        },
      },
    },
  });
}
// 모델 사용 예제
(async () => {
  console.profile();
  const args = process.argv.slice(2); // 명령줄 인수 받기
  const expNum = parseInt(args[0], 10); // 첫 번째 인수를 depth로 사용
  console.log(`expNum: ${expNum}`);
  // if (isNaN(depth) || depth < 1 || depth > 5) {
  //   console.error("Please provide a valid depth (1-5).");
  //   process.exit(1);
  // }
  const depth = 2 ** expNum;
  console.log(`Creating MLP model with depth ${depth}...`);
  // MLP 모델 생성
  const MLP = createMLPProgram(depth);
  // 입력 데이터 (4개의 입력값)
  let input = [
    o1js_1.UInt32.from(0),
    o1js_1.UInt32.from(0),
    o1js_1.UInt32.from(0),
    o1js_1.UInt32.from(0),
  ];
  // MLP 실행
  const { verificationKey } = await MLP.compile({
    cache: o1js_1.Cache.FileSystemDefault,
    forceRecompile: false,
  });
  console.log(`Making proof for MLP with depth ${depth}...`);
  const proof = await MLP.predict(input);
  // console.log(`Proof created for MLP with depth ${depth}: `, proof.proof);
  console.log("Value: ", proof.publicOutput.toString());
  console.profileEnd();
  // const isValid = await verify(proof, verificationKey);
  // console.log(`Proof is valid for depth ${depth}:`, isValid);
})();
