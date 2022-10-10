(function() {var implementors = {};
implementors["common_helpers"] = [{"text":"impl Freeze for <a class=\"enum\" href=\"common_helpers/avro/enum.AvroError.html\" title=\"enum common_helpers::avro::AvroError\">AvroError</a>","synthetic":true,"types":["common_helpers::avro::AvroError"]}];
implementors["common_primitives"] = [{"text":"impl&lt;T, S&gt; Freeze for <a class=\"struct\" href=\"common_primitives/ds/struct.OrderedSetExt.html\" title=\"struct common_primitives::ds::OrderedSetExt\">OrderedSetExt</a>&lt;T, S&gt;","synthetic":true,"types":["common_primitives::ds::OrderedSetExt"]},{"text":"impl&lt;BlockNumber&gt; Freeze for <a class=\"struct\" href=\"common_primitives/messages/struct.MessageResponse.html\" title=\"struct common_primitives::messages::MessageResponse\">MessageResponse</a>&lt;BlockNumber&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Freeze,&nbsp;</span>","synthetic":true,"types":["common_primitives::messages::MessageResponse"]},{"text":"impl&lt;BlockNumber&gt; Freeze for <a class=\"struct\" href=\"common_primitives/messages/struct.BlockPaginationRequest.html\" title=\"struct common_primitives::messages::BlockPaginationRequest\">BlockPaginationRequest</a>&lt;BlockNumber&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Freeze,&nbsp;</span>","synthetic":true,"types":["common_primitives::messages::BlockPaginationRequest"]},{"text":"impl&lt;BlockNumber, T&gt; Freeze for <a class=\"struct\" href=\"common_primitives/messages/struct.BlockPaginationResponse.html\" title=\"struct common_primitives::messages::BlockPaginationResponse\">BlockPaginationResponse</a>&lt;BlockNumber, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Freeze,&nbsp;</span>","synthetic":true,"types":["common_primitives::messages::BlockPaginationResponse"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_primitives/msa/struct.Delegator.html\" title=\"struct common_primitives::msa::Delegator\">Delegator</a>","synthetic":true,"types":["common_primitives::msa::Delegator"]},{"text":"impl&lt;BlockNumber, MaxSchemaGrants&gt; Freeze for <a class=\"struct\" href=\"common_primitives/msa/struct.ProviderInfo.html\" title=\"struct common_primitives::msa::ProviderInfo\">ProviderInfo</a>&lt;BlockNumber, MaxSchemaGrants&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Freeze,&nbsp;</span>","synthetic":true,"types":["common_primitives::msa::ProviderInfo"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_primitives/msa/struct.Provider.html\" title=\"struct common_primitives::msa::Provider\">Provider</a>","synthetic":true,"types":["common_primitives::msa::Provider"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"common_primitives/msa/struct.ProviderMetadata.html\" title=\"struct common_primitives::msa::ProviderMetadata\">ProviderMetadata</a>&lt;T&gt;","synthetic":true,"types":["common_primitives::msa::ProviderMetadata"]},{"text":"impl&lt;AccountId&gt; Freeze for <a class=\"struct\" href=\"common_primitives/msa/struct.KeyInfoResponse.html\" title=\"struct common_primitives::msa::KeyInfoResponse\">KeyInfoResponse</a>&lt;AccountId&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Freeze,&nbsp;</span>","synthetic":true,"types":["common_primitives::msa::KeyInfoResponse"]},{"text":"impl Freeze for <a class=\"enum\" href=\"common_primitives/parquet/base/enum.ParquetBaseType.html\" title=\"enum common_primitives::parquet::base::ParquetBaseType\">ParquetBaseType</a>","synthetic":true,"types":["common_primitives::parquet::base::ParquetBaseType"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_primitives/parquet/column/struct.ParquetColumn.html\" title=\"struct common_primitives::parquet::column::ParquetColumn\">ParquetColumn</a>","synthetic":true,"types":["common_primitives::parquet::column::ParquetColumn"]},{"text":"impl Freeze for <a class=\"enum\" href=\"common_primitives/parquet/column_compression_codec/enum.ColumnCompressionCodec.html\" title=\"enum common_primitives::parquet::column_compression_codec::ColumnCompressionCodec\">ColumnCompressionCodec</a>","synthetic":true,"types":["common_primitives::parquet::column_compression_codec::ColumnCompressionCodec"]},{"text":"impl Freeze for <a class=\"enum\" href=\"common_primitives/parquet/numeric/enum.ParquetNumericType.html\" title=\"enum common_primitives::parquet::numeric::ParquetNumericType\">ParquetNumericType</a>","synthetic":true,"types":["common_primitives::parquet::numeric::ParquetNumericType"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_primitives/parquet/numeric/struct.ParquetInteger.html\" title=\"struct common_primitives::parquet::numeric::ParquetInteger\">ParquetInteger</a>","synthetic":true,"types":["common_primitives::parquet::numeric::ParquetInteger"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_primitives/parquet/numeric/struct.ParquetDecimal.html\" title=\"struct common_primitives::parquet::numeric::ParquetDecimal\">ParquetDecimal</a>","synthetic":true,"types":["common_primitives::parquet::numeric::ParquetDecimal"]},{"text":"impl Freeze for <a class=\"enum\" href=\"common_primitives/parquet/string/enum.ParquetStringType.html\" title=\"enum common_primitives::parquet::string::ParquetStringType\">ParquetStringType</a>","synthetic":true,"types":["common_primitives::parquet::string::ParquetStringType"]},{"text":"impl Freeze for <a class=\"enum\" href=\"common_primitives/parquet/temporal/enum.ParquetTemporalType.html\" title=\"enum common_primitives::parquet::temporal::ParquetTemporalType\">ParquetTemporalType</a>","synthetic":true,"types":["common_primitives::parquet::temporal::ParquetTemporalType"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_primitives/parquet/temporal/struct.ParquetTime.html\" title=\"struct common_primitives::parquet::temporal::ParquetTime\">ParquetTime</a>","synthetic":true,"types":["common_primitives::parquet::temporal::ParquetTime"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_primitives/parquet/temporal/struct.ParquetTimestamp.html\" title=\"struct common_primitives::parquet::temporal::ParquetTimestamp\">ParquetTimestamp</a>","synthetic":true,"types":["common_primitives::parquet::temporal::ParquetTimestamp"]},{"text":"impl Freeze for <a class=\"enum\" href=\"common_primitives/parquet/types/enum.ParquetType.html\" title=\"enum common_primitives::parquet::types::ParquetType\">ParquetType</a>","synthetic":true,"types":["common_primitives::parquet::types::ParquetType"]},{"text":"impl Freeze for <a class=\"enum\" href=\"common_primitives/schema/enum.ModelType.html\" title=\"enum common_primitives::schema::ModelType\">ModelType</a>","synthetic":true,"types":["common_primitives::schema::ModelType"]},{"text":"impl Freeze for <a class=\"enum\" href=\"common_primitives/schema/enum.PayloadLocation.html\" title=\"enum common_primitives::schema::PayloadLocation\">PayloadLocation</a>","synthetic":true,"types":["common_primitives::schema::PayloadLocation"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_primitives/schema/struct.SchemaResponse.html\" title=\"struct common_primitives::schema::SchemaResponse\">SchemaResponse</a>","synthetic":true,"types":["common_primitives::schema::SchemaResponse"]}];
implementors["common_runtime"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.MaxDataSize.html\" title=\"struct common_runtime::constants::MaxDataSize\">MaxDataSize</a>","synthetic":true,"types":["common_runtime::constants::MaxDataSize"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.SchemasMaxRegistrations.html\" title=\"struct common_runtime::constants::SchemasMaxRegistrations\">SchemasMaxRegistrations</a>","synthetic":true,"types":["common_runtime::constants::SchemasMaxRegistrations"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.VestingPalletId.html\" title=\"struct common_runtime::constants::VestingPalletId\">VestingPalletId</a>","synthetic":true,"types":["common_runtime::constants::VestingPalletId"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.MinVestedTransfer.html\" title=\"struct common_runtime::constants::MinVestedTransfer\">MinVestedTransfer</a>","synthetic":true,"types":["common_runtime::constants::MinVestedTransfer"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.MinimumPeriod.html\" title=\"struct common_runtime::constants::MinimumPeriod\">MinimumPeriod</a>","synthetic":true,"types":["common_runtime::constants::MinimumPeriod"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.ExistentialDeposit.html\" title=\"struct common_runtime::constants::ExistentialDeposit\">ExistentialDeposit</a>","synthetic":true,"types":["common_runtime::constants::ExistentialDeposit"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.PreimageBaseDeposit.html\" title=\"struct common_runtime::constants::PreimageBaseDeposit\">PreimageBaseDeposit</a>","synthetic":true,"types":["common_runtime::constants::PreimageBaseDeposit"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.PreimageByteDeposit.html\" title=\"struct common_runtime::constants::PreimageByteDeposit\">PreimageByteDeposit</a>","synthetic":true,"types":["common_runtime::constants::PreimageByteDeposit"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.CouncilMotionDuration.html\" title=\"struct common_runtime::constants::CouncilMotionDuration\">CouncilMotionDuration</a>","synthetic":true,"types":["common_runtime::constants::CouncilMotionDuration"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.TCMotionDuration.html\" title=\"struct common_runtime::constants::TCMotionDuration\">TCMotionDuration</a>","synthetic":true,"types":["common_runtime::constants::TCMotionDuration"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.LaunchPeriod.html\" title=\"struct common_runtime::constants::LaunchPeriod\">LaunchPeriod</a>","synthetic":true,"types":["common_runtime::constants::LaunchPeriod"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.VotingPeriod.html\" title=\"struct common_runtime::constants::VotingPeriod\">VotingPeriod</a>","synthetic":true,"types":["common_runtime::constants::VotingPeriod"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.FastTrackVotingPeriod.html\" title=\"struct common_runtime::constants::FastTrackVotingPeriod\">FastTrackVotingPeriod</a>","synthetic":true,"types":["common_runtime::constants::FastTrackVotingPeriod"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.EnactmentPeriod.html\" title=\"struct common_runtime::constants::EnactmentPeriod\">EnactmentPeriod</a>","synthetic":true,"types":["common_runtime::constants::EnactmentPeriod"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.CooloffPeriod.html\" title=\"struct common_runtime::constants::CooloffPeriod\">CooloffPeriod</a>","synthetic":true,"types":["common_runtime::constants::CooloffPeriod"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.MinimumDeposit.html\" title=\"struct common_runtime::constants::MinimumDeposit\">MinimumDeposit</a>","synthetic":true,"types":["common_runtime::constants::MinimumDeposit"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.TransactionByteFee.html\" title=\"struct common_runtime::constants::TransactionByteFee\">TransactionByteFee</a>","synthetic":true,"types":["common_runtime::constants::TransactionByteFee"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.ReservedXcmpWeight.html\" title=\"struct common_runtime::constants::ReservedXcmpWeight\">ReservedXcmpWeight</a>","synthetic":true,"types":["common_runtime::constants::ReservedXcmpWeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.ReservedDmpWeight.html\" title=\"struct common_runtime::constants::ReservedDmpWeight\">ReservedDmpWeight</a>","synthetic":true,"types":["common_runtime::constants::ReservedDmpWeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.CollatorPotId.html\" title=\"struct common_runtime::constants::CollatorPotId\">CollatorPotId</a>","synthetic":true,"types":["common_runtime::constants::CollatorPotId"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/constants/struct.MessagesMaxPayloadSizeBytes.html\" title=\"struct common_runtime::constants::MessagesMaxPayloadSizeBytes\">MessagesMaxPayloadSizeBytes</a>","synthetic":true,"types":["common_runtime::constants::MessagesMaxPayloadSizeBytes"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"common_runtime/extensions/check_nonce/struct.CheckNonce.html\" title=\"struct common_runtime::extensions::check_nonce::CheckNonce\">CheckNonce</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::Index: Freeze,&nbsp;</span>","synthetic":true,"types":["common_runtime::extensions::check_nonce::CheckNonce"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/weights/block_weights/constants/struct.BlockExecutionWeight.html\" title=\"struct common_runtime::weights::block_weights::constants::BlockExecutionWeight\">BlockExecutionWeight</a>","synthetic":true,"types":["common_runtime::weights::block_weights::constants::BlockExecutionWeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"common_runtime/weights/extrinsic_weights/constants/struct.ExtrinsicBaseWeight.html\" title=\"struct common_runtime::weights::extrinsic_weights::constants::ExtrinsicBaseWeight\">ExtrinsicBaseWeight</a>","synthetic":true,"types":["common_runtime::weights::extrinsic_weights::constants::ExtrinsicBaseWeight"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"common_runtime/weights/orml_vesting/struct.SubstrateWeight.html\" title=\"struct common_runtime::weights::orml_vesting::SubstrateWeight\">SubstrateWeight</a>&lt;T&gt;","synthetic":true,"types":["common_runtime::weights::orml_vesting::SubstrateWeight"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"common_runtime/weights/pallet_democracy/struct.SubstrateWeight.html\" title=\"struct common_runtime::weights::pallet_democracy::SubstrateWeight\">SubstrateWeight</a>&lt;T&gt;","synthetic":true,"types":["common_runtime::weights::pallet_democracy::SubstrateWeight"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"common_runtime/weights/pallet_preimage/struct.SubstrateWeight.html\" title=\"struct common_runtime::weights::pallet_preimage::SubstrateWeight\">SubstrateWeight</a>&lt;T&gt;","synthetic":true,"types":["common_runtime::weights::pallet_preimage::SubstrateWeight"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"common_runtime/weights/pallet_scheduler/struct.SubstrateWeight.html\" title=\"struct common_runtime::weights::pallet_scheduler::SubstrateWeight\">SubstrateWeight</a>&lt;T&gt;","synthetic":true,"types":["common_runtime::weights::pallet_scheduler::SubstrateWeight"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"common_runtime/weights/pallet_utility/struct.SubstrateWeight.html\" title=\"struct common_runtime::weights::pallet_utility::SubstrateWeight\">SubstrateWeight</a>&lt;T&gt;","synthetic":true,"types":["common_runtime::weights::pallet_utility::SubstrateWeight"]}];
implementors["frequency_cli"] = [{"text":"impl Freeze for <a class=\"enum\" href=\"frequency_cli/enum.Subcommand.html\" title=\"enum frequency_cli::Subcommand\">Subcommand</a>","synthetic":true,"types":["frequency_cli::cli::Subcommand"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_cli/struct.Cli.html\" title=\"struct frequency_cli::Cli\">Cli</a>","synthetic":true,"types":["frequency_cli::cli::Cli"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_cli/struct.RelayChainCli.html\" title=\"struct frequency_cli::RelayChainCli\">RelayChainCli</a>","synthetic":true,"types":["frequency_cli::cli::RelayChainCli"]}];
implementors["frequency_rococo_runtime"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/xcm_config/struct.RelayLocation.html\" title=\"struct frequency_rococo_runtime::xcm_config::RelayLocation\">RelayLocation</a>","synthetic":true,"types":["frequency_rococo_runtime::xcm_config::RelayLocation"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/xcm_config/struct.RelayNetwork.html\" title=\"struct frequency_rococo_runtime::xcm_config::RelayNetwork\">RelayNetwork</a>","synthetic":true,"types":["frequency_rococo_runtime::xcm_config::RelayNetwork"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/xcm_config/struct.RelayChainOrigin.html\" title=\"struct frequency_rococo_runtime::xcm_config::RelayChainOrigin\">RelayChainOrigin</a>","synthetic":true,"types":["frequency_rococo_runtime::xcm_config::RelayChainOrigin"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/xcm_config/struct.Ancestry.html\" title=\"struct frequency_rococo_runtime::xcm_config::Ancestry\">Ancestry</a>","synthetic":true,"types":["frequency_rococo_runtime::xcm_config::Ancestry"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/xcm_config/struct.UnitWeightCost.html\" title=\"struct frequency_rococo_runtime::xcm_config::UnitWeightCost\">UnitWeightCost</a>","synthetic":true,"types":["frequency_rococo_runtime::xcm_config::UnitWeightCost"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/xcm_config/struct.MaxInstructions.html\" title=\"struct frequency_rococo_runtime::xcm_config::MaxInstructions\">MaxInstructions</a>","synthetic":true,"types":["frequency_rococo_runtime::xcm_config::MaxInstructions"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/xcm_config/struct.ParentOrParentsExecutivePlurality.html\" title=\"struct frequency_rococo_runtime::xcm_config::ParentOrParentsExecutivePlurality\">ParentOrParentsExecutivePlurality</a>","synthetic":true,"types":["frequency_rococo_runtime::xcm_config::ParentOrParentsExecutivePlurality"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/xcm_config/struct.XcmConfig.html\" title=\"struct frequency_rococo_runtime::xcm_config::XcmConfig\">XcmConfig</a>","synthetic":true,"types":["frequency_rococo_runtime::xcm_config::XcmConfig"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.WeightToFee.html\" title=\"struct frequency_rococo_runtime::WeightToFee\">WeightToFee</a>","synthetic":true,"types":["frequency_rococo_runtime::WeightToFee"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.SessionKeys.html\" title=\"struct frequency_rococo_runtime::SessionKeys\">SessionKeys</a>","synthetic":true,"types":["frequency_rococo_runtime::SessionKeys"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.Version.html\" title=\"struct frequency_rococo_runtime::Version\">Version</a>","synthetic":true,"types":["frequency_rococo_runtime::Version"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.RuntimeBlockLength.html\" title=\"struct frequency_rococo_runtime::RuntimeBlockLength\">RuntimeBlockLength</a>","synthetic":true,"types":["frequency_rococo_runtime::RuntimeBlockLength"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.RuntimeBlockWeights.html\" title=\"struct frequency_rococo_runtime::RuntimeBlockWeights\">RuntimeBlockWeights</a>","synthetic":true,"types":["frequency_rococo_runtime::RuntimeBlockWeights"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.SS58Prefix.html\" title=\"struct frequency_rococo_runtime::SS58Prefix\">SS58Prefix</a>","synthetic":true,"types":["frequency_rococo_runtime::SS58Prefix"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.RootAsVestingPallet.html\" title=\"struct frequency_rococo_runtime::RootAsVestingPallet\">RootAsVestingPallet</a>","synthetic":true,"types":["frequency_rococo_runtime::RootAsVestingPallet"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.MaxVestingSchedules.html\" title=\"struct frequency_rococo_runtime::MaxVestingSchedules\">MaxVestingSchedules</a>","synthetic":true,"types":["frequency_rococo_runtime::MaxVestingSchedules"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.MaximumSchedulerWeight.html\" title=\"struct frequency_rococo_runtime::MaximumSchedulerWeight\">MaximumSchedulerWeight</a>","synthetic":true,"types":["frequency_rococo_runtime::MaximumSchedulerWeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.NoPreimagePostponement.html\" title=\"struct frequency_rococo_runtime::NoPreimagePostponement\">NoPreimagePostponement</a>","synthetic":true,"types":["frequency_rococo_runtime::NoPreimagePostponement"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.ReservedXcmpWeight.html\" title=\"struct frequency_rococo_runtime::ReservedXcmpWeight\">ReservedXcmpWeight</a>","synthetic":true,"types":["frequency_rococo_runtime::ReservedXcmpWeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.ReservedDmpWeight.html\" title=\"struct frequency_rococo_runtime::ReservedDmpWeight\">ReservedDmpWeight</a>","synthetic":true,"types":["frequency_rococo_runtime::ReservedDmpWeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.Runtime.html\" title=\"struct frequency_rococo_runtime::Runtime\">Runtime</a>","synthetic":true,"types":["frequency_rococo_runtime::Runtime"]},{"text":"impl Freeze for <a class=\"enum\" href=\"frequency_rococo_runtime/enum.Event.html\" title=\"enum frequency_rococo_runtime::Event\">Event</a>","synthetic":true,"types":["frequency_rococo_runtime::Event"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.Origin.html\" title=\"struct frequency_rococo_runtime::Origin\">Origin</a>","synthetic":true,"types":["frequency_rococo_runtime::Origin"]},{"text":"impl Freeze for <a class=\"enum\" href=\"frequency_rococo_runtime/enum.OriginCaller.html\" title=\"enum frequency_rococo_runtime::OriginCaller\">OriginCaller</a>","synthetic":true,"types":["frequency_rococo_runtime::OriginCaller"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.PalletInfo.html\" title=\"struct frequency_rococo_runtime::PalletInfo\">PalletInfo</a>","synthetic":true,"types":["frequency_rococo_runtime::PalletInfo"]},{"text":"impl Freeze for <a class=\"enum\" href=\"frequency_rococo_runtime/enum.Call.html\" title=\"enum frequency_rococo_runtime::Call\">Call</a>","synthetic":true,"types":["frequency_rococo_runtime::Call"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.GenesisConfig.html\" title=\"struct frequency_rococo_runtime::GenesisConfig\">GenesisConfig</a>","synthetic":true,"types":["frequency_rococo_runtime::GenesisConfig"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.RuntimeApi.html\" title=\"struct frequency_rococo_runtime::RuntimeApi\">RuntimeApi</a>","synthetic":true,"types":["frequency_rococo_runtime::RuntimeApi"]},{"text":"impl&lt;Block, C&gt; !Freeze for <a class=\"struct\" href=\"frequency_rococo_runtime/struct.RuntimeApiImpl.html\" title=\"struct frequency_rococo_runtime::RuntimeApiImpl\">RuntimeApiImpl</a>&lt;Block, C&gt;","synthetic":true,"types":["frequency_rococo_runtime::RuntimeApiImpl"]}];
implementors["frequency_runtime"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/xcm_config/struct.RelayLocation.html\" title=\"struct frequency_runtime::xcm_config::RelayLocation\">RelayLocation</a>","synthetic":true,"types":["frequency_runtime::xcm_config::RelayLocation"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/xcm_config/struct.RelayNetwork.html\" title=\"struct frequency_runtime::xcm_config::RelayNetwork\">RelayNetwork</a>","synthetic":true,"types":["frequency_runtime::xcm_config::RelayNetwork"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/xcm_config/struct.RelayChainOrigin.html\" title=\"struct frequency_runtime::xcm_config::RelayChainOrigin\">RelayChainOrigin</a>","synthetic":true,"types":["frequency_runtime::xcm_config::RelayChainOrigin"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/xcm_config/struct.Ancestry.html\" title=\"struct frequency_runtime::xcm_config::Ancestry\">Ancestry</a>","synthetic":true,"types":["frequency_runtime::xcm_config::Ancestry"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/xcm_config/struct.UnitWeightCost.html\" title=\"struct frequency_runtime::xcm_config::UnitWeightCost\">UnitWeightCost</a>","synthetic":true,"types":["frequency_runtime::xcm_config::UnitWeightCost"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/xcm_config/struct.MaxInstructions.html\" title=\"struct frequency_runtime::xcm_config::MaxInstructions\">MaxInstructions</a>","synthetic":true,"types":["frequency_runtime::xcm_config::MaxInstructions"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/xcm_config/struct.ParentOrParentsExecutivePlurality.html\" title=\"struct frequency_runtime::xcm_config::ParentOrParentsExecutivePlurality\">ParentOrParentsExecutivePlurality</a>","synthetic":true,"types":["frequency_runtime::xcm_config::ParentOrParentsExecutivePlurality"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/xcm_config/struct.XcmConfig.html\" title=\"struct frequency_runtime::xcm_config::XcmConfig\">XcmConfig</a>","synthetic":true,"types":["frequency_runtime::xcm_config::XcmConfig"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.BaseCallFilter.html\" title=\"struct frequency_runtime::BaseCallFilter\">BaseCallFilter</a>","synthetic":true,"types":["frequency_runtime::BaseCallFilter"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.WeightToFee.html\" title=\"struct frequency_runtime::WeightToFee\">WeightToFee</a>","synthetic":true,"types":["frequency_runtime::WeightToFee"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.SessionKeys.html\" title=\"struct frequency_runtime::SessionKeys\">SessionKeys</a>","synthetic":true,"types":["frequency_runtime::SessionKeys"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.Version.html\" title=\"struct frequency_runtime::Version\">Version</a>","synthetic":true,"types":["frequency_runtime::Version"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.RuntimeBlockLength.html\" title=\"struct frequency_runtime::RuntimeBlockLength\">RuntimeBlockLength</a>","synthetic":true,"types":["frequency_runtime::RuntimeBlockLength"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.RuntimeBlockWeights.html\" title=\"struct frequency_runtime::RuntimeBlockWeights\">RuntimeBlockWeights</a>","synthetic":true,"types":["frequency_runtime::RuntimeBlockWeights"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.SS58Prefix.html\" title=\"struct frequency_runtime::SS58Prefix\">SS58Prefix</a>","synthetic":true,"types":["frequency_runtime::SS58Prefix"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.RootAsVestingPallet.html\" title=\"struct frequency_runtime::RootAsVestingPallet\">RootAsVestingPallet</a>","synthetic":true,"types":["frequency_runtime::RootAsVestingPallet"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.MaxVestingSchedules.html\" title=\"struct frequency_runtime::MaxVestingSchedules\">MaxVestingSchedules</a>","synthetic":true,"types":["frequency_runtime::MaxVestingSchedules"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.ExistentialDeposit.html\" title=\"struct frequency_runtime::ExistentialDeposit\">ExistentialDeposit</a>","synthetic":true,"types":["frequency_runtime::ExistentialDeposit"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.MaximumSchedulerWeight.html\" title=\"struct frequency_runtime::MaximumSchedulerWeight\">MaximumSchedulerWeight</a>","synthetic":true,"types":["frequency_runtime::MaximumSchedulerWeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.NoPreimagePostponement.html\" title=\"struct frequency_runtime::NoPreimagePostponement\">NoPreimagePostponement</a>","synthetic":true,"types":["frequency_runtime::NoPreimagePostponement"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.ReservedXcmpWeight.html\" title=\"struct frequency_runtime::ReservedXcmpWeight\">ReservedXcmpWeight</a>","synthetic":true,"types":["frequency_runtime::ReservedXcmpWeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.ReservedDmpWeight.html\" title=\"struct frequency_runtime::ReservedDmpWeight\">ReservedDmpWeight</a>","synthetic":true,"types":["frequency_runtime::ReservedDmpWeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.Runtime.html\" title=\"struct frequency_runtime::Runtime\">Runtime</a>","synthetic":true,"types":["frequency_runtime::Runtime"]},{"text":"impl Freeze for <a class=\"enum\" href=\"frequency_runtime/enum.Event.html\" title=\"enum frequency_runtime::Event\">Event</a>","synthetic":true,"types":["frequency_runtime::Event"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.Origin.html\" title=\"struct frequency_runtime::Origin\">Origin</a>","synthetic":true,"types":["frequency_runtime::Origin"]},{"text":"impl Freeze for <a class=\"enum\" href=\"frequency_runtime/enum.OriginCaller.html\" title=\"enum frequency_runtime::OriginCaller\">OriginCaller</a>","synthetic":true,"types":["frequency_runtime::OriginCaller"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.PalletInfo.html\" title=\"struct frequency_runtime::PalletInfo\">PalletInfo</a>","synthetic":true,"types":["frequency_runtime::PalletInfo"]},{"text":"impl Freeze for <a class=\"enum\" href=\"frequency_runtime/enum.Call.html\" title=\"enum frequency_runtime::Call\">Call</a>","synthetic":true,"types":["frequency_runtime::Call"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.GenesisConfig.html\" title=\"struct frequency_runtime::GenesisConfig\">GenesisConfig</a>","synthetic":true,"types":["frequency_runtime::GenesisConfig"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.RuntimeApi.html\" title=\"struct frequency_runtime::RuntimeApi\">RuntimeApi</a>","synthetic":true,"types":["frequency_runtime::RuntimeApi"]},{"text":"impl&lt;Block, C&gt; !Freeze for <a class=\"struct\" href=\"frequency_runtime/struct.RuntimeApiImpl.html\" title=\"struct frequency_runtime::RuntimeApiImpl\">RuntimeApiImpl</a>&lt;Block, C&gt;","synthetic":true,"types":["frequency_runtime::RuntimeApiImpl"]}];
implementors["frequency_service"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_service/chain_spec/struct.Extensions.html\" title=\"struct frequency_service::chain_spec::Extensions\">Extensions</a>","synthetic":true,"types":["frequency_service::chain_spec::Extensions"]},{"text":"impl Freeze for <a class=\"struct\" href=\"frequency_service/chain_spec/struct.ExtensionsFork.html\" title=\"struct frequency_service::chain_spec::ExtensionsFork\">ExtensionsFork</a>","synthetic":true,"types":["frequency_service::chain_spec::ExtensionsFork"]},{"text":"impl&lt;C, P&gt; Freeze for <a class=\"struct\" href=\"frequency_service/rpc/struct.FullDeps.html\" title=\"struct frequency_service::rpc::FullDeps\">FullDeps</a>&lt;C, P&gt;","synthetic":true,"types":["frequency_service::rpc::FullDeps"]}];
implementors["pallet_messages"] = [{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"pallet_messages/weights/struct.SubstrateWeight.html\" title=\"struct pallet_messages::weights::SubstrateWeight\">SubstrateWeight</a>&lt;T&gt;","synthetic":true,"types":["pallet_messages::weights::SubstrateWeight"]},{"text":"impl&lt;MaxDataSize&gt; Freeze for <a class=\"struct\" href=\"pallet_messages/struct.Message.html\" title=\"struct pallet_messages::Message\">Message</a>&lt;MaxDataSize&gt;","synthetic":true,"types":["pallet_messages::types::Message"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"pallet_messages/pallet/struct.Pallet.html\" title=\"struct pallet_messages::pallet::Pallet\">Pallet</a>&lt;T&gt;","synthetic":true,"types":["pallet_messages::pallet::Pallet"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"pallet_messages/pallet/enum.Error.html\" title=\"enum pallet_messages::pallet::Error\">Error</a>&lt;T&gt;","synthetic":true,"types":["pallet_messages::pallet::Error"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"pallet_messages/pallet/enum.Event.html\" title=\"enum pallet_messages::pallet::Event\">Event</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::BlockNumber: Freeze,&nbsp;</span>","synthetic":true,"types":["pallet_messages::pallet::Event"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"pallet_messages/pallet/enum.Call.html\" title=\"enum pallet_messages::pallet::Call\">Call</a>&lt;T&gt;","synthetic":true,"types":["pallet_messages::pallet::Call"]}];
implementors["pallet_messages_rpc"] = [{"text":"impl&lt;C, M&gt; Freeze for <a class=\"struct\" href=\"pallet_messages_rpc/struct.MessagesHandler.html\" title=\"struct pallet_messages_rpc::MessagesHandler\">MessagesHandler</a>&lt;C, M&gt;","synthetic":true,"types":["pallet_messages_rpc::MessagesHandler"]},{"text":"impl Freeze for <a class=\"enum\" href=\"pallet_messages_rpc/enum.MessageRpcError.html\" title=\"enum pallet_messages_rpc::MessageRpcError\">MessageRpcError</a>","synthetic":true,"types":["pallet_messages_rpc::MessageRpcError"]}];
implementors["pallet_msa"] = [{"text":"impl Freeze for <a class=\"struct\" href=\"pallet_msa/types/struct.AddKeyData.html\" title=\"struct pallet_msa::types::AddKeyData\">AddKeyData</a>","synthetic":true,"types":["pallet_msa::types::AddKeyData"]},{"text":"impl Freeze for <a class=\"struct\" href=\"pallet_msa/types/struct.AddProvider.html\" title=\"struct pallet_msa::types::AddProvider\">AddProvider</a>","synthetic":true,"types":["pallet_msa::types::AddProvider"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"pallet_msa/weights/struct.SubstrateWeight.html\" title=\"struct pallet_msa::weights::SubstrateWeight\">SubstrateWeight</a>&lt;T&gt;","synthetic":true,"types":["pallet_msa::weights::SubstrateWeight"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"pallet_msa/pallet/struct.Pallet.html\" title=\"struct pallet_msa::pallet::Pallet\">Pallet</a>&lt;T&gt;","synthetic":true,"types":["pallet_msa::pallet::Pallet"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"pallet_msa/pallet/enum.Event.html\" title=\"enum pallet_msa::pallet::Event\">Event</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Freeze,&nbsp;</span>","synthetic":true,"types":["pallet_msa::pallet::Event"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"pallet_msa/pallet/enum.Error.html\" title=\"enum pallet_msa::pallet::Error\">Error</a>&lt;T&gt;","synthetic":true,"types":["pallet_msa::pallet::Error"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"pallet_msa/pallet/enum.Call.html\" title=\"enum pallet_msa::pallet::Call\">Call</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Freeze,&nbsp;</span>","synthetic":true,"types":["pallet_msa::pallet::Call"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"pallet_msa/struct.CheckFreeExtrinsicUse.html\" title=\"struct pallet_msa::CheckFreeExtrinsicUse\">CheckFreeExtrinsicUse</a>&lt;T&gt;","synthetic":true,"types":["pallet_msa::CheckFreeExtrinsicUse"]}];
implementors["pallet_msa_rpc"] = [{"text":"impl&lt;C, M&gt; Freeze for <a class=\"struct\" href=\"pallet_msa_rpc/struct.MsaHandler.html\" title=\"struct pallet_msa_rpc::MsaHandler\">MsaHandler</a>&lt;C, M&gt;","synthetic":true,"types":["pallet_msa_rpc::MsaHandler"]}];
implementors["pallet_schemas"] = [{"text":"impl&lt;MaxModelSize&gt; Freeze for <a class=\"struct\" href=\"pallet_schemas/struct.Schema.html\" title=\"struct pallet_schemas::Schema\">Schema</a>&lt;MaxModelSize&gt;","synthetic":true,"types":["pallet_schemas::types::Schema"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"pallet_schemas/weights/struct.SubstrateWeight.html\" title=\"struct pallet_schemas::weights::SubstrateWeight\">SubstrateWeight</a>&lt;T&gt;","synthetic":true,"types":["pallet_schemas::weights::SubstrateWeight"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"pallet_schemas/pallet/enum.Event.html\" title=\"enum pallet_schemas::pallet::Event\">Event</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Freeze,&nbsp;</span>","synthetic":true,"types":["pallet_schemas::pallet::Event"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"pallet_schemas/pallet/enum.Error.html\" title=\"enum pallet_schemas::pallet::Error\">Error</a>&lt;T&gt;","synthetic":true,"types":["pallet_schemas::pallet::Error"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"pallet_schemas/pallet/struct.Pallet.html\" title=\"struct pallet_schemas::pallet::Pallet\">Pallet</a>&lt;T&gt;","synthetic":true,"types":["pallet_schemas::pallet::Pallet"]},{"text":"impl Freeze for <a class=\"struct\" href=\"pallet_schemas/pallet/struct.GenesisConfig.html\" title=\"struct pallet_schemas::pallet::GenesisConfig\">GenesisConfig</a>","synthetic":true,"types":["pallet_schemas::pallet::GenesisConfig"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"enum\" href=\"pallet_schemas/pallet/enum.Call.html\" title=\"enum pallet_schemas::pallet::Call\">Call</a>&lt;T&gt;","synthetic":true,"types":["pallet_schemas::pallet::Call"]}];
implementors["pallet_schemas_rpc"] = [{"text":"impl Freeze for <a class=\"enum\" href=\"pallet_schemas_rpc/enum.Error.html\" title=\"enum pallet_schemas_rpc::Error\">Error</a>","synthetic":true,"types":["pallet_schemas_rpc::Error"]},{"text":"impl&lt;C, M&gt; Freeze for <a class=\"struct\" href=\"pallet_schemas_rpc/struct.SchemasHandler.html\" title=\"struct pallet_schemas_rpc::SchemasHandler\">SchemasHandler</a>&lt;C, M&gt;","synthetic":true,"types":["pallet_schemas_rpc::SchemasHandler"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()