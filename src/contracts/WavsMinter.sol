// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {IWavsServiceHandler} from "@wavs/interfaces/IWavsServiceHandler.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";

/**
 * @title Minter
 * @notice Contract for minting AI-generated NFTs through the WAVS system
 */
contract WavsMinter is Ownable, ReentrancyGuard, IWavsServiceHandler {
    // Config parameters
    uint256 public mintPrice = 0.1 ether;

    // Mapping to store additional metadata for each trigger
    mapping(ITypes.TriggerId => TriggerMetadata) public triggerMetadata;

    // Interface to the WAVS service manager
    IWavsServiceManager public serviceManager;

    // Auto-incrementing trigger ID counter
    ITypes.TriggerId public nextTriggerId;

    // Structure to hold metadata about the trigger
    struct TriggerMetadata {
        address creator;
        string prompt;
        ITypes.TriggerType triggerType;
        bool fulfilled;
    }

    // Event emitted when a mint/update is triggered
    event AvsMintTrigger(
        address indexed sender,
        string prompt,
        uint64 indexed triggerId,
        uint8 triggerType
    );

    // Event emitted when a mint is fulfilled
    event MintFulfilled(ITypes.TriggerId indexed triggerId, uint256 tokenId);

    // Event emitted when mint price is updated
    event MintPriceUpdated(uint256 newPrice);

    // Event emitted when fees are withdrawn
    event FeesWithdrawn(address indexed owner, uint256 amount);

    constructor(address _serviceManager) Ownable(msg.sender) {
        require(
            _serviceManager != address(0),
            "Invalid service manager address"
        );
        serviceManager = IWavsServiceManager(_serviceManager);
    }

    /**
     * @notice Trigger an AVS-generated NFT mint
     * @param prompt The text prompt for AI generation
     */
    function triggerMint(
        string calldata prompt
    ) external payable nonReentrant returns (ITypes.TriggerId) {
        require(msg.value >= mintPrice, "Insufficient payment");

        // Get the next trigger ID and increment the counter
        ITypes.TriggerId triggerId = nextTriggerId;
        nextTriggerId = ITypes.TriggerId.wrap(
            ITypes.TriggerId.unwrap(nextTriggerId) + 1
        );

        // Store metadata for this mint request
        triggerMetadata[triggerId] = TriggerMetadata({
            creator: msg.sender,
            prompt: prompt,
            triggerType: ITypes.TriggerType.MINT,
            fulfilled: false
        });

        // Refund any excess payment
        uint256 excess = msg.value - mintPrice;
        if (excess > 0) {
            (bool refundSuccess, ) = payable(msg.sender).call{value: excess}(
                ""
            );
            require(refundSuccess, "Failed to refund excess");
        }

        // Emit the AvsMintTrigger event
        emit AvsMintTrigger(
            msg.sender,
            prompt,
            ITypes.TriggerId.unwrap(triggerId),
            uint8(ITypes.TriggerType.MINT)
        );

        return triggerId;
    }

    /**
     * @notice Handle the signed AVS output data.
     * @param data The data to handle.
     * @param signature The signature of the data.
     */
    function handleSignedData(
        bytes calldata data,
        bytes calldata signature
    ) external override {
        serviceManager.validate(data, signature);

        ITypes.DataWithId memory dataWithId = abi.decode(
            data,
            (ITypes.DataWithId)
        );

        // Get the trigger metadata
        TriggerMetadata storage metadata = triggerMetadata[
            dataWithId.triggerId
        ];
        require(!metadata.fulfilled, "Already fulfilled");

        // Decode the result data
        string memory dataUri = abi.decode(dataWithId.data, (string));
        require(bytes(dataUri).length > 0, "URI is empty");

        // Mark the trigger as fulfilled
        metadata.fulfilled = true;

        // Emit the fulfillment event
        emit MintFulfilled(dataWithId.triggerId, 0); // TODO: Add tokenId when we have NFT contract integration
    }

    /**
     * @notice Update the mint price (owner only)
     * @param newPrice The new mint price in wei
     */
    function setMintPrice(uint256 newPrice) external onlyOwner {
        mintPrice = newPrice;
        emit MintPriceUpdated(newPrice);
    }

    /**
     * @notice Update the service manager address (owner only)
     * @param newAddress The new service manager address
     */
    function setServiceManager(address newAddress) external onlyOwner {
        require(newAddress != address(0), "Invalid address");
        serviceManager = IWavsServiceManager(newAddress);
    }

    /**
     * @notice Withdraw collected fees (owner only)
     */
    function withdrawFees() external onlyOwner nonReentrant {
        uint256 balance = address(this).balance;
        require(balance > 0, "No balance to withdraw");

        (bool success, ) = payable(msg.sender).call{value: balance}("");
        require(success, "Transfer failed");

        emit FeesWithdrawn(msg.sender, balance);
    }

    /**
     * @notice Get metadata for a trigger
     * @param triggerId The trigger ID to query
     * @return The trigger metadata struct
     */
    function getTrigger(
        ITypes.TriggerId triggerId
    ) external view returns (TriggerMetadata memory) {
        return triggerMetadata[triggerId];
    }
}
