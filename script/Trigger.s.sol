// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {WavsMinter} from "../src/contracts/WavsMinter.sol";
import {ITypes} from "../src/interfaces/ITypes.sol";
import {Common} from "./Common.s.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Strings} from "@openzeppelin-contracts/utils/Strings.sol";

contract Trigger is Common {
    using stdJson for string;

    string public root = vm.projectRoot();
    string public script_output_path =
        string.concat(root, "/.docker/script_deploy.json");

    /**
     * @dev Triggers the NFT minter contract with a prompt
     * @param _prompt The prompt to use for generating the NFT
     */
    function run(string memory _prompt) public {
        // Read minter address from deployment JSON
        string memory json = vm.readFile(script_output_path);
        address minterAddr = vm.parseAddress(json.readString(".minter"));

        console.log("Using minter address:", Strings.toHexString(minterAddr));
        WavsMinter minter = WavsMinter(minterAddr);

        vm.startBroadcast(_privateKey);

        // Trigger the minter contract
        minter.triggerMint{value: 0.1 ether}(_prompt);

        vm.stopBroadcast();

        // Get the trigger ID (it will be 0 for the first trigger)
        ITypes.TriggerId triggerId = ITypes.TriggerId.wrap(0);

        // Fetch and log the trigger metadata
        WavsMinter.TriggerMetadata memory metadata = minter.getTrigger(
            triggerId
        );

        console.log("Trigger created by:", metadata.creator);
        console.log("Trigger message:", metadata.prompt);
        console.log("Trigger type:", uint8(metadata.triggerType));
        console.log("Fulfilled:", metadata.fulfilled ? "Yes" : "No");
    }
}
