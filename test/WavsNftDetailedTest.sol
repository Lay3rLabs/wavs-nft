// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {WavsNft} from "../src/contracts/WavsNft.sol";
import {IWavsNftServiceTypes} from "../src/interfaces/IWavsNftServiceTypes.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";

// Mock for the WAVS Service Manager
contract MockServiceManager is IWavsServiceManager {
    function validate(bytes calldata, bytes calldata) external pure {
        // In a real scenario, this would validate signatures
        // For testing, we'll just pass validation
    }
}

contract WavsNftDetailedTest is Test {
    WavsNft public nft;
    MockServiceManager public serviceManager;

    address public owner = address(1);
    address public minter = address(2);
    address public user1 = address(3);
    address public user2 = address(4);

    uint256 public updateFee = 0.01 ether;

    // Role constants from the WavsNft contract
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    function setUp() public {
        vm.startPrank(owner);

        // Deploy mock service manager
        serviceManager = new MockServiceManager();

        // Deploy NFT contract
        nft = new WavsNft(address(serviceManager), owner);

        // Grant minter role to the minter address
        nft.grantRole(MINTER_ROLE, minter);

        vm.stopPrank();
    }

    function testConstructor() public {
        // Check contract initialization
        assertEq(address(nft.serviceManager()), address(serviceManager));
        assertEq(nft.fundsRecipient(), owner);
        assertEq(nft.updateFee(), updateFee);
        assertEq(nft.nextTokenId(), 0);

        // Check roles
        assertTrue(nft.hasRole(DEFAULT_ADMIN_ROLE, owner));
        assertTrue(nft.hasRole(PAUSER_ROLE, owner));
        assertTrue(nft.hasRole(MINTER_ROLE, owner));
        assertTrue(nft.hasRole(MINTER_ROLE, minter));
    }

    function testPauseUnpause() public {
        // Check initial state
        assertFalse(nft.paused());

        // Pause contract as owner
        vm.prank(owner);
        nft.pause();
        assertTrue(nft.paused());

        // Unpause contract as owner
        vm.prank(owner);
        nft.unpause();
        assertFalse(nft.paused());
    }

    function test_RevertWhen_NonPauserPauses() public {
        // Should revert when called by non-pauser
        vm.expectRevert();
        vm.prank(user1);
        nft.pause();
    }

    function test_RevertWhen_NonPauserUnpauses() public {
        // First pause the contract
        vm.prank(owner);
        nft.pause();

        // Should revert when called by non-pauser
        vm.expectRevert();
        vm.prank(user1);
        nft.unpause();
    }

    function testHandleSignedDataForMint() public {
        // Create a mint result
        IWavsNftServiceTypes.TriggerId triggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(1);
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: triggerId,
                recipient: user1,
                tokenURI: "ipfs://mint-token-uri"
            });

        // Create a response
        IWavsNftServiceTypes.WavsResponse
            memory wavsResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: triggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        // Encode the response
        bytes memory data = abi.encode(wavsResponse);
        bytes memory signature = ""; // Mock signature

        // Handle the data
        vm.prank(address(0)); // Can be any address since we're using a mock
        nft.handleSignedData(data, signature);

        // Check that the NFT was minted
        assertEq(nft.ownerOf(0), user1);
        assertEq(nft.tokenURI(0), "ipfs://mint-token-uri");

        // Verify next token ID was incremented
        assertEq(nft.nextTokenId(), 1);
    }

    function testHandleSignedDataForUpdate() public {
        // First mint an NFT
        IWavsNftServiceTypes.TriggerId mintTriggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(1);
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: mintTriggerId,
                recipient: user1,
                tokenURI: "ipfs://mint-token-uri"
            });

        IWavsNftServiceTypes.WavsResponse
            memory mintResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: mintTriggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        bytes memory mintData = abi.encode(mintResponse);
        bytes memory signature = ""; // Mock signature

        vm.prank(address(0));
        nft.handleSignedData(mintData, signature);

        // Now update the NFT
        IWavsNftServiceTypes.TriggerId updateTriggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(2);
        IWavsNftServiceTypes.WavsUpdateResult
            memory updateResult = IWavsNftServiceTypes.WavsUpdateResult({
                triggerId: updateTriggerId,
                owner: user1,
                tokenURI: "ipfs://update-token-uri",
                tokenId: 0
            });

        IWavsNftServiceTypes.WavsResponse
            memory updateResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: updateTriggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.UPDATE,
                data: abi.encode(updateResult)
            });

        bytes memory updateData = abi.encode(updateResponse);

        vm.prank(address(0));
        nft.handleSignedData(updateData, signature);

        // Check that the NFT was updated
        assertEq(nft.tokenURI(0), "ipfs://update-token-uri");
    }

    function testTriggerUpdate() public {
        // First mint an NFT
        IWavsNftServiceTypes.TriggerId mintTriggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(1);
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: mintTriggerId,
                recipient: user1,
                tokenURI: "ipfs://mint-token-uri"
            });

        IWavsNftServiceTypes.WavsResponse
            memory mintResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: mintTriggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        bytes memory mintData = abi.encode(mintResponse);
        bytes memory signature = ""; // Mock signature

        vm.prank(address(0));
        nft.handleSignedData(mintData, signature);

        // Now user1 (NFT owner) triggers an update
        vm.deal(user1, 1 ether);
        vm.prank(user1);
        string memory updatePrompt = "Updated prompt for NFT";

        vm.expectEmit(true, false, true, true);
        emit IWavsNftServiceTypes.WavsNftTrigger(
            user1,
            updatePrompt,
            1, // This will be the TriggerId since we're starting from 0
            uint8(IWavsNftServiceTypes.WavsTriggerType.UPDATE),
            0 // tokenId
        );

        nft.triggerUpdate{value: updateFee}(0, updatePrompt);

        // Verify next trigger ID was incremented
        IWavsNftServiceTypes.TriggerId nextTriggerId = nft.nextTriggerId();
        assertEq(IWavsNftServiceTypes.TriggerId.unwrap(nextTriggerId), 1);
    }

    function testTriggerUpdateWithExcessPayment() public {
        // First mint an NFT
        IWavsNftServiceTypes.TriggerId mintTriggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(1);
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: mintTriggerId,
                recipient: user1,
                tokenURI: "ipfs://mint-token-uri"
            });

        IWavsNftServiceTypes.WavsResponse
            memory mintResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: mintTriggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        bytes memory mintData = abi.encode(mintResponse);
        bytes memory signature = ""; // Mock signature

        vm.prank(address(0));
        nft.handleSignedData(mintData, signature);

        // Now user1 triggers an update with excess payment
        vm.deal(user1, 1 ether);
        vm.startPrank(user1);

        uint256 initialBalance = user1.balance;
        string memory updatePrompt = "Updated prompt for NFT";
        nft.triggerUpdate{value: 0.02 ether}(0, updatePrompt);

        // Check that excess was refunded (0.01 ether)
        assertEq(user1.balance, initialBalance - updateFee);

        vm.stopPrank();
    }

    function test_RevertWhen_UpdatingWithInsufficientPayment() public {
        // First mint an NFT
        IWavsNftServiceTypes.TriggerId mintTriggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(1);
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: mintTriggerId,
                recipient: user1,
                tokenURI: "ipfs://mint-token-uri"
            });

        IWavsNftServiceTypes.WavsResponse
            memory mintResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: mintTriggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        bytes memory mintData = abi.encode(mintResponse);
        bytes memory signature = ""; // Mock signature

        vm.prank(address(0));
        nft.handleSignedData(mintData, signature);

        // Now user1 tries to trigger an update with insufficient payment
        vm.deal(user1, 1 ether);
        vm.prank(user1);
        string memory updatePrompt = "Updated prompt for NFT";

        vm.expectRevert("Insufficient update fee");
        nft.triggerUpdate{value: 0.005 ether}(0, updatePrompt); // Should revert
    }

    function test_RevertWhen_NonOwnerUpdatesNft() public {
        // First mint an NFT for user1
        IWavsNftServiceTypes.TriggerId mintTriggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(1);
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: mintTriggerId,
                recipient: user1,
                tokenURI: "ipfs://mint-token-uri"
            });

        IWavsNftServiceTypes.WavsResponse
            memory mintResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: mintTriggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        bytes memory mintData = abi.encode(mintResponse);
        bytes memory signature = ""; // Mock signature

        vm.prank(address(0));
        nft.handleSignedData(mintData, signature);

        // Now user2 (not the owner) tries to trigger an update
        vm.deal(user2, 1 ether);
        vm.prank(user2);
        string memory updatePrompt = "Updated prompt for NFT";

        vm.expectRevert("Not NFT owner");
        nft.triggerUpdate{value: updateFee}(0, updatePrompt); // Should revert
    }

    function testSetUpdateFee() public {
        uint256 newFee = 0.02 ether;

        // Only admin should be able to set update fee
        vm.prank(owner);
        nft.setUpdateFee(newFee);

        assertEq(nft.updateFee(), newFee);
    }

    function test_RevertWhen_NonAdminSetsUpdateFee() public {
        uint256 newFee = 0.02 ether;

        // Should revert when called by non-admin
        vm.expectRevert();
        vm.prank(user1);
        nft.setUpdateFee(newFee);
    }

    function testSetFundsRecipient() public {
        address newRecipient = address(5);

        // Only admin should be able to set funds recipient
        vm.prank(owner);
        nft.setFundsRecipient(newRecipient);

        assertEq(nft.fundsRecipient(), newRecipient);
    }

    function test_RevertWhen_NonAdminSetsFundsRecipient() public {
        address newRecipient = address(5);

        // Should revert when called by non-admin
        vm.expectRevert();
        vm.prank(user1);
        nft.setFundsRecipient(newRecipient);
    }

    function test_RevertWhen_SettingZeroAddressAsFundsRecipient() public {
        // Should revert with zero address
        vm.expectRevert("Invalid funds recipient");
        vm.prank(owner);
        nft.setFundsRecipient(address(0));
    }

    function testERC721Extensions() public {
        // Test some of the ERC721 extension functionality

        // First mint an NFT
        IWavsNftServiceTypes.TriggerId mintTriggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(1);
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: mintTriggerId,
                recipient: user1,
                tokenURI: "ipfs://mint-token-uri"
            });

        IWavsNftServiceTypes.WavsResponse
            memory mintResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: mintTriggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        bytes memory mintData = abi.encode(mintResponse);
        bytes memory signature = ""; // Mock signature

        vm.prank(address(0));
        nft.handleSignedData(mintData, signature);

        // Test ERC721Enumerable
        assertEq(nft.totalSupply(), 1);
        assertEq(nft.tokenOfOwnerByIndex(user1, 0), 0);
        assertEq(nft.tokenByIndex(0), 0);

        // Test ERC721URIStorage
        assertEq(nft.tokenURI(0), "ipfs://mint-token-uri");

        // Test ERC721Burnable (owner can burn their token)
        vm.prank(user1);
        nft.burn(0);

        // Verify token was burned
        assertEq(nft.totalSupply(), 0);
        vm.expectRevert(); // Token no longer exists
        nft.ownerOf(0);
    }

    function testERC721Pausable() public {
        // Pause the contract
        vm.prank(owner);
        nft.pause();

        // Test that operations are paused
        IWavsNftServiceTypes.TriggerId mintTriggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(1);
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: mintTriggerId,
                recipient: user1,
                tokenURI: "ipfs://mint-token-uri"
            });

        IWavsNftServiceTypes.WavsResponse
            memory mintResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: mintTriggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        bytes memory mintData = abi.encode(mintResponse);
        bytes memory signature = ""; // Mock signature

        // Should revert when contract is paused
        vm.expectRevert();
        vm.prank(address(0));
        nft.handleSignedData(mintData, signature);

        // Unpause the contract
        vm.prank(owner);
        nft.unpause();

        // Now it should work
        vm.prank(address(0));
        nft.handleSignedData(mintData, signature);

        // Verify the token was minted
        assertEq(nft.ownerOf(0), user1);
    }
}
