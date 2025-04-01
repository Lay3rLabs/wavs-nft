// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {WavsNft} from "../src/contracts/WavsNft.sol";
import {Common} from "./Common.s.sol";

contract ShowLastResultWavsNft is Common {
    function run() public view {
        WavsNft nft = WavsNft(vm.envAddress("NFT_ADDRESS"));

        uint256 nextTokenId = nft.nextTokenId();
        if (nextTokenId == 0) {
            console.log("No triggers created yet");
            return;
        }

        nextTokenId--;

        console.log("Last token ID:", nextTokenId);

        string memory dataURI = nft.tokenURI(nextTokenId);
        console.log("dataURI:", dataURI);

        // remove the base64 prefix
        bytes memory dataURIBytes = bytes(dataURI);
        bytes memory base64Prefix = bytes("data:application/json;base64,");
        bytes memory data = new bytes(
            dataURIBytes.length - base64Prefix.length
        );
        for (uint256 i = 0; i < data.length; i++) {
            data[i] = dataURIBytes[base64Prefix.length + i];
        }

        console.log(
            string.concat(
                'Run `echo "',
                string(data),
                '" | base64 --decode` to view the data'
            )
        );
    }
}
