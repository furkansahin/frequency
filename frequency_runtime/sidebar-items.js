window.SIDEBAR_ITEMS = {"constant":[["AVERAGE_ON_INITIALIZE_RATIO","We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is used to limit the maximal weight of a single extrinsic."],["DAYS",""],["EXISTENTIAL_DEPOSIT","The existential deposit. Set to be 1/100th of a token."],["FREQUENCY_LOCAL_TOKEN",""],["FREQUENCY_ROCOCO_TOKEN",""],["FREQUENCY_TOKEN",""],["HOURS",""],["MAXIMUM_BLOCK_WEIGHT","We allow for 0.5 of a second of compute with a 12 second average block time."],["MILLISECS_PER_BLOCK","This determines the average expected block time that we are targeting. Blocks will be produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn slot_duration()`."],["MINUTES",""],["NORMAL_DISPATCH_RATIO","We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by `Operational` extrinsics."],["ORML_MAX_VESTING_SCHEDULES",""],["SLOT_DURATION",""],["TOKEN_DECIMALS",""],["TREASURY_PALLET_ID","Generates the pallet “account” 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z"],["VERSION",""],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""]],"enum":[["MultiAddress","A multi-format address wrapper for on-chain accounts."],["OriginCaller",""],["RuntimeCall",""],["RuntimeEvent",""]],"fn":[["native_version","The version information used to identify this runtime when compiled natively."]],"mod":[["api",""],["currency",""],["opaque","Opaque types. These are used by the CLI to instantiate machinery that don’t need to know the specifics of the runtime. They can then be made to be agnostic over specific formats of data like extrinsics, allowing for them to continue syncing the network through upgrades to even the core data structures."],["weights","Expose the auto generated weight files."]],"struct":[["BaseCallFilter","Basefilter to only allow specified transactions call to be executed For non mainnet [–features frequency] all transactions are allowed"],["BlockExecutionWeight","Importing a block with 0 Extrinsics."],["Burn","How much of the treasury to burn, if funds remain at the end of the SpendPeriod Set to zero until the economic system is setup and stabilized"],["CollatorKickThreshold",""],["CollatorMaxCandidates",""],["CollatorMaxInvulnerables",""],["CollatorMinCandidates",""],["CooloffPeriod",""],["CouncilMotionDuration",""],["EnactmentPeriod",""],["ExtrinsicBaseWeight","Executing a NO-OP `System::remarks` Extrinsic."],["FastTrackVotingPeriod",""],["GenesisConfig",""],["LaunchPeriod",""],["MaxApprovals","Maximum number of approved proposals per Spending Period Set to 64 or 16 per week"],["MaxDataSize","Clone + Debug + Eq  implementation for u32 types"],["MaxVestingSchedules","Need this declaration method for use + type safety in benchmarks"],["MaximumSchedulerWeight",""],["MessagesMaxPayloadSizeBytes",""],["MinVestedTransfer",""],["MinimumDeposit",""],["MinimumPeriod",""],["NeverDepositIntoId",""],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["Perbill","A fixed point representation of a number in the range [0, 1]."],["Permill","A fixed point representation of a number in the range [0, 1]."],["PreimageBaseDeposit",""],["PreimageByteDeposit",""],["ProposalBondMaximum","Minimum bond for a treasury proposal"],["ProposalBondMinimum","Minimum bond for a treasury proposal"],["ProposalBondPercent","Bond amount a treasury request must put up to make the proposal This will be transferred to OnSlash if the proposal is rejected"],["RootAsVestingPallet",""],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["RuntimeBlockLength",""],["RuntimeBlockWeights",""],["RuntimeOrigin","The runtime origin type representing the origin of a call."],["SchemasMaxRegistrations","The maximum number of schema registrations"],["SessionKeys",""],["SpendPeriod",""],["Ss58Prefix","SS58 Prefix for the for Frequency Network 90 is the prefix for the Frequency Network on Polkadot 42 is the prefix for the Frequency Network on Rococo"],["TCMotionDuration",""],["TransactionByteFee","Relay Chain `TransactionByteFee` / 10"],["TreasuryPalletId","Keyless account that holds the money for the treasury"],["Version",""],["VestingPalletId",""],["VotingPeriod",""],["WeightToFee","Handles converting a weight scalar to a fee value, based on the scale and granularity of the node’s balance type."]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["Aura",""],["AuraConfig",""],["AuraExt",""],["AuraExtConfig",""],["AuraId","An Aura authority identifier using S/R 25519 as its crypto."],["AuraMaxAuthorities","The maximum number of authorities"],["Authorship",""],["AuthorshipUncleGenerations",""],["Balances",""],["BalancesConfig",""],["BalancesMaxLocks",""],["BalancesMaxReserves",""],["Block","Block type as expected by this runtime."],["BlockId","BlockId type as expected by this runtime."],["CheckedExtrinsic","Extrinsic type that has already been checked."],["CollatorSelection",""],["CollatorSelectionConfig",""],["Council",""],["CouncilConfig",""],["CouncilMaxMembers",""],["CouncilMaxProposals",""],["Democracy",""],["DemocracyConfig",""],["DemocracyMaxProposals",""],["DemocracyMaxVotes",""],["Executive","Executive: handles dispatch to the various modules."],["FIFTY",""],["FrameSystemMaxConsumers",""],["HUNDRED",""],["MSAMaxSignaturesPerBucket","The maximum number of signatures per virtual bucket"],["MSAMaxSignaturesStored","The upper limit on total stored signatures. This MUST be MaxSignaturesPerBucket * NumberOfBuckets."],["MSAMortalityWindowSize","The number of blocks per virtual bucket"],["MSANumberOfBuckets","The total number of virtual buckets"],["Messages",""],["MessagesMaxPerBlock","The maximum number of messages per block"],["Msa",""],["MsaMaxProviderNameSize","The maximum size of the provider name (in bytes)"],["MsaMaxPublicKeysPerMsa","The maximum number of public keys per MSA"],["ParachainInfo",""],["ParachainInfoConfig",""],["ParachainSystem",""],["ParachainSystemConfig",""],["Preimage",""],["PreimageMaxSize","Preimage maximum size set to 4 MB Expected to be removed in Polkadot v0.9.31"],["Scheduler",""],["SchedulerMaxScheduledPerBlock",""],["SchemaId","Schema Id is the unique identifier for a Schema"],["Schemas",""],["SchemasConfig",""],["SchemasMaxBytesBoundedVecLimit","The maximum length of a schema model (in bytes)"],["SchemasMinModelSizeBytes","The minimum schema model size (in bytes)"],["Session",""],["SessionConfig",""],["SessionOffset",""],["SessionPeriod",""],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The SignedExtension to the basic transaction logic."],["System",""],["SystemConfig",""],["TCMaxMembers",""],["TCMaxProposals",""],["TechnicalCommittee",""],["TechnicalCommitteeConfig",""],["Timestamp",""],["TransactionPayment",""],["TransactionPaymentOperationalFeeMultiplier",""],["Treasury",""],["TreasuryConfig",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Utility",""],["Vesting",""],["VestingConfig",""],["ZERO",""]]};