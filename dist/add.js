"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const o1js_1 = require("o1js");
const Add = (0, o1js_1.ZkProgram)({
    name: 'add-example',
    publicInput: o1js_1.Field,
    methods: {
        init: {
            privateInputs: [],
            async method(state) {
                state.assertEquals((0, o1js_1.Field)(0));
            },
        },
        addNumber: {
            privateInputs: [o1js_1.SelfProof, o1js_1.Field],
            async method(newState, earlierProof, numberToAdd) {
                earlierProof.verify();
                newState.assertEquals(earlierProof.publicInput.add(numberToAdd));
            },
        },
        add: {
            privateInputs: [o1js_1.SelfProof, o1js_1.SelfProof],
            async method(newState, earlierProof1, earlierProof2) {
                earlierProof1.verify();
                earlierProof2.verify();
                newState.assertEquals(earlierProof1.publicInput.add(earlierProof2.publicInput));
            },
        },
    },
});
async function main() {
    console.log('compiling...');
    const { verificationKey } = await Add.compile();
    console.log('making proof 0');
    const proof0 = await Add.init((0, o1js_1.Field)(0));
    console.log(`proof 0: ${proof0}`);
    //   const proof1 = await Add.addNumber(Field(4), proof0, Field(4));
    //   console.log('making proof 2');
    //   const proof2 = await Add.add(Field(4), proof1, proof0);
    //   console.log('verifying proof 2');
    //   console.log('proof 2 data', proof2.publicInput.toString());
    //   const ok = await verify(proof2.toJSON(), verificationKey);
    //   console.log('ok', ok);
}
main();
