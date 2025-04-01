// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {stdJson} from "forge-std/StdJson.sol";
import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {WavsNft} from "../src/WavsNft.sol";
import {Strings} from "@openzeppelin-contracts/utils/Strings.sol";

contract DeployNFTDemo is Script {
    using stdJson for string;

    string public root = vm.projectRoot();
    string public script_output_path =
        string.concat(root, "/.docker/script_deploy.json");

    function run() public {
        (uint256 privateKey, ) = Utils.getPrivateKey(vm);

        vm.startBroadcast(privateKey);

        // Get the deployed service manager
        address serviceManager = Utils.getServiceManager(vm);

        // Deploy the contract
        WavsNft nft = new WavsNft(serviceManager);

        vm.stopBroadcast();

        // Log the deployment
        console.log("Service manager:", serviceManager);
        console.log("NFTDemo deployed at:", address(nft));

        // Write to JSON file
        string memory _json = "json";
        _json.serialize("nft", Strings.toHexString(address(nft)));
        _json.serialize("service_manager", Strings.toHexString(serviceManager));
        string memory _finalJson = _json.serialize(
            "trigger_event",
            "NewTrigger(bytes)"
        );
        vm.writeFile(script_output_path, _finalJson);

        // Write to .env file
        Utils.saveEnvVars(
            vm,
            string.concat("\nNFT_ADDRESS=", vm.toString(address(nft)))
        );
        console.log("Updated .env file with NFT_ADDRESS");
    }
}
