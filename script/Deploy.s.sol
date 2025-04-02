// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {stdJson} from "forge-std/StdJson.sol";
import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {WavsNft} from "../src/contracts/WavsNft.sol";
import {WavsMinter} from "../src/contracts/WavsMinter.sol";
import {Strings} from "@openzeppelin-contracts/utils/Strings.sol";
import {Common, EigenContracts} from "./Common.s.sol";

/// @dev Deployment script for WavsNft and WavsNftMinter contracts
contract Deploy is Common {
    using stdJson for string;

    string public root = vm.projectRoot();
    string public deployments_path =
        string.concat(root, "/.docker/deployments.json");
    string public script_output_path =
        string.concat(root, "/.docker/script_deploy.json");

    /**
     * @dev Deploys the WavsNft and WavsNftMinter contracts and writes the results to a JSON file
     * @param _serviceManagerAddr The address of the service manager
     */
    function run(string calldata _serviceManagerAddr) public {
        vm.startBroadcast(_privateKey);

        // Deploy the NFT contract
        WavsNft nft = new WavsNft(vm.parseAddress(_serviceManagerAddr));

        // Deploy the minter contract
        WavsMinter minter = new WavsMinter(
            vm.parseAddress(_serviceManagerAddr)
        );

        vm.stopBroadcast();

        // Log the deployments
        console.log("Service manager:", _serviceManagerAddr);
        console.log("WavsNft deployed at:", address(nft));
        console.log("WavsMinter deployed at:", address(minter));

        // Write to JSON file
        string memory _json = "json";
        _json.serialize("nft", Strings.toHexString(address(nft)));
        _json.serialize("minter", Strings.toHexString(address(minter)));
        _json.serialize("service_handler", Strings.toHexString(address(nft)));
        string memory _finalJson = _json.serialize(
            "service_manager",
            _serviceManagerAddr
        );
        vm.writeFile(script_output_path, _finalJson);
    }

    /**
     * @dev Loads the Eigen contracts from the deployments.json file
     * @return _fixture The Eigen contracts
     */
    function loadEigenContractsFromFS()
        public
        view
        returns (EigenContracts memory _fixture)
    {
        address _dm = _jsonBytesToAddress(
            ".eigen_core.local.delegation_manager"
        );
        address _rc = _jsonBytesToAddress(
            ".eigen_core.local.rewards_coordinator"
        );
        address _avs = _jsonBytesToAddress(".eigen_core.local.avs_directory");

        _fixture = EigenContracts({
            delegation_manager: _dm,
            rewards_coordinator: _rc,
            avs_directory: _avs
        });
    }

    /**
     * @dev Loads the service managers from the deployments.json file
     * @return _service_managers The list of service managers
     */
    function loadServiceManagersFromFS()
        public
        view
        returns (address[] memory _service_managers)
    {
        _service_managers = vm.readFile(deployments_path).readAddressArray(
            ".eigen_service_managers.local"
        );
    }

    // --- Internal Utils ---

    /**
     * @dev Converts a string to an address
     * @param _byteString The string to convert
     * @return _address The address
     */
    function _jsonBytesToAddress(
        string memory _byteString
    ) internal view returns (address _address) {
        _address = address(
            uint160(
                bytes20(vm.readFile(deployments_path).readBytes(_byteString))
            )
        );
    }
}
