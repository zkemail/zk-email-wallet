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
import { hash_to_curve, point_scalar_mul } from "circom-grumpkin";
// const grumpkin = require("circom-grumpkin");
jest.setTimeout(120000);
describe("Account Creation", () => {
    it("create an account", async () => {
        const emailAddr = "suegamisora@gmail.com";
        const paddedEmailAddr = emailWalletUtils.paddedEmailAddr(emailAddr);
        const relayerRand = emailWalletUtils.genRelayerRand();
        const viewingKey = emailWalletUtils.genViewingKey();
        const circuitInputs = {
            email_addr: paddedEmailAddr,
            relayer_rand: relayerRand,
            viewing_key: viewingKey,
        };
        const circuit = await wasm_tester(path.join(__dirname, "../src/account_creation.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        const expectedRelayerRandHash = emailWalletUtils.relayerRandHash(relayerRand);
        // expect(expectedRelayerRandHash).toEqual("0x" + witness[1].toString(16));
        expect(BigInt(expectedRelayerRandHash)).toEqual(witness[1]);
        const expectedEmailAddrPointer = emailWalletUtils.emailAddrPointer(emailAddr, relayerRand);
        expect(BigInt(expectedEmailAddrPointer)).toEqual(witness[2]);
        const expectedVkCommit = emailWalletUtils.viewingKeyCommit(viewingKey, emailAddr, expectedRelayerRandHash);
        // expect(expectedVkCommit).toEqual("0x" + witness[2].toString(16));
        expect(BigInt(expectedVkCommit)).toEqual(witness[3]);
        const expectedWalletSalt = emailWalletUtils.walletSalt(viewingKey);
        expect(BigInt(expectedWalletSalt)).toEqual(witness[4]);
        const expectedExtAccountSalt = emailWalletUtils.extAccountSalt(viewingKey);
        expect(BigInt(expectedExtAccountSalt)).toEqual(witness[5]);
        const hashedPoint = hash_to_curve(paddedEmailAddr);
        const expectedPsiPoint = point_scalar_mul(hashedPoint.x, hashedPoint.y, BigInt(relayerRand));
        expect(expectedPsiPoint.x).toEqual(witness[6]);
        expect(expectedPsiPoint.y).toEqual(witness[7]);
    });
});