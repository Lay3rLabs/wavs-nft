// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

/**
 * @title IWavsNftServiceTypes
 * @notice Interface containing all types needed for the WAVS NFT service
 */
interface IWavsNftServiceTypes {
    /**
     * @notice TriggerId is a unique identifier for a trigger
     */
    type TriggerId is uint64;

    /**
     * @notice Enum for trigger operation types
     */
    enum TriggerType {
        MINT,
        UPDATE,
        FULFILLMENT
    }

    /**
     * @notice Struct to store the WAVS response data
     * @param triggerId The trigger ID
     * @param triggerType The type of trigger
     * @param data Contains WavsMintResult or WavsUpdateResult
     */
    struct WavsResponse {
        TriggerId triggerId;
        TriggerType triggerType;
        bytes data;
    }

    /**
     * @notice Event emitted when an NFT is minted via the AVS
     * @param to The recipient of the NFT
     * @param tokenId The ID of the minted NFT
     * @param dataUri The URI of the NFT data
     * @param triggerId The ID of the trigger that initiated the mint
     */
    event NftMintedViaWavs(
        address indexed to,
        uint256 indexed tokenId,
        string dataUri,
        uint64 triggerId
    );

    /**
     * @notice Event emitted when a mint/update is triggered
     * @param sender The address that triggered the mint
     * @param prompt The text prompt for AI generation
     * @param triggerId The ID of the trigger
     * @param triggerType The type of trigger
     */
    event AvsMintTrigger(
        address indexed sender,
        string prompt,
        uint64 indexed triggerId,
        uint8 triggerType
    );

    /**
     * @notice Event emitted when a mint is fulfilled
     * @param triggerId The ID of the fulfilled trigger
     */
    event MintFulfilled(TriggerId indexed triggerId);

    /**
     * @notice Struct to store the result of a mint operation
     * @param triggerId The trigger ID
     * @param recipient The address that will receive the NFT
     * @param tokenURI The URI of the NFT data
     */
    struct WavsMintResult {
        TriggerId triggerId;
        address recipient;
        string tokenURI;
    }

    /**
     * @notice Struct to store the result of an update operation
     * @param triggerId The trigger ID
     * @param tokenURI The new URI of the NFT data
     * @param tokenId The ID of the NFT to update
     */
    struct WavsUpdateResult {
        TriggerId triggerId;
        string tokenURI;
        uint256 tokenId;
    }
}
