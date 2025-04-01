// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {WavsNft} from "../src/contracts/WavsNft.sol";
import {WavsMinter} from "../src/contracts/WavsMinter.sol";
import {ITypes} from "../src/interfaces/ITypes.sol";
import {Common} from "./Common.s.sol";

contract ShowResults is Common {
    function run() public view {
        // WavsNft nft = WavsNft(vm.envAddress("NFT_ADDRESS"));
        // WavsMinter minter = WavsMinter(vm.envAddress("MINTER_ADDRESS"));
        // uint256 nextTokenId = nft.nextTokenId();
        // if (nextTokenId == 0) {
        //     console.log("No NFTs minted yet");
        //     return;
        // }
        // nextTokenId--;
        // console.log("Last token ID:", nextTokenId);
        // string memory dataURI = nft.tokenURI(nextTokenId);
        // console.log("dataURI:", dataURI);
        // // remove the base64 prefix
        // bytes memory dataURIBytes = bytes(dataURI);
        // bytes memory base64Prefix = bytes("data:application/json;base64,");
        // bytes memory data = new bytes(
        //     dataURIBytes.length - base64Prefix.length
        // );
        // for (uint256 i = 0; i < data.length; i++) {
        //     data[i] = dataURIBytes[base64Prefix.length + i];
        // }
        // console.log(
        //     string.concat(
        //         'Run `echo "',
        //         string(data),
        //         '" | base64 --decode` to view the data'
        //     )
        // );
        // // Show the last trigger info from the minter
        // ITypes.TriggerId lastTriggerId = ITypes.TriggerId.wrap(nextTokenId);
        // ITypes.TriggerInfo memory info = minter.getTrigger(lastTriggerId);
        // console.log("Last trigger created by:", info.creator);
        // console.log("Last trigger message:", string(info.data));
    }
}
