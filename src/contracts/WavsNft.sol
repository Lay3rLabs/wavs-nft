// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Pausable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Votes.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import {IWavsServiceHandler} from "@wavs/interfaces/IWavsServiceHandler.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {IWavsNftServiceTypes} from "interfaces/IWavsNftServiceTypes.sol";

contract WavsNft is
    ERC721,
    ERC721Enumerable,
    ERC721URIStorage,
    ERC721Pausable,
    AccessControl,
    ERC721Burnable,
    EIP712,
    ERC721Votes,
    IWavsServiceHandler
{
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    IWavsServiceManager public serviceManager;
    IWavsNftServiceTypes.TriggerId public nextTriggerId;
    uint256 public nextTokenId;
    uint256 public updateFee = 0.01 ether;
    address public fundsRecipient;

    constructor(
        address serviceManager_,
        address fundsRecipient_
    ) ERC721("TriggerNFT", "TNFT") EIP712("TriggerNFT", "1") {
        require(serviceManager_ != address(0), "Invalid service manager");
        require(fundsRecipient_ != address(0), "Invalid funds recipient");

        // TODO consider what the permissions of this contract should be
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(PAUSER_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);

        serviceManager = IWavsServiceManager(serviceManager_);
        fundsRecipient = fundsRecipient_;
    }

    function pause() public onlyRole(PAUSER_ROLE) {
        _pause();
    }

    function unpause() public onlyRole(PAUSER_ROLE) {
        _unpause();
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
        // Validate the data and signature via the service manager
        serviceManager.validate(data, signature);

        // Decode the data to get the mint result
        IWavsNftServiceTypes.WavsResponse memory wavsResponse = abi.decode(
            data,
            (IWavsNftServiceTypes.WavsResponse)
        );

        // Handle the mint or update by checking the trigger type
        if (
            wavsResponse.wavsTriggerType ==
            IWavsNftServiceTypes.WavsTriggerType.MINT
        ) {
            // Decode the mint info
            IWavsNftServiceTypes.WavsMintResult memory mintResult = abi.decode(
                wavsResponse.data,
                (IWavsNftServiceTypes.WavsMintResult)
            );

            // Increment the tokenId
            uint256 tokenId = nextTokenId++;

            // Mint the NFT
            _safeMint(mintResult.recipient, tokenId);

            // Set the tokenURI
            _setTokenURI(tokenId, mintResult.tokenURI);

            // Emit event to notify the minter contract that the mint has been fulfilled
            emit IWavsNftServiceTypes.WavsNftMint(
                mintResult.recipient,
                tokenId,
                mintResult.tokenURI,
                IWavsNftServiceTypes.TriggerId.unwrap(mintResult.triggerId)
            );
        } else if (
            wavsResponse.wavsTriggerType ==
            IWavsNftServiceTypes.WavsTriggerType.UPDATE
        ) {
            // Decode the update info
            IWavsNftServiceTypes.WavsUpdateResult memory updateResult = abi
                .decode(
                    wavsResponse.data,
                    (IWavsNftServiceTypes.WavsUpdateResult)
                );

            // Update the tokenURI
            _setTokenURI(updateResult.tokenId, updateResult.tokenURI);

            // Emit event to notify that the NFT has been updated
            emit IWavsNftServiceTypes.WavsNftUpdate(
                ownerOf(updateResult.tokenId),
                updateResult.tokenId,
                updateResult.tokenURI,
                IWavsNftServiceTypes.TriggerId.unwrap(updateResult.triggerId)
            );
        }
    }

    // Add tokenURI override
    function tokenURI(
        uint256 tokenId
    ) public view override(ERC721, ERC721URIStorage) returns (string memory) {
        return super.tokenURI(tokenId);
    }

    function _update(
        address to,
        uint256 tokenId,
        address auth
    )
        internal
        override(ERC721, ERC721Enumerable, ERC721Pausable, ERC721Votes)
        returns (address)
    {
        return super._update(to, tokenId, auth);
    }

    function _increaseBalance(
        address account,
        uint128 value
    ) internal override(ERC721, ERC721Enumerable, ERC721Votes) {
        super._increaseBalance(account, value);
    }

    function supportsInterface(
        bytes4 interfaceId
    )
        public
        view
        override(ERC721, ERC721Enumerable, ERC721URIStorage, AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }

    function clock() public view override returns (uint48) {
        return uint48(block.timestamp);
    }

    function CLOCK_MODE() public pure override returns (string memory) {
        return "mode=timestamp";
    }

    function _setTokenURI(
        uint256 tokenId,
        string memory _tokenURI
    ) internal override(ERC721URIStorage) {
        super._setTokenURI(tokenId, _tokenURI);
    }

    /**
     * @notice Sets the fee required for updating an NFT
     * @param newFee The new update fee in wei
     */
    function setUpdateFee(
        uint256 newFee
    ) external onlyRole(DEFAULT_ADMIN_ROLE) {
        updateFee = newFee;
    }

    /**
     * @notice Sets the address that receives update fees
     * @param newRecipient The new funds recipient address
     */
    function setFundsRecipient(
        address newRecipient
    ) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(newRecipient != address(0), "Invalid funds recipient");
        fundsRecipient = newRecipient;
    }

    /**
     * @notice Triggers an update for an existing NFT
     * @param tokenId The ID of the NFT to update
     * @param prompt The text prompt for AI generation
     */
    function triggerUpdate(
        uint256 tokenId,
        string calldata prompt
    ) external payable {
        require(msg.value >= updateFee, "Insufficient update fee");
        require(ownerOf(tokenId) == msg.sender, "Not NFT owner");

        // Refund any excess payment
        uint256 excess = msg.value - updateFee;
        if (excess > 0) {
            (bool refundSuccess, ) = payable(msg.sender).call{value: excess}(
                ""
            );
            require(refundSuccess, "Failed to refund excess");
        }

        // Increment trigger ID
        nextTriggerId = IWavsNftServiceTypes.TriggerId.wrap(
            IWavsNftServiceTypes.TriggerId.unwrap(nextTriggerId) + 1
        );

        // Emit trigger event
        emit IWavsNftServiceTypes.WavsNftTrigger(
            msg.sender,
            prompt,
            IWavsNftServiceTypes.TriggerId.unwrap(nextTriggerId),
            uint8(IWavsNftServiceTypes.WavsTriggerType.UPDATE),
            tokenId
        );
    }
}
