import { ZkProgram, Int64, Provable, Cache } from "o1js";

import { relu } from "./relu";
// 선형 변환을 수행하는 모듈화된 레이어 함수
function linearLayer(input: Int64[], weights: Int64[], bias: Int64): Int64 {
  let z = Int64.from(0);
  for (let i = 0; i < weights.length; i++) {
    z = z.add(weights[i].mul(input[i]));
  }
  z = z.add(bias);
  return z;
}

// 퍼셉트론 레이어를 처리하는 함수
function perceptron(input: Int64[], weights: Int64[], bias: Int64): Int64 {
  const z = linearLayer(input, weights, bias);
  return relu(z); // ReLU 활성화 함수 적용
}

// MLP 모델 정의
function createMLPProgram(depth: number) {
  return ZkProgram({
    name: `Simple_Depth_${depth}`,
    publicOutput: Int64,
    methods: {
      predict: {
        privateInputs: [Provable.Array(Int64, 1)], // 4개의 입력값
        async method(input: Int64[]): Promise<Int64> {
          let a = input;

          const weightsOut = Int64.from(1);
          const biasOut = Int64.from(1);
          let zOut = Int64.from(0);
          //   const zOut = linearLayer(a, weightsOut, biasOut);
          for (let i = 0; i < a.length; i++) {
            zOut = zOut.add(a[i]);
          }
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
  let input = [Int64.from(0)];

  // MLP 실행
  const { verificationKey } = await MLP.compile({
    cache: Cache.FileSystemDefault,
    forceRecompile: false,
  });
  console.log(`Making proof for MLP with depth ${depth}...`);
  const predict = (await MLP.analyzeMethods()).predict;
  const gates = predict.gates;
  const summary = predict.summary();
  console.log(
    `circuit summary: ${summary["Total rows"]} ${summary.EndoMulScalar}`
  );
  console.log(`circuit gates: ${gates.length}`);

  // 각 게이트의 세부 정보를 출력
  // gates.forEach((gate, index) => {
  //   console.log(`Gate ${index + 1}:`);
  //   console.log(`  Type: ${gate.type}`);
  //   console.log(`  Wires: ${gate.wires.length}`);
  //   gate.wires.forEach((wire, i) => {
  //     console.log(`    Wire ${i + 1}: col ${wire.col} row ${wire.col}`);
  //   });
  //   console.log(`  Coeffs: ${gate.coeffs}`);
  // });

  const proof = await MLP.predict(input);
  // console.log(`Proof created for MLP with depth ${depth}: `, proof.proof);
  console.log("Value: ", proof.publicOutput.toString());

  console.log("proof: ", proof);
  console.profileEnd();

  // const isValid = await verify(proof, verificationKey);
  // console.log(`Proof is valid for depth ${depth}:`, isValid);
})();
