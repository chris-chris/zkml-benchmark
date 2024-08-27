"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const o1js_1 = require("o1js");
const fs = __importStar(require("fs/promises"));
// ReLU 활성화 함수
function relu(x) {
    return o1js_1.Int64.from(x > o1js_1.Int64.from(0) ? x : 0);
}
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
    return relu(z); // ReLU 활성화 함수 적용
}
// 100개의 작업을 미리 수행하는 함수
function process100Tasks(input) {
    const results = [];
    for (let i = 0; i < input.length; i++) {
        const weights = Array(input[i].length).fill(o1js_1.Int64.from(1)); // 임의의 가중치
        const bias = o1js_1.Int64.from(1); // 임의의 bias
        results.push(perceptron(input[i], weights, bias));
    }
    return results;
}
// 100개의 결과를 집계하여 최종 결과를 출력하는 함수
function aggregateResults(results) {
    const weights = Array(results.length).fill(o1js_1.Int64.from(1)); // 임의의 가중치
    const bias = o1js_1.Int64.from(0); // 임의의 bias
    return linearLayer(results, weights, bias);
}
// MLP 모델 정의
function createMLPProgram() {
    return (0, o1js_1.ZkProgram)({
        name: "MLP_Parallel",
        publicOutput: o1js_1.Int64,
        methods: {
            predict: {
                privateInputs: [o1js_1.Provable.Array(o1js_1.Provable.Array(o1js_1.Int64, 5), 100)], // 100개의 5차원 입력값
                async method(input) {
                    // 100개의 작업을 미리 수행
                    const processedResults = process100Tasks(input);
                    // 집계하여 최종 결과 도출
                    const finalResult = aggregateResults(processedResults);
                    return finalResult;
                },
            },
        },
    });
}
// 모델 사용 예제
(async () => {
    // 입력 데이터 (input.json 파일에서 가져옴)
    const inputJson = await fs.readFile("input.json", "utf-8");
    const inputData = JSON.parse(inputJson).input_data;
    // 입력 데이터를 Int64[][] 형식으로 변환
    const input = inputData.map((arr) => arr.map((num) => o1js_1.Int64.from(Math.floor(num * 100))) // 소수점을 처리하기 위해 100배 스케일링
    );
    console.log(`Creating MLP model with parallel processing...`);
    // MLP 모델 생성
    const MLP = createMLPProgram();
    // MLP 실행
    const { verificationKey } = await MLP.compile({
        cache: o1js_1.Cache.FileSystemDefault,
        forceRecompile: false,
    });
    console.log(`Making proof for MLP with parallel processing...`);
    const proof = await MLP.predict(input);
    console.log("Value: ", proof.publicOutput.toString());
    // const isValid = await verify(proof, verificationKey);
    // console.log(`Proof is valid:`, isValid);
})();
