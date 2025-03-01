// This file is part of Basilisk-node.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use crate::governance::{MajorityTechCommitteeOrRoot, SuperMajorityCouncilOrRoot, TreasuryAccount};
use crate::system::WeightToFee;

use codec::{Decode, Encode, MaxEncodedLen};
use cumulus_primitives_core::ParaId;
use frame_support::{
	parameter_types,
	sp_runtime::traits::Convert,
	traits::{Contains, Everything, Get, Nothing},
	PalletId,
};
use frame_system::EnsureRoot;
use hydradx_adapters::xcm_exchange::XcmAssetExchanger;
use hydradx_adapters::xcm_execute_filter::AllowTransferAndSwap;
use hydradx_adapters::{MultiCurrencyTrader, ToFeeReceiver};
use hydradx_traits::router::PoolType;
use orml_traits::{location::AbsoluteReserveProvider, parameter_type_with_key};
pub use orml_xcm_support::{DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset};
use pallet_transaction_multi_payment::DepositAll;
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::RelayChainBlockNumber;
use polkadot_parachain::primitives::Sibling;
use polkadot_xcm::v3::{prelude::*, Error, MultiLocation, Weight as XcmWeight};
use primitives::AssetId;
use scale_info::TypeInfo;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom,
	EnsureXcmOrigin, FixedWeightBounds, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
	SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation,
	TakeWeightCredit, WithComputedOrigin,
};
use xcm_executor::{traits::WeightTrader, Assets, Config, XcmExecutor};

#[derive(Debug, Default, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct AssetLocation(pub MultiLocation);

pub const RELAY_CHAIN_ASSET_LOCATION: AssetLocation = AssetLocation(MultiLocation {
	parents: 1,
	interior: Here,
});

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

pub type Barrier = (
	TakeWeightCredit,
	// Expected responses are OK.
	AllowKnownQueryResponses<PolkadotXcm>,
	WithComputedOrigin<
		(
			AllowTopLevelPaidExecutionFrom<Everything>,
			// Subscriptions for version tracking are OK.
			AllowSubscriptionsFrom<Everything>,
		),
		UniversalLocation,
		ConstU32<8>,
	>,
);

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::get().into())));
}

parameter_types! {
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;

	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();

	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToCallOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
	// Derives signed AccountId32 origins for Tinkernet multisigs.
	invarch_xcm_builder::DeriveOriginFromTinkernetMultisig<RuntimeOrigin>,
);

parameter_types! {
	/// The amount of weight an XCM operation takes. This is a safe overestimate.
	pub const BaseXcmWeight: XcmWeight = XcmWeight::from_ref_time(100_000_000);
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsForTransfer: usize = 2;
	pub UniversalLocation: InteriorMultiLocation = X2(GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into()));
}

pub struct TradePassthrough();
impl WeightTrader for TradePassthrough {
	fn new() -> Self {
		Self()
	}

	fn buy_weight(&mut self, _weight: Weight, payment: Assets) -> Result<Assets, Error> {
		// Just let it through for now
		Ok(payment)
	}
}

pub struct XcmConfig;
impl Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;

	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToCallOrigin;
	type IsReserve = MultiNativeAsset<AbsoluteReserveProvider>;

	type IsTeleporter = (); // disabled
	type UniversalLocation = UniversalLocation;

	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	// We calculate weight fees the same way as for regular extrinsics and use the prices and choice
	// of accepted currencies of the transaction payment pallet. Fees go to the same fee receiver as
	// configured in `MultiTransactionPayment`.
	type Trader = MultiCurrencyTrader<
		AssetId,
		Balance,
		Price,
		WeightToFee,
		MultiTransactionPayment,
		CurrencyIdConvert,
		ToFeeReceiver<AccountId, AssetId, Balance, Price, CurrencyIdConvert, DepositAll<Runtime>, TreasuryAccount>,
	>;

	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetLocker = ();
	type AssetExchanger = XcmAssetExchanger<Runtime, TempAccount, CurrencyIdConvert, Currencies, DefaultPoolType>;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = ConstU32<64>;
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = SafeCallFilter;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = PolkadotXcm;
	type ExecuteOverweightOrigin = MajorityTechCommitteeOrRoot;
	type ControllerOrigin = MajorityTechCommitteeOrRoot;
	type ControllerOriginConverter = XcmOriginToCallOrigin;
	type PriceForSiblingDelivery = ();
	type WeightInfo = weights::xcmp_queue::BasiliskWeight<Runtime>;
	type ExecuteDeferredOrigin = EnsureRoot<AccountId>;
	type MaxDeferredMessages = ConstU32<100>;
	type RelayChainBlockNumberProvider = RelayChainBlockNumberProvider<Runtime>;
	type XcmDeferFilter = XcmRateLimiter;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = MajorityTechCommitteeOrRoot;
}

