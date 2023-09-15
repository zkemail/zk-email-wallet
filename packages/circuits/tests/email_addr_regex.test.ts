const utils = require("../../utils");
const ff = require('ffjavascript');
const stringifyBigInts = ff.utils.stringifyBigInts;
const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
const p = "21888242871839275222246405745257275088548364400416034343698204186575808495617";
const field = new ff.F1Field(p);
const emailWalletUtils = require("../../utils");
const option = {
    include: path.join(__dirname, "../../../node_modules")
};
// const grumpkin = require("circom-grumpkin");
jest.setTimeout(120000);
describe("Email Address Regex", () => {
    it("test a regex of an email address", async () => {
        const emailAddr = "suegamisora@gmail.com";
        const paddedEmailAddr = emailWalletUtils.paddedEmailAddr(emailAddr);
        const circuitInputs = {
            msg: paddedEmailAddr,
        };
        const circuit = await wasm_tester(path.join(__dirname, "./circuits/test_email_addr_regex.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        expect(1n).toEqual(witness[1]);
        for (let idx = 0; idx < emailAddr.length; ++idx) {
            expect(BigInt(paddedEmailAddr[idx])).toEqual(witness[2 + idx]);
        }
    });
});