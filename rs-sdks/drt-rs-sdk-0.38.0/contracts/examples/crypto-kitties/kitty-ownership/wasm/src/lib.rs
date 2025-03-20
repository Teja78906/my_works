// Code generated by the numbat-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           21
// Async Callback:                       1
// Total number of exported functions:  23

#![no_std]

numbat_wasm_node::wasm_endpoints! {
    kitty_ownership
    (
        setGeneScienceContractAddress
        setKittyAuctionContractAddress
        claim
        totalSupply
        balanceOf
        ownerOf
        approve
        transfer
        transfer_from
        tokensOfOwner
        allowAuctioning
        approveSiringAndReturnKitty
        createGenZeroKitty
        getKittyById
        isReadyToBreed
        isPregnant
        canBreedWith
        approveSiring
        breedWith
        giveBirth
        birthFee
        callBack
    )
}
