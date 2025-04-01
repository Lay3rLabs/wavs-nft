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
import {ITypes} from "interfaces/ITypes.sol";

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
    ITypes.TriggerId public nextTriggerId;
    uint256 public nextTokenId;

    struct WavsMintResult {
        ITypes.TriggerId triggerId;
        address recipient;
        string tokenURI;
    }

    // // TODO support updates
    // struct WavsUpdateResult {
    //     ITypes.TriggerId triggerId;
    //     string tokenURI;
    //     uint256 tokenId;
    // }

    // Event emitted when an NFT is minted via the AVS
    event NFTMinted(
        address indexed to,
        uint256 indexed tokenId,
        string dataUri,
        uint64 triggerId
    );

    constructor(
        address serviceManager_
    ) ERC721("TriggerNFT", "TNFT") EIP712("TriggerNFT", "1") {
        require(serviceManager_ != address(0), "Invalid service manager");

        // TODO consider what the permissions of this contract should be
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(PAUSER_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);

        serviceManager = IWavsServiceManager(serviceManager_);
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
        WavsMintResult memory mintResult = abi.decode(data, (WavsMintResult));

        // Increment the tokenId
        uint256 tokenId = nextTokenId++;

        // Mint the NFT
        _safeMint(mintResult.recipient, tokenId);

        // Set the tokenURI
        _setTokenURI(tokenId, mintResult.tokenURI);

        // Emit event to notify the minter contract that the mint has been fulfilled
        emit NFTMinted(
            mintResult.recipient,
            tokenId,
            mintResult.tokenURI,
            ITypes.TriggerId.unwrap(mintResult.triggerId)
        );
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
}
