"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const o1js_1 = require("o1js");
const relu_1 = require("./relu");
var expNum = 0;
var depth = -1;
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
                            o1js_1.UInt32.from(1),
                            o1js_1.UInt32.from(1),
                            o1js_1.UInt32.from(1),
                            o1js_1.UInt32.from(1),
                        ];
                        const bias = o1js_1.UInt32.from(1);
                        a = [
                            perceptron(a, weights, bias),
                            perceptron(a, weights, bias),
                            perceptron(a, weights, bias),
                            perceptron(a, weights, bias),
                        ];
                    }
                    const weightsOut = [
                        o1js_1.UInt32.from(1),
                        o1js_1.UInt32.from(2),
                        o1js_1.UInt32.from(3),
                        o1js_1.UInt32.from(4),
                    ];
                    const biasOut = o1js_1.UInt32.from(0);
                    const zOut = linearLayer(a, weightsOut, biasOut);
                    return zOut;
                },
            },
        },
    });
}
// SecondMLPPrograms
// SecondMLPProgram 정의: Array 크기를 2^i로 설정한 10개의 함수
// const SecondMLPPrograms = Array.from({ length: 10 }, (_, i) =>
//   ZkProgram({
//     name: `SecondMLP_${i + 1}`,
//     publicOutput: UInt32,
//     methods: {
//       computeFinal: {
//         privateInputs: [Provable.Array(UInt32, 2 ** (i + 1))], // 'depth' 개의 입력값
//         async method(inputs: UInt32[]): Promise<UInt32> {
//           const weightsOut: UInt32[] = Array(2 ** (i + 1)).fill(UInt32.from(2));
//           const biasOut = UInt32.from(0);
//           const finalOutput = linearLayer(inputs, weightsOut, biasOut);
//           return finalOutput;
//         },
//       },
//     },
//   })
// );
function createSecondProgram(depth) {
    return (0, o1js_1.ZkProgram)({
        name: `SecondMLP_${depth}`,
        publicOutput: o1js_1.UInt32,
        methods: {
            computeFinal: {
                privateInputs: [o1js_1.Provable.Array(o1js_1.UInt32, depth)], // 'depth' 개의 입력값
                async method(inputs) {
                    const weightsOut = Array(depth).fill(o1js_1.UInt32.from(2));
                    const biasOut = o1js_1.UInt32.from(0);
                    const finalOutput = linearLayer(inputs, weightsOut, biasOut);
                    return finalOutput;
                },
            },
        },
    });
}
// 모델 사용 예제
(async () => {
    var startDate = new Date();
    const args = process.argv.slice(2); // 명령줄 인수 받기
    expNum = parseInt(args[0], 10); // 첫 번째 인수를 depth로 사용
    depth = 2 ** expNum;
    console.log(`Creating FirstMLP model with depth ${depth}...`);
    // 첫 번째 MLP 모델 생성
    const FirstMLP = createMLPProgram(1); // 첫 번째 MLP는 한 번만 수행
    var seconds = (new Date().getTime() - startDate.getTime()) / 1000;
    console.log(`seconds: ${seconds}s`);
    // 입력 데이터 (4개의 입력값)
    const input = [
        o1js_1.UInt32.from(5),
        o1js_1.UInt32.from(3),
        o1js_1.UInt32.from(1),
        o1js_1.UInt32.from(0),
    ];
    // MLP 실행
    const { verificationKey: vk1 } = await FirstMLP.compile({
        cache: o1js_1.Cache.FileSystemDefault,
        forceRecompile: false,
    });
    console.log(`Making proof for FirstMLP...`);
    var seconds = (new Date().getTime() - startDate.getTime()) / 1000;
    console.log(`seconds: ${seconds}s`);
    const singleProof = await FirstMLP.predict(input);
    const singleOutput = singleProof.publicOutput;
    var seconds = (new Date().getTime() - startDate.getTime()) / 1000;
    console.log(`seconds: ${seconds}s`);
    // 첫 번째 MLP 결과를 depth 만큼 복사
    const inputsArray = Array(depth).fill(singleOutput);
    // const proofsArray: SelfProof<undefined, UInt32>[] =
    //   Array(depth).fill(singleProof);
    console.log(`First proof and output generated and copied ${depth} times.`);
    var seconds = (new Date().getTime() - startDate.getTime()) / 1000;
    console.log(`seconds: ${seconds}s`);
    // 해당하는 SecondMLP 프로그램 선택
    console.log(`\nCreating SecondMLP_${expNum} model...`);
    // const SecondMLPProgram = SecondMLPPrograms[expNum - 1]; // expNum에 따라 프로그램 선택
    const SecondPro = createSecondProgram(depth);
    // SecondMLP 프로그램 컴파일
    const { verificationKey: vk2 } = await SecondPro.compile({
        cache: o1js_1.Cache.FileSystemDefault,
        forceRecompile: false,
    });
    console.log(`Generating final proof for SecondMLP_${expNum}...`);
    var seconds = (new Date().getTime() - startDate.getTime()) / 1000;
    console.log(`seconds: ${seconds}s`);
    const finalProof = await SecondPro.computeFinal(inputsArray);
    // 증명 검증
    // console.log(`\nVerifying all the proofs, proof count: ${proofsArray.length + 1}`);
    // for (let i = 0; i < proofsArray.length; i++) {
    //   await verify(proofsArray[i].toJSON(), vk1);
    //}
    // const isValidFinalProof = await verify(finalProof.toJSON(), vk2);
    // console.log(`Final proof is valid:`, isValidFinalProof);
})();
