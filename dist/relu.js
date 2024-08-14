"use strict";
// Creation Date: 2023-10-15
// Last Update: 2023-12-30
// Creator: only4sim
// relu function for Snarky-ML
// relu takes a Field and outputs a Field.
Object.defineProperty(exports, "__esModule", { value: true });
exports.relu = void 0;
const o1js_1 = require("o1js");
const relu = (input) => {
    return o1js_1.Provable.if(input.isPositive(), input, o1js_1.Int64.zero);
};
exports.relu = relu;
