// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {WavsNft} from "../src/contracts/WavsNft.sol";
import {ITypes} from "../src/interfaces/ITypes.sol";
import {Common} from "./Common.s.sol";

contract TriggerWavsNft is Common {
    function run(string memory testMessage) public {
        address nftAddress = vm.envAddress("NFT_ADDRESS");

        WavsNft nft = WavsNft(nftAddress);

        vm.startBroadcast(_privateKey);

        // Create test trigger data using the provided message
        bytes memory testData = abi.encode(testMessage);

        // Add trigger (sends 0.1 ETH)
        nft.addTrigger{value: 0.1 ether}(testData);

        vm.stopBroadcast();

        // Get the trigger ID (it will be 0 for the first trigger)
        ITypes.TriggerId triggerId = ITypes.TriggerId.wrap(0);

        // Fetch and log the trigger info
        ITypes.TriggerInfo memory info = nft.getTrigger(triggerId);
        console.log("Trigger created by:", info.creator);
        console.log("Trigger message:", testMessage);
        console.logBytes(info.data);
    }
}
