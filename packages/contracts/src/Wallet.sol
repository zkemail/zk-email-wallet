// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

/// @title EmailWallet
/// @notice Simple Wallet contract to be used as the EmailWallet for users
/// @notice This wallet can `execute` any function on any contract provided calle is `owner`
/// @notice The deployed is the `owner` by default (EmailWalletCore)
contract Wallet is OwnableUpgradeable, UUPSUpgradeable {
    fallback() external payable {}

    receive() external payable {}

    modifier ownerOrSelf() {
        require(msg.sender == owner() || msg.sender == address(this), "only owner or self");
        _;
    }

    function initialize() public initializer {
        __Ownable_init();
    }

    function execute(address target, uint256 value, bytes calldata data) external ownerOrSelf {
        (bool success, bytes memory result) = target.call{value: value}(data);

        if (!success) {
            assembly {
                revert(add(result, 32), mload(result))
            }
        }
    }

    function _authorizeUpgrade(address newImplementation) internal override ownerOrSelf {}
}
