import {
  Field,
  ZkProgram,
  UInt32,
  Provable,
  verify,
  Cache,
  SelfProof,
} from "o1js";

import { relu } from "./relu";

var expNum = 0;
var depth = -1;

// 선형 변환을 수행하는 모듈화된 레이어 함수
function linearLayer(input: UInt32[], weights: UInt32[], bias: UInt32): UInt32 {
  let z = UInt32.from(0);
  for (let i = 0; i < weights.length; i++) {
    z = z.add(weights[i].mul(input[i]));
  }
  z = z.add(bias);
  return z;
}

// 퍼셉트론 레이어를 처리하는 함수
function perceptron(input: UInt32[], weights: UInt32[], bias: UInt32): UInt32 {
  const z = linearLayer(input, weights, bias);
  return relu(z); // ReLU 활성화 함수 적용
}

// MLP 모델 정의
function createMLPProgram(depth: number) {
  return ZkProgram({
    name: `MLP_Depth_${depth}`,
    publicOutput: UInt32,
    methods: {
      predict: {
        privateInputs: [Provable.Array(UInt32, 4)], // 4개의 입력값
        async method(input: UInt32[]): Promise<UInt32> {
          let a = input;
          for (let i = 0; i < depth; i++) {
            const weights = [
              UInt32.from(1),
              UInt32.from(1),
              UInt32.from(1),
              UInt32.from(1),
            ];
            const bias = UInt32.from(1);
            a = [
              perceptron(a, weights, bias),
              perceptron(a, weights, bias),
              perceptron(a, weights, bias),
              perceptron(a, weights, bias),
            ];
          }

          const weightsOut = [
            UInt32.from(1),
            UInt32.from(2),
            UInt32.from(3),
            UInt32.from(4),
          ];
          const biasOut = UInt32.from(0);
          const zOut = linearLayer(a, weightsOut, biasOut);

          return zOut;
        },
      },
    },
  });
}

// SecondMLPPrograms
const SecondMLPProgram = ZkProgram({
  name: `SecondMLP_${i + 1}`,
  publicOutput: UInt32,

  methods: {
    computeFinal: {
      privateInputs: [Provable.Array(UInt32, 2 ** (i + 1))], // 'depth' 개의 입력값
      async method(inputs: UInt32[]): Promise<UInt32> {
        const weightsOut: UInt32[] = Array(2 ** (i + 1)).fill(UInt32.from(2));
        const biasOut = UInt32.from(0);
        const finalOutput = linearLayer(inputs, weightsOut, biasOut);

        return finalOutput;
      },
    },
  },
});

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

// 모델 사용 예제
(async () => {
  const args = process.argv.slice(2); // 명령줄 인수 받기
  expNum = parseInt(args[0], 10); // 첫 번째 인수를 depth로 사용
  depth = 2 ** expNum;

  console.log(`Creating FirstMLP model with depth ${depth}...`);

  // 첫 번째 MLP 모델 생성
  const FirstMLP = createMLPProgram(1); // 첫 번째 MLP는 한 번만 수행

  // 입력 데이터 (4개의 입력값)
  const input = [
    UInt32.from(5),
    UInt32.from(3),
    UInt32.from(1),
    UInt32.from(0),
  ];

  // MLP 실행
  const { verificationKey: vk1 } = await FirstMLP.compile({
    cache: Cache.FileSystemDefault,
    forceRecompile: false,
  });
  console.log(`Making proof for FirstMLP...`);

  const singleProof = await FirstMLP.predict(input);
  const singleOutput = singleProof.publicOutput;

  // 첫 번째 MLP 결과를 depth 만큼 복사
  const inputsArray: UInt32[] = Array(depth).fill(singleOutput);
  // const proofsArray: SelfProof<undefined, UInt32>[] =
  //   Array(depth).fill(singleProof);
  console.log(`First proof and output generated and copied ${depth} times.`);

  // 해당하는 SecondMLP 프로그램 선택
  console.log(`\nCreating SecondMLP_${expNum} model...`);
  // const SecondMLPProgram = SecondMLPPrograms[expNum - 1]; // expNum에 따라 프로그램 선택

  // SecondMLP 프로그램 컴파일
  const { verificationKey: vk2 } = await SecondMLPProgram.compile({
    cache: Cache.FileSystemDefault,
    forceRecompile: false,
  });
  console.log(`Generating final proof for SecondMLP_${expNum}...`);

  const finalProof = await SecondMLPProgram.computeFinal(inputsArray);

  // 증명 검증
  // console.log(`\nVerifying all the proofs, proof count: ${proofsArray.length + 1}`);
  // for (let i = 0; i < proofsArray.length; i++) {
  //   await verify(proofsArray[i].toJSON(), vk1);
  //}
  // const isValidFinalProof = await verify(finalProof.toJSON(), vk2);

  // console.log(`Final proof is valid:`, isValidFinalProof);
})();
