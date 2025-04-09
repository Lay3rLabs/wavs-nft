// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {WavsNft} from "../src/contracts/WavsNft.sol";
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

contract WavsNftTest is Test {
    WavsNft public nft;
    WavsMinter public minter;
    MockServiceManager public serviceManager;

    address public owner = address(1);
    address public user1 = address(2);
    address public user2 = address(3);

    uint256 public mintPrice = 0.1 ether;
    uint256 public updateFee = 0.01 ether;

    function setUp() public {
        vm.startPrank(owner);

        // Deploy mock service manager
        serviceManager = new MockServiceManager();

        // Deploy NFT contract
        nft = new WavsNft(address(serviceManager), owner);

        // Deploy minter contract
        minter = new WavsMinter(address(serviceManager));

        // Grant minter role to minter contract
        nft.grantRole(nft.MINTER_ROLE(), address(minter));

        vm.stopPrank();
    }

    function testMintPrice() public {
        assertEq(minter.mintPrice(), mintPrice);
    }

    function testUpdateFee() public {
        assertEq(nft.updateFee(), updateFee);
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

    function testHandleSignedDataForMint() public {
        // First trigger a mint
        vm.deal(user1, 1 ether);
        vm.prank(user1);
        string memory prompt = "Test prompt for NFT";
        IWavsNftServiceTypes.TriggerId triggerId = minter.triggerMint{
            value: mintPrice
        }(prompt);

        // Create a WavsMintResult
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: triggerId,
                recipient: user1,
                tokenURI: "ipfs://some-token-uri"
            });

        // Create a WavsResponse
        IWavsNftServiceTypes.WavsResponse
            memory wavsResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: triggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        // Encode the response
        bytes memory data = abi.encode(wavsResponse);
        bytes memory signature = ""; // Mock signature

        // Call handleSignedData on the NFT contract as if it were from the AVS
        vm.prank(address(0)); // Can be any address since we're using a mock
        nft.handleSignedData(data, signature);

        // Check that the NFT was minted
        assertEq(nft.ownerOf(0), user1);
        assertEq(nft.tokenURI(0), "ipfs://some-token-uri");

        // Now call handleSignedData on the minter contract to complete the process
        data = abi.encode(triggerId);
        vm.prank(address(0));
        minter.handleSignedData(data, signature);

        // Check that the receipt was marked as fulfilled
        WavsMinter.Receipt memory receipt = minter.getTrigger(triggerId);
        assertTrue(receipt.fulfilled);
    }

    function testTriggerUpdate() public {
        // First mint an NFT
        vm.deal(user1, 1 ether);
        vm.prank(user1);
        string memory mintPrompt = "Test prompt for NFT";
        IWavsNftServiceTypes.TriggerId triggerId = minter.triggerMint{
            value: mintPrice
        }(mintPrompt);

        // Simulate the mint being completed
        IWavsNftServiceTypes.WavsMintResult
            memory mintResult = IWavsNftServiceTypes.WavsMintResult({
                triggerId: triggerId,
                recipient: user1,
                tokenURI: "ipfs://original-token-uri"
            });

        IWavsNftServiceTypes.WavsResponse
            memory wavsResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: triggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.MINT,
                data: abi.encode(mintResult)
            });

        bytes memory data = abi.encode(wavsResponse);
        bytes memory signature = ""; // Mock signature

        vm.prank(address(0));
        nft.handleSignedData(data, signature);

        // Now trigger an update
        // Give user1 more ETH for the update fee
        vm.deal(user1, updateFee);

        // Single prank for update
        vm.prank(user1);
        string memory updatePrompt = "Updated prompt for NFT";
        nft.triggerUpdate{value: updateFee}(0, updatePrompt);

        // Simulate the update being completed
        IWavsNftServiceTypes.TriggerId updateTriggerId = IWavsNftServiceTypes
            .TriggerId
            .wrap(1);

        IWavsNftServiceTypes.WavsUpdateResult
            memory updateResult = IWavsNftServiceTypes.WavsUpdateResult({
                triggerId: updateTriggerId,
                owner: user1,
                tokenURI: "ipfs://updated-token-uri",
                tokenId: 0
            });

        IWavsNftServiceTypes.WavsResponse
            memory updateResponse = IWavsNftServiceTypes.WavsResponse({
                triggerId: updateTriggerId,
                wavsTriggerType: IWavsNftServiceTypes.WavsTriggerType.UPDATE,
                data: abi.encode(updateResult)
            });

        data = abi.encode(updateResponse);

        vm.prank(address(0));
        nft.handleSignedData(data, signature);

        // Check that the NFT was updated
        assertEq(nft.tokenURI(0), "ipfs://updated-token-uri");
    }

    function testAdminFunctions() public {
        // Test setting mint price
        vm.prank(owner);
        minter.setMintPrice(0.2 ether);
        assertEq(minter.mintPrice(), 0.2 ether);

        // Test setting update fee
        vm.prank(owner);
        nft.setUpdateFee(0.02 ether);
        assertEq(nft.updateFee(), 0.02 ether);

        // Test setting funds recipient
        address newRecipient = address(4);
        vm.prank(owner);
        nft.setFundsRecipient(newRecipient);
        assertEq(nft.fundsRecipient(), newRecipient);
    }

    function testWithdrawFees() public {
        // Send some ETH to the minter contract
        vm.deal(address(minter), 1 ether);

        uint256 initialOwnerBalance = owner.balance;

        // Withdraw fees
        vm.prank(owner);
        minter.withdrawFees();

        // Check that the owner received the fees
        assertEq(owner.balance, initialOwnerBalance + 1 ether);
        assertEq(address(minter).balance, 0);
    }
}
