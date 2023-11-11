// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  ethereum,
  JSONValue,
  TypedMap,
  Entity,
  Bytes,
  Address,
  BigInt
} from "@graphprotocol/graph-ts";

export class AccountCreated extends ethereum.Event {
  get params(): AccountCreated__Params {
    return new AccountCreated__Params(this);
  }
}

export class AccountCreated__Params {
  _event: AccountCreated;

  constructor(event: AccountCreated) {
    this._event = event;
  }

  get emailAddrPointer(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }

  get accountKeyCommit(): Bytes {
    return this._event.parameters[1].value.toBytes();
  }

  get walletSalt(): Bytes {
    return this._event.parameters[2].value.toBytes();
  }

  get psiPoint(): Bytes {
    return this._event.parameters[3].value.toBytes();
  }
}

export class AccountInitialized extends ethereum.Event {
  get params(): AccountInitialized__Params {
    return new AccountInitialized__Params(this);
  }
}

export class AccountInitialized__Params {
  _event: AccountInitialized;

  constructor(event: AccountInitialized) {
    this._event = event;
  }

  get emailAddrPointer(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }

  get accountKeyCommit(): Bytes {
    return this._event.parameters[1].value.toBytes();
  }

  get walletSalt(): Bytes {
    return this._event.parameters[2].value.toBytes();
  }
}

export class AccountTransported extends ethereum.Event {
  get params(): AccountTransported__Params {
    return new AccountTransported__Params(this);
  }
}

export class AccountTransported__Params {
  _event: AccountTransported;

  constructor(event: AccountTransported) {
    this._event = event;
  }

  get oldAccountKeyCommit(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }

  get newEmailAddrPointer(): Bytes {
    return this._event.parameters[1].value.toBytes();
  }

  get newAccountKeyCommit(): Bytes {
    return this._event.parameters[2].value.toBytes();
  }

  get newPSIPoint(): Bytes {
    return this._event.parameters[3].value.toBytes();
  }
}

export class OwnershipTransferred extends ethereum.Event {
  get params(): OwnershipTransferred__Params {
    return new OwnershipTransferred__Params(this);
  }
}

export class OwnershipTransferred__Params {
  _event: OwnershipTransferred;

  constructor(event: OwnershipTransferred) {
    this._event = event;
  }

  get previousOwner(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get newOwner(): Address {
    return this._event.parameters[1].value.toAddress();
  }
}

export class AccountHandler__getInfoOfAccountKeyCommitResultValue0Struct extends ethereum.Tuple {
  get relayer(): Address {
    return this[0].toAddress();
  }

  get initialized(): boolean {
    return this[1].toBoolean();
  }

  get walletSalt(): Bytes {
    return this[2].toBytes();
  }
}

export class AccountHandler__infoOfAccountKeyCommitResult {
  value0: Address;
  value1: boolean;
  value2: Bytes;

  constructor(value0: Address, value1: boolean, value2: Bytes) {
    this.value0 = value0;
    this.value1 = value1;
    this.value2 = value2;
  }

  toMap(): TypedMap<string, ethereum.Value> {
    let map = new TypedMap<string, ethereum.Value>();
    map.set("value0", ethereum.Value.fromAddress(this.value0));
    map.set("value1", ethereum.Value.fromBoolean(this.value1));
    map.set("value2", ethereum.Value.fromFixedBytes(this.value2));
    return map;
  }

  getRelayer(): Address {
    return this.value0;
  }

  getInitialized(): boolean {
    return this.value1;
  }

  getWalletSalt(): Bytes {
    return this.value2;
  }
}

export class AccountHandler extends ethereum.SmartContract {
  static bind(address: Address): AccountHandler {
    return new AccountHandler("AccountHandler", address);
  }

  accountKeyCommitOfPointer(param0: Bytes): Bytes {
    let result = super.call(
      "accountKeyCommitOfPointer",
      "accountKeyCommitOfPointer(bytes32):(bytes32)",
      [ethereum.Value.fromFixedBytes(param0)]
    );

    return result[0].toBytes();
  }

