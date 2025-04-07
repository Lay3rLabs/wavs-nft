// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {WavsNft} from "../src/contracts/WavsNft.sol";
import {Common} from "./Common.s.sol";

/// @dev Script to trigger an NFT update
contract Update is Common {
    /**
     * @dev Triggers an update for an existing NFT
     * @param nftAddr The address of the WavsNft contract
     * @param tokenId The ID of the NFT to update
     * @param prompt The text prompt for AI generation
     */
    function run(
        address nftAddr,
        uint256 tokenId,
        string calldata prompt
    ) public {
        vm.startBroadcast(_privateKey);

        // Get the update fee from the contract
        uint256 updateFee = WavsNft(nftAddr).updateFee();

        // Trigger the update with the exact fee
        WavsNft(nftAddr).triggerUpdate{value: updateFee}(tokenId, prompt);

        vm.stopBroadcast();

        // Log the update details
        console.log("NFT Address:", nftAddr);
        console.log("Token ID:", tokenId);
        console.log("Prompt:", prompt);
        console.log("Update Fee:", updateFee);
    }
}
