import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

export const ICU4XWordBreakRule_js_to_rust = {
  "Normal": 0,
  "BreakAll": 1,
  "KeepAll": 2,
};

export const ICU4XWordBreakRule_rust_to_js = {
  0: "Normal",
  1: "BreakAll",
  2: "KeepAll",
};

export const ICU4XWordBreakRule = {
  "Normal": "Normal",
  "BreakAll": "BreakAll",
  "KeepAll": "KeepAll",
};