  try_accountKeyCommitOfPointer(param0: Bytes): ethereum.CallResult<Bytes> {
    let result = super.tryCall(
      "accountKeyCommitOfPointer",
      "accountKeyCommitOfPointer(bytes32):(bytes32)",
      [ethereum.Value.fromFixedBytes(param0)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBytes());
  }

  createAccount(
    emailAddrPointer: Bytes,
    accountKeyCommit: Bytes,
    walletSalt: Bytes,
    psiPoint: Bytes,
    proof: Bytes
  ): Address {
    let result = super.call(
      "createAccount",
      "createAccount(bytes32,bytes32,bytes32,bytes,bytes):(address)",
      [
        ethereum.Value.fromFixedBytes(emailAddrPointer),
        ethereum.Value.fromFixedBytes(accountKeyCommit),
        ethereum.Value.fromFixedBytes(walletSalt),
        ethereum.Value.fromBytes(psiPoint),
        ethereum.Value.fromBytes(proof)
      ]
    );

    return result[0].toAddress();
  }

  try_createAccount(
    emailAddrPointer: Bytes,
    accountKeyCommit: Bytes,
    walletSalt: Bytes,
    psiPoint: Bytes,
    proof: Bytes
  ): ethereum.CallResult<Address> {
    let result = super.tryCall(
      "createAccount",
      "createAccount(bytes32,bytes32,bytes32,bytes,bytes):(address)",
      [
        ethereum.Value.fromFixedBytes(emailAddrPointer),
        ethereum.Value.fromFixedBytes(accountKeyCommit),
        ethereum.Value.fromFixedBytes(walletSalt),
        ethereum.Value.fromBytes(psiPoint),
        ethereum.Value.fromBytes(proof)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  defaultDkimRegistry(): Address {
    let result = super.call(
      "defaultDkimRegistry",
      "defaultDkimRegistry():(address)",
      []
    );

    return result[0].toAddress();
  }

  try_defaultDkimRegistry(): ethereum.CallResult<Address> {
    let result = super.tryCall(
      "defaultDkimRegistry",
      "defaultDkimRegistry():(address)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  dkimRegistryOfWalletSalt(param0: Bytes): Address {
    let result = super.call(
      "dkimRegistryOfWalletSalt",
      "dkimRegistryOfWalletSalt(bytes32):(address)",
      [ethereum.Value.fromFixedBytes(param0)]
    );

    return result[0].toAddress();
  }

  try_dkimRegistryOfWalletSalt(param0: Bytes): ethereum.CallResult<Address> {
    let result = super.tryCall(
      "dkimRegistryOfWalletSalt",
      "dkimRegistryOfWalletSalt(bytes32):(address)",
      [ethereum.Value.fromFixedBytes(param0)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  emailNullifiers(param0: Bytes): boolean {
    let result = super.call(
      "emailNullifiers",
      "emailNullifiers(bytes32):(bool)",
      [ethereum.Value.fromFixedBytes(param0)]
    );

    return result[0].toBoolean();
  }

  try_emailNullifiers(param0: Bytes): ethereum.CallResult<boolean> {
    let result = super.tryCall(
      "emailNullifiers",
      "emailNullifiers(bytes32):(bool)",
      [ethereum.Value.fromFixedBytes(param0)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBoolean());
  }

  emailValidityDuration(): BigInt {
    let result = super.call(
      "emailValidityDuration",
      "emailValidityDuration():(uint256)",
      []
    );

    return result[0].toBigInt();
  }

  try_emailValidityDuration(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "emailValidityDuration",
      "emailValidityDuration():(uint256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  getInfoOfAccountKeyCommit(
    accountKeyCommit: Bytes
  ): AccountHandler__getInfoOfAccountKeyCommitResultValue0Struct {
    let result = super.call(
      "getInfoOfAccountKeyCommit",
      "getInfoOfAccountKeyCommit(bytes32):((address,bool,bytes32))",
      [ethereum.Value.fromFixedBytes(accountKeyCommit)]
    );

    return changetype<
      AccountHandler__getInfoOfAccountKeyCommitResultValue0Struct
    >(result[0].toTuple());
  }

  try_getInfoOfAccountKeyCommit(
    accountKeyCommit: Bytes
  ): ethereum.CallResult<
    AccountHandler__getInfoOfAccountKeyCommitResultValue0Struct
  > {
    let result = super.tryCall(
      "getInfoOfAccountKeyCommit",
      "getInfoOfAccountKeyCommit(bytes32):((address,bool,bytes32))",
      [ethereum.Value.fromFixedBytes(accountKeyCommit)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(
      changetype<AccountHandler__getInfoOfAccountKeyCommitResultValue0Struct>(
        value[0].toTuple()
      )
    );
  }

  getWalletOfEmailAddrPointer(emailAddrPointer: Bytes): Address {
    let result = super.call(
      "getWalletOfEmailAddrPointer",
      "getWalletOfEmailAddrPointer(bytes32):(address)",
      [ethereum.Value.fromFixedBytes(emailAddrPointer)]
    );

    return result[0].toAddress();
  }

  try_getWalletOfEmailAddrPointer(
    emailAddrPointer: Bytes
  ): ethereum.CallResult<Address> {
    let result = super.tryCall(
      "getWalletOfEmailAddrPointer",
      "getWalletOfEmailAddrPointer(bytes32):(address)",
      [ethereum.Value.fromFixedBytes(emailAddrPointer)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  getWalletOfSalt(salt: Bytes): Address {
    let result = super.call(
      "getWalletOfSalt",
      "getWalletOfSalt(bytes32):(address)",
      [ethereum.Value.fromFixedBytes(salt)]
    );

    return result[0].toAddress();
  }

  try_getWalletOfSalt(salt: Bytes): ethereum.CallResult<Address> {
    let result = super.tryCall(
      "getWalletOfSalt",
      "getWalletOfSalt(bytes32):(address)",
      [ethereum.Value.fromFixedBytes(salt)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  infoOfAccountKeyCommit(
    param0: Bytes
  ): AccountHandler__infoOfAccountKeyCommitResult {
    let result = super.call(
      "infoOfAccountKeyCommit",
      "infoOfAccountKeyCommit(bytes32):(address,bool,bytes32)",
      [ethereum.Value.fromFixedBytes(param0)]
    );

    return new AccountHandler__infoOfAccountKeyCommitResult(
      result[0].toAddress(),
      result[1].toBoolean(),
      result[2].toBytes()
    );
  }

  try_infoOfAccountKeyCommit(
    param0: Bytes
  ): ethereum.CallResult<AccountHandler__infoOfAccountKeyCommitResult> {
    let result = super.tryCall(
      "infoOfAccountKeyCommit",
      "infoOfAccountKeyCommit(bytes32):(address,bool,bytes32)",
      [ethereum.Value.fromFixedBytes(param0)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(
      new AccountHandler__infoOfAccountKeyCommitResult(
        value[0].toAddress(),
        value[1].toBoolean(),
        value[2].toBytes()
      )
    );
  }

  isDKIMPublicKeyHashValid(
    walletSalt: Bytes,
    emailDomain: string,
    publicKeyHash: Bytes
  ): boolean {
    let result = super.call(
      "isDKIMPublicKeyHashValid",
      "isDKIMPublicKeyHashValid(bytes32,string,bytes32):(bool)",
      [
        ethereum.Value.fromFixedBytes(walletSalt),
        ethereum.Value.fromString(emailDomain),
        ethereum.Value.fromFixedBytes(publicKeyHash)
      ]
    );

    return result[0].toBoolean();
  }

  try_isDKIMPublicKeyHashValid(
    walletSalt: Bytes,
    emailDomain: string,
    publicKeyHash: Bytes
  ): ethereum.CallResult<boolean> {
    let result = super.tryCall(
      "isDKIMPublicKeyHashValid",
      "isDKIMPublicKeyHashValid(bytes32,string,bytes32):(bool)",
      [
        ethereum.Value.fromFixedBytes(walletSalt),
        ethereum.Value.fromString(emailDomain),
        ethereum.Value.fromFixedBytes(publicKeyHash)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBoolean());
  }

  owner(): Address {
    let result = super.call("owner", "owner():(address)", []);

    return result[0].toAddress();
  }

  try_owner(): ethereum.CallResult<Address> {
    let result = super.tryCall("owner", "owner():(address)", []);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  pointerOfPSIPoint(param0: Bytes): Bytes {
    let result = super.call(
      "pointerOfPSIPoint",
      "pointerOfPSIPoint(bytes):(bytes32)",
      [ethereum.Value.fromBytes(param0)]
    );

    return result[0].toBytes();
  }

  try_pointerOfPSIPoint(param0: Bytes): ethereum.CallResult<Bytes> {
    let result = super.tryCall(
      "pointerOfPSIPoint",
      "pointerOfPSIPoint(bytes):(bytes32)",
      [ethereum.Value.fromBytes(param0)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBytes());
  }

  relayerHandler(): Address {
    let result = super.call("relayerHandler", "relayerHandler():(address)", []);

    return result[0].toAddress();
  }

  try_relayerHandler(): ethereum.CallResult<Address> {
    let result = super.tryCall(
      "relayerHandler",
      "relayerHandler():(address)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  verifier(): Address {
    let result = super.call("verifier", "verifier():(address)", []);

    return result[0].toAddress();
  }

  try_verifier(): ethereum.CallResult<Address> {
    let result = super.tryCall("verifier", "verifier():(address)", []);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  walletImplementation(): Address {
    let result = super.call(
      "walletImplementation",
      "walletImplementation():(address)",
      []
    );

    return result[0].toAddress();
  }

  try_walletImplementation(): ethereum.CallResult<Address> {
    let result = super.tryCall(
      "walletImplementation",
      "walletImplementation():(address)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }
}

export class ConstructorCall extends ethereum.Call {
  get inputs(): ConstructorCall__Inputs {
    return new ConstructorCall__Inputs(this);
  }

  get outputs(): ConstructorCall__Outputs {
    return new ConstructorCall__Outputs(this);
  }
}

export class ConstructorCall__Inputs {
  _call: ConstructorCall;

  constructor(call: ConstructorCall) {
    this._call = call;
  }

  get _relayerHandler(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get _defaultDkimRegistry(): Address {
    return this._call.inputValues[1].value.toAddress();
  }

  get _verifier(): Address {
    return this._call.inputValues[2].value.toAddress();
  }

  get _walletImplementation(): Address {
    return this._call.inputValues[3].value.toAddress();
  }

  get _emailValidityDuration(): BigInt {
    return this._call.inputValues[4].value.toBigInt();
  }
}

export class ConstructorCall__Outputs {
  _call: ConstructorCall;

  constructor(call: ConstructorCall) {
    this._call = call;
  }
}

export class CreateAccountCall extends ethereum.Call {
  get inputs(): CreateAccountCall__Inputs {
    return new CreateAccountCall__Inputs(this);
  }

  get outputs(): CreateAccountCall__Outputs {
    return new CreateAccountCall__Outputs(this);
  }
}

export class CreateAccountCall__Inputs {
  _call: CreateAccountCall;

  constructor(call: CreateAccountCall) {
    this._call = call;
  }

  get emailAddrPointer(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get accountKeyCommit(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get walletSalt(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get psiPoint(): Bytes {
    return this._call.inputValues[3].value.toBytes();
  }

  get proof(): Bytes {
    return this._call.inputValues[4].value.toBytes();
  }
}

export class CreateAccountCall__Outputs {
  _call: CreateAccountCall;

  constructor(call: CreateAccountCall) {
    this._call = call;
  }

  get wallet(): Address {
    return this._call.outputValues[0].value.toAddress();
  }
}

export class InitializeAccountCall extends ethereum.Call {
  get inputs(): InitializeAccountCall__Inputs {
    return new InitializeAccountCall__Inputs(this);
  }

  get outputs(): InitializeAccountCall__Outputs {
    return new InitializeAccountCall__Outputs(this);
  }
}

export class InitializeAccountCall__Inputs {
  _call: InitializeAccountCall;

  constructor(call: InitializeAccountCall) {
    this._call = call;
  }

  get emailAddrPointer(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get emailDomain(): string {
    return this._call.inputValues[1].value.toString();
  }

  get emailTimestamp(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get emailNullifier(): Bytes {
    return this._call.inputValues[3].value.toBytes();
  }

  get dkimPublicKeyHash(): Bytes {
    return this._call.inputValues[4].value.toBytes();
  }

  get proof(): Bytes {
    return this._call.inputValues[5].value.toBytes();
  }
}

export class InitializeAccountCall__Outputs {
  _call: InitializeAccountCall;

  constructor(call: InitializeAccountCall) {
    this._call = call;
  }
}

export class RenounceOwnershipCall extends ethereum.Call {
  get inputs(): RenounceOwnershipCall__Inputs {
    return new RenounceOwnershipCall__Inputs(this);
  }

  get outputs(): RenounceOwnershipCall__Outputs {
    return new RenounceOwnershipCall__Outputs(this);
  }
}

export class RenounceOwnershipCall__Inputs {
  _call: RenounceOwnershipCall;

  constructor(call: RenounceOwnershipCall) {
    this._call = call;
  }
}

export class RenounceOwnershipCall__Outputs {
  _call: RenounceOwnershipCall;

  constructor(call: RenounceOwnershipCall) {
    this._call = call;
  }
}

export class TransferOwnershipCall extends ethereum.Call {
  get inputs(): TransferOwnershipCall__Inputs {
    return new TransferOwnershipCall__Inputs(this);
  }

  get outputs(): TransferOwnershipCall__Outputs {
    return new TransferOwnershipCall__Outputs(this);
  }
}

export class TransferOwnershipCall__Inputs {
  _call: TransferOwnershipCall;

  constructor(call: TransferOwnershipCall) {
    this._call = call;
  }

  get newOwner(): Address {
    return this._call.inputValues[0].value.toAddress();
  }
}

export class TransferOwnershipCall__Outputs {
  _call: TransferOwnershipCall;

  constructor(call: TransferOwnershipCall) {
    this._call = call;
  }
}

export class TransportAccountCall extends ethereum.Call {
  get inputs(): TransportAccountCall__Inputs {
    return new TransportAccountCall__Inputs(this);
  }

  get outputs(): TransportAccountCall__Outputs {
    return new TransportAccountCall__Outputs(this);
  }
}

export class TransportAccountCall__Inputs {
  _call: TransportAccountCall;

  constructor(call: TransportAccountCall) {
    this._call = call;
  }

  get oldAccountKeyCommit(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get newEmailAddrPointer(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get newAccountKeyCommit(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get newPSIPoint(): Bytes {
    return this._call.inputValues[3].value.toBytes();
  }

  get transportEmailProof(): TransportAccountCallTransportEmailProofStruct {
    return changetype<TransportAccountCallTransportEmailProofStruct>(
      this._call.inputValues[4].value.toTuple()
    );
  }

  get accountCreationProof(): Bytes {
    return this._call.inputValues[5].value.toBytes();
  }
}

export class TransportAccountCall__Outputs {
  _call: TransportAccountCall;

  constructor(call: TransportAccountCall) {
    this._call = call;
  }
}

export class TransportAccountCallTransportEmailProofStruct extends ethereum.Tuple {
  get domain(): string {
    return this[0].toString();
  }

  get timestamp(): BigInt {
    return this[1].toBigInt();
  }

  get nullifier(): Bytes {
    return this[2].toBytes();
  }

  get dkimPublicKeyHash(): Bytes {
    return this[3].toBytes();
  }

  get proof(): Bytes {
    return this[4].toBytes();
  }
}

export class UpdateDKIMRegistryOfWalletSaltCall extends ethereum.Call {
  get inputs(): UpdateDKIMRegistryOfWalletSaltCall__Inputs {
    return new UpdateDKIMRegistryOfWalletSaltCall__Inputs(this);
  }

  get outputs(): UpdateDKIMRegistryOfWalletSaltCall__Outputs {
    return new UpdateDKIMRegistryOfWalletSaltCall__Outputs(this);
  }
}

export class UpdateDKIMRegistryOfWalletSaltCall__Inputs {
  _call: UpdateDKIMRegistryOfWalletSaltCall;

  constructor(call: UpdateDKIMRegistryOfWalletSaltCall) {
    this._call = call;
  }

  get walletSalt(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get dkimRegistry(): Address {
    return this._call.inputValues[1].value.toAddress();
  }
}

export class UpdateDKIMRegistryOfWalletSaltCall__Outputs {
  _call: UpdateDKIMRegistryOfWalletSaltCall;

  constructor(call: UpdateDKIMRegistryOfWalletSaltCall) {
    this._call = call;
  }
}
