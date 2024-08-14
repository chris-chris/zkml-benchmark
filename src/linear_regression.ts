import { Field, ZkProgram, Int64, Provable, verify } from 'o1js';
import * as fs from 'fs/promises';
// 선형 회귀 모델 정의

const LinearRegression = ZkProgram({
  name: 'LinearRegression',
  publicOutput: Int64,
  methods: {
    predict: {
      privateInputs: [Provable.Array(Int64, 1)],
      async method(input: Int64[]): Promise<Int64> {

        const coefficients: Int64[] = [Int64.from(2), Int64.from(4)];

        // 예측 수행
        let dotProduct = Int64.from(0);
        for (let i = 0; i < coefficients.length - 1; i++) {
          dotProduct = dotProduct.add(coefficients[i].mul(input[i]));
        }

        const intercept = coefficients[coefficients.length - 1];
        const z = dotProduct.add(intercept);

        return z;
      },
    },
  },
});


(async () => {
  console.log("start");

  // 입력 데이터
  let input = [Int64.from(25)];

  // 증명 키 컴파일
  const { verificationKey } = await LinearRegression.compile();
  console.log('making proof');

  // 예측 수행
  const proof = await LinearRegression.predict(input);
  console.log('proof created');
  console.log('value: ', proof.publicOutput.toString());

  // 증명 검증 함수
  const verifyProof = async (proof: any, verificationKey: any) => {
    return await verify(proof, verificationKey);
  };

  // 검증 수행
  const isValid = await verifyProof(proof, verificationKey);
  console.log('Proof is valid:', isValid);
})();

