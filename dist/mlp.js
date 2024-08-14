"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const o1js_1 = require("o1js");
const relu_1 = require("./relu");
// 선형 변환을 수행하는 모듈화된 레이어 함수
function linearLayer(input, weights, bias) {
  let z = o1js_1.Int64.from(0);
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
const MLP = (0, o1js_1.ZkProgram)({
  name: "MLP",
  publicOutput: o1js_1.Int64,
  methods: {
    predict: {
      privateInputs: [o1js_1.Provable.Array(o1js_1.Int64, 5)], // 5개의 입력값
      async method(input) {
        // 첫 번째 히든 레이어
        const weights1 = [
          o1js_1.Int64.from(2),
          o1js_1.Int64.from(4),
          o1js_1.Int64.from(3),
          o1js_1.Int64.from(1),
          o1js_1.Int64.from(5),
        ];
        const bias1 = o1js_1.Int64.from(3);
        const a1 = perceptron(input, weights1, bias1);
        // 두 번째 히든 레이어
        const weights2 = [
          o1js_1.Int64.from(3),
          o1js_1.Int64.from(1),
          o1js_1.Int64.from(4),
          o1js_1.Int64.from(2),
          o1js_1.Int64.from(6),
        ];
        const bias2 = o1js_1.Int64.from(2);
        const a2 = perceptron([a1, a1, a1, a1, a1], weights2, bias2); // 각 z1 값을 복제해서 전달
        // 출력 레이어
        const weights3 = [o1js_1.Int64.from(1)]; // 활성화 함수 출력값 하나에 대한 가중치
        const bias3 = o1js_1.Int64.from(5);
        const z3 = linearLayer([a2], weights3, bias3);
        // 최종 출력값 반환
        return z3;
      },
    },
  },
});
(async () => {
  console.log("start");
  // 입력 데이터 (5개의 입력값)
  let input = [
    o1js_1.Int64.from(25),
    o1js_1.Int64.from(15),
    o1js_1.Int64.from(10),
    o1js_1.Int64.from(5),
    o1js_1.Int64.from(3),
  ];
  // 증명 키 컴파일
  const { verificationKey } = await MLP.compile();
  console.log("making proof");
  // 예측 수행
  const proof = await MLP.predict(input);
  console.log("proof created: ", proof.proof);
  console.log("value: ", proof.publicOutput.toString());
  // // 증명 검증 함수
  // const verifyProof = async (proof, verificationKey) => {
  //     return await (0, o1js_1.verify)(proof, verificationKey);
  // };
  // // 검증 수행
  // const isValid = await verifyProof(proof, verificationKey);
  // console.log("Proof is valid:", isValid);
})();