parameter_type_with_key! {
	pub ParachainMinFee: |_location: MultiLocation| -> Option<u128> {
		None
	};
}

impl orml_xtokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type CurrencyId = AssetId;
	type CurrencyIdConvert = CurrencyIdConvert;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = SelfLocation;
	type MinXcmFee = ParachainMinFee;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type MultiLocationsFilter = Everything;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type UniversalLocation = UniversalLocation;
	type MaxAssetsForTransfer = MaxAssetsForTransfer;
	type ReserveProvider = AbsoluteReserveProvider;
}

impl orml_unknown_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

impl orml_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SovereignOrigin = SuperMajorityCouncilOrRoot;
}

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
}

parameter_types! {
	//Xcm asset exchange
	pub DefaultPoolType: PoolType<AssetId>  = PoolType::XYK;
	pub TempAccount: AccountId = [42; 32].into();

	//Xcm executor filter
	pub const MaxXcmDepth: u16 = 5;
	pub const MaxNumberOfInstructions: u16 = 100;
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = AllowTransferAndSwap<MaxXcmDepth, MaxNumberOfInstructions, RuntimeCall>;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type TrustedLockers = ();
	type SovereignAccountOf = ();
	type MaxLockers = ConstU32<8>;
	type WeightInfo = weights::xcm::BasiliskWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;
}

parameter_types! {
	pub DeferDuration: RelayChainBlockNumber = 100; // 10 min
	pub MaxDeferDuration: RelayChainBlockNumber = 600 * 24 * 10; // 10 days
}

impl pallet_xcm_rate_limiter::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = AssetId;
	type DeferDuration = DeferDuration;
	type MaxDeferDuration = MaxDeferDuration;
	type RelayBlockNumberProvider = RelayChainBlockNumberProvider<Runtime>;
	type CurrencyIdConvert = CurrencyIdConvert;
	type RateLimitFor = pallet_asset_registry::XcmRateLimitsInRegistry<Runtime>;
}

pub struct CurrencyIdConvert;
use primitives::constants::chain::CORE_ASSET_ID;

impl Convert<AssetId, Option<MultiLocation>> for CurrencyIdConvert {
	fn convert(id: AssetId) -> Option<MultiLocation> {
		match id {
			CORE_ASSET_ID => Some(MultiLocation::new(
				1,
				X2(Parachain(ParachainInfo::get().into()), GeneralIndex(id.into())),
			)),
			_ => AssetRegistry::asset_to_location(id).map(|loc| loc.0),
		}
	}
}

impl Convert<MultiLocation, Option<AssetId>> for CurrencyIdConvert {
	fn convert(location: MultiLocation) -> Option<AssetId> {
		match location {
			MultiLocation {
				parents,
				interior: X2(Parachain(id), GeneralIndex(index)),
			} if parents == 1 && ParaId::from(id) == ParachainInfo::get() && (index as u32) == CORE_ASSET_ID => {
				// Handling native asset for this parachain
				Some(CORE_ASSET_ID)
			}
			// handle reanchor canonical location: https://github.com/paritytech/polkadot/pull/4470
			MultiLocation {
				parents: 0,
				interior: X1(GeneralIndex(index)),
			} if (index as u32) == CORE_ASSET_ID => Some(CORE_ASSET_ID),
			// delegate to asset-registry
			_ => AssetRegistry::location_to_asset(AssetLocation(location)),
		}
	}
}

impl Convert<MultiAsset, Option<AssetId>> for CurrencyIdConvert {
	fn convert(asset: MultiAsset) -> Option<AssetId> {
		if let MultiAsset {
			id: Concrete(location), ..
		} = asset
		{
			Self::convert(location)
		} else {
			None
		}
	}
}

pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		X1(AccountId32 {
			network: None,
			id: account.into(),
		})
		.into()
	}
}

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the default `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
	// Mapping Tinkernet multisig to the correctly derived AccountId32.
	invarch_xcm_builder::TinkernetMultisigAsAccountId<AccountId>,
);

parameter_types! {
	// The account which receives multi-currency tokens from failed attempts to deposit them
	pub Alternative: AccountId = PalletId(*b"xcm/alte").into_account_truncating();
}

pub type LocalAssetTransactor = MultiCurrencyAdapter<
	Currencies,
	UnknownTokens,
	IsNativeConcrete<AssetId, CurrencyIdConvert>,
	AccountId,
	LocationToAccountId,
	AssetId,
	CurrencyIdConvert,
	DepositToAlternative<Alternative, Currencies, AssetId, AccountId, Balance>,
>;

/// A call filter for the XCM Transact instruction. This is a temporary measure until we properly
/// account for proof size weights.
///
/// Calls that are allowed through this filter must:
/// 1. Have a fixed weight;
/// 2. Cannot lead to another call being made;
/// 3. Have a defined proof size weight, e.g. no unbounded vecs in call parameters.
pub struct SafeCallFilter;
impl Contains<RuntimeCall> for SafeCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		#[cfg(feature = "runtime-benchmarks")]
		{
			if matches!(call, RuntimeCall::System(frame_system::Call::remark_with_event { .. })) {
				return true;
			}
		}

		// check the runtime call filter
		if !BaseFilter::contains(call) {
			return false;
		}

		match call {
			RuntimeCall::System(frame_system::Call::kill_prefix { .. } | frame_system::Call::set_heap_pages { .. })
			| RuntimeCall::Timestamp(..)
			| RuntimeCall::Balances(..)
			| RuntimeCall::Treasury(..)
			| RuntimeCall::Utility(pallet_utility::Call::as_derivative { .. })
			| RuntimeCall::Vesting(..)
			| RuntimeCall::Proxy(..)
			| RuntimeCall::CollatorSelection(
				pallet_collator_selection::Call::set_desired_candidates { .. }
				| pallet_collator_selection::Call::set_candidacy_bond { .. }
				| pallet_collator_selection::Call::register_as_candidate { .. }
				| pallet_collator_selection::Call::leave_intent { .. },
			)
			| RuntimeCall::Session(pallet_session::Call::purge_keys { .. })
			| RuntimeCall::Uniques(
				pallet_uniques::Call::create { .. }
				| pallet_uniques::Call::force_create { .. }
				| pallet_uniques::Call::mint { .. }
				| pallet_uniques::Call::burn { .. }
				| pallet_uniques::Call::transfer { .. }
				| pallet_uniques::Call::freeze { .. }
				| pallet_uniques::Call::thaw { .. }
				| pallet_uniques::Call::freeze_collection { .. }
				| pallet_uniques::Call::thaw_collection { .. }
				| pallet_uniques::Call::transfer_ownership { .. }
				| pallet_uniques::Call::set_team { .. }
				| pallet_uniques::Call::approve_transfer { .. }
				| pallet_uniques::Call::cancel_approval { .. }
				| pallet_uniques::Call::force_item_status { .. }
				| pallet_uniques::Call::set_attribute { .. }
				| pallet_uniques::Call::clear_attribute { .. }
				| pallet_uniques::Call::set_metadata { .. }
				| pallet_uniques::Call::clear_metadata { .. }
				| pallet_uniques::Call::set_collection_metadata { .. }
				| pallet_uniques::Call::clear_collection_metadata { .. }
				| pallet_uniques::Call::set_accept_ownership { .. }
				| pallet_uniques::Call::set_price { .. }
				| pallet_uniques::Call::buy_item { .. },
			)
			| RuntimeCall::Identity(
				pallet_identity::Call::add_registrar { .. }
				| pallet_identity::Call::set_identity { .. }
				| pallet_identity::Call::clear_identity { .. }
				| pallet_identity::Call::request_judgement { .. }
				| pallet_identity::Call::cancel_request { .. }
				| pallet_identity::Call::set_fee { .. }
				| pallet_identity::Call::set_account_id { .. }
				| pallet_identity::Call::set_fields { .. }
				| pallet_identity::Call::provide_judgement { .. }
				| pallet_identity::Call::kill_identity { .. }
				| pallet_identity::Call::add_sub { .. }
				| pallet_identity::Call::rename_sub { .. }
				| pallet_identity::Call::remove_sub { .. }
				| pallet_identity::Call::quit_sub { .. },
			)
			| RuntimeCall::XYK(..)
			| RuntimeCall::NFT(..)
			| RuntimeCall::MultiTransactionPayment(..)
			| RuntimeCall::XYKLiquidityMining(..)
			| RuntimeCall::Currencies(..)
			| RuntimeCall::Tokens(..)
			| RuntimeCall::OrmlXcm(..) => true,
			_ => false,
		}
	}
}
