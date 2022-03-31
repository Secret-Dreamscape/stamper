# Stamping contract

The stamping contract is responsible for stamping a word onto a DreamScape NFT.


## Instantiating the contract
To instantiate the contract you need to pass the following parameters to it: 

| Parameter      | Type      | Description                                 |
|----------------|-----------|---------------------------------------------|
| phonebook_addr | HumanAddr | The address to an instance of the phonebook |
| phonebook_hash | String    | The code hash for the phonebook contract    |
| nft_addr       | HumanAddr | The address to the NFT contract             |
| nft_hash       | String    | The code hash for the NFT contract          |

## Handle Functions
| Function name   | Description                                                                        | Admin Only? |
|-----------------|------------------------------------------------------------------------------------|-------------|
| UpdateContracts | Modifies the address to the phonebook and NFT contracts as well as their code hash | Yes         |
| Stamp           | Stamps an NFT with the selected word (can only be called by a game contract)       | No          |

## Queries
This contract doesn't have any queriable data