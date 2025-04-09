// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {WavsMinter} from "../src/contracts/WavsMinter.sol";
import {IWavsNftServiceTypes} from "../src/interfaces/IWavsNftServiceTypes.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";

// Mock for the WAVS Service Manager
contract MockServiceManager is IWavsServiceManager {
    function validate(bytes calldata, bytes calldata) external pure {
        // In a real scenario, this would validate signatures
        // For testing, we'll just pass validation
    }
}

contract WavsMinterTest is Test {
    WavsMinter public minter;
    MockServiceManager public serviceManager;

    address public owner = address(1);
    address public user1 = address(2);
    address public user2 = address(3);

    uint256 public mintPrice = 0.1 ether;

    function setUp() public {
        vm.startPrank(owner);

        // Deploy mock service manager
        serviceManager = new MockServiceManager();

        // Deploy minter contract
        minter = new WavsMinter(address(serviceManager));

        vm.stopPrank();
    }

    function testConstructor() public {
        assertEq(address(minter.serviceManager()), address(serviceManager));
        assertEq(minter.owner(), owner);
        assertEq(minter.mintPrice(), mintPrice);

        // Verify initial triggerId
        IWavsNftServiceTypes.TriggerId triggerId = minter.nextTriggerId();
        assertEq(IWavsNftServiceTypes.TriggerId.unwrap(triggerId), 0);
    }

    function testTriggerMint() public {
        vm.deal(user1, 1 ether);
        vm.startPrank(user1);

        string memory prompt = "Test prompt for NFT";
        IWavsNftServiceTypes.TriggerId triggerId = minter.triggerMint{
            value: mintPrice
        }(prompt);

        // Check that the receipt was created properly
        WavsMinter.Receipt memory receipt = minter.getTrigger(triggerId);
        assertEq(receipt.creator, user1);
        assertEq(receipt.prompt, prompt);
        assertEq(
            uint8(receipt.wavsTriggerType),
            uint8(IWavsNftServiceTypes.WavsTriggerType.MINT)
        );
        assertFalse(receipt.fulfilled);

        // Verify next triggerId was incremented
        IWavsNftServiceTypes.TriggerId nextId = minter.nextTriggerId();
        assertEq(IWavsNftServiceTypes.TriggerId.unwrap(nextId), 1);

        vm.stopPrank();
    }

    function testTriggerMintWithExcessPayment() public {
        vm.deal(user1, 1 ether);
        vm.startPrank(user1);

        uint256 initialBalance = user1.balance;
        string memory prompt = "Test prompt for NFT";
        minter.triggerMint{value: 0.15 ether}(prompt);

        // Check that excess was refunded (0.05 ether)
        assertEq(user1.balance, initialBalance - mintPrice);

        vm.stopPrank();
    }

    function testFailTriggerMintInsufficientPayment() public {
        vm.deal(user1, 1 ether);
        vm.startPrank(user1);

        string memory prompt = "Test prompt for NFT";
        minter.triggerMint{value: 0.05 ether}(prompt); // Should fail

        vm.stopPrank();
    }

    function testMultipleMints() public {
        vm.deal(user1, 1 ether);
        vm.startPrank(user1);

        // First mint
        string memory prompt1 = "First NFT prompt";
        IWavsNftServiceTypes.TriggerId triggerId1 = minter.triggerMint{
            value: mintPrice
        }(prompt1);

        // Second mint
        string memory prompt2 = "Second NFT prompt";
        IWavsNftServiceTypes.TriggerId triggerId2 = minter.triggerMint{
            value: mintPrice
        }(prompt2);

        // Verify both receipts
        WavsMinter.Receipt memory receipt1 = minter.getTrigger(triggerId1);
        WavsMinter.Receipt memory receipt2 = minter.getTrigger(triggerId2);

        assertEq(receipt1.prompt, prompt1);
        assertEq(receipt2.prompt, prompt2);

        // Check that the triggerIds are different
        assertFalse(
            IWavsNftServiceTypes.TriggerId.unwrap(triggerId1) ==
                IWavsNftServiceTypes.TriggerId.unwrap(triggerId2)
        );

        // Verify next triggerId
        IWavsNftServiceTypes.TriggerId nextId = minter.nextTriggerId();
        assertEq(IWavsNftServiceTypes.TriggerId.unwrap(nextId), 2);

        vm.stopPrank();
    }

    function testHandleSignedData() public {
        // First trigger a mint
        vm.deal(user1, 1 ether);
        vm.prank(user1);
        string memory prompt = "Test prompt for NFT";
        IWavsNftServiceTypes.TriggerId triggerId = minter.triggerMint{
            value: mintPrice
        }(prompt);

        // Create and encode the response with the triggerId
        bytes memory data = abi.encode(triggerId);
        bytes memory signature = ""; // Mock signature

        // First emit the expected event to record its pattern
        vm.expectEmit(true, false, false, false, address(minter));
        emit IWavsNftServiceTypes.MintFulfilled(triggerId);

        // Call handleSignedData
        vm.prank(address(0)); // Can be any address since we're using a mock
        minter.handleSignedData(data, signature);

        // Check that the receipt was marked as fulfilled
        WavsMinter.Receipt memory receipt = minter.getTrigger(triggerId);
        assertTrue(receipt.fulfilled);
    }

    function testFailHandleSignedDataNonExistentTrigger() public {
        // Create a triggerId that doesn't exist
        IWavsNftServiceTypes.TriggerId nonExistentTriggerId = IWavsNftServiceTypes
                .TriggerId
                .wrap(999);

        // Encode the response
        bytes memory data = abi.encode(nonExistentTriggerId);
        bytes memory signature = ""; // Mock signature

        // This should fail because the trigger doesn't exist
        vm.prank(address(0));
        minter.handleSignedData(data, signature);
    }

    function testFailHandleSignedDataAlreadyFulfilled() public {
        // First trigger a mint
        vm.deal(user1, 1 ether);
        vm.prank(user1);
        string memory prompt = "Test prompt for NFT";
        IWavsNftServiceTypes.TriggerId triggerId = minter.triggerMint{
            value: mintPrice
        }(prompt);

        // Mark it as fulfilled
        bytes memory data = abi.encode(triggerId);
        bytes memory signature = ""; // Mock signature
        vm.prank(address(0));
        minter.handleSignedData(data, signature);

        // Try to fulfill it again - this should fail
        vm.prank(address(0));
        minter.handleSignedData(data, signature);
    }

    function testSetMintPrice() public {
        uint256 newPrice = 0.2 ether;

        // First emit the expected event to record its pattern
        vm.expectEmit(false, false, false, true, address(minter));
        emit WavsMinter.MintPriceUpdated(newPrice);

        // Only owner should be able to set mint price
        vm.prank(owner);
        minter.setMintPrice(newPrice);

        assertEq(minter.mintPrice(), newPrice);
    }

    function testFailSetMintPriceNonOwner() public {
        uint256 newPrice = 0.2 ether;

        // Should fail when called by non-owner
        vm.prank(user1);
        minter.setMintPrice(newPrice);
    }

    function testWithdrawFees() public {
        // Send some ETH to the minter contract
        vm.deal(address(minter), 1 ether);

        uint256 initialOwnerBalance = owner.balance;

        // First emit the expected event to record its pattern
        vm.expectEmit(true, false, false, true, address(minter));
        emit WavsMinter.FeesWithdrawn(owner, 1 ether);

        // Withdraw fees
        vm.prank(owner);
        minter.withdrawFees();

        // Check that the owner received the fees
        assertEq(owner.balance, initialOwnerBalance + 1 ether);
        assertEq(address(minter).balance, 0);
    }

    function testFailWithdrawFeesNonOwner() public {
        // Send some ETH to the minter contract
        vm.deal(address(minter), 1 ether);

        // Should fail when called by non-owner
        vm.prank(user1);
        minter.withdrawFees();
    }

    function testFailWithdrawFeesNoBalance() public {
        // No ETH in the contract
        vm.prank(owner);
        minter.withdrawFees(); // Should fail with "No balance to withdraw"
    }
}
