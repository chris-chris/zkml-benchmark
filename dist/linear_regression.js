"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const o1js_1 = require("o1js");
// 선형 회귀 모델 정의
const LinearRegression = (0, o1js_1.ZkProgram)({
    name: 'LinearRegression',
    publicOutput: o1js_1.Int64,
    methods: {
        predict: {
            privateInputs: [o1js_1.Provable.Array(o1js_1.Int64, 1)],
            async method(input) {
                const coefficients = [o1js_1.Int64.from(2), o1js_1.Int64.from(4)];
                // 예측 수행
                let dotProduct = o1js_1.Int64.from(0);
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
    let input = [o1js_1.Int64.from(25)];
    // 증명 키 컴파일
    const { verificationKey } = await LinearRegression.compile();
    console.log('making proof');
    // 예측 수행
    const proof = await LinearRegression.predict(input);
    console.log('proof created');
    console.log('value: ', proof.publicOutput.toString());
    // 증명 검증 함수
    const verifyProof = async (proof, verificationKey) => {
        return await (0, o1js_1.verify)(proof, verificationKey);
    };
    // 검증 수행
    const isValid = await verifyProof(proof, verificationKey);
    console.log('Proof is valid:', isValid);
})();
