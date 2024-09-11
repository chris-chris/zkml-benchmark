// Creation Date: 2023-10-15
// Last Update: 2023-12-30
// Creator: only4sim
// relu function for Snarky-ML
// relu takes a Field and outputs a Field.

import { Provable, UInt32 } from "o1js";

export const relu = (input: UInt32): UInt32 => {
  return Provable.if(input.greaterThan(new UInt32(0)), input, new UInt32(0));
};
