use std::fmt;

use crate::types::{
    AccountId, BytesM, Hash, ScAddress, ScSymbol, ScVal, SignerKeyEd25519SignedPayload, String32,
    String64, TimePoint, Uint256, VecM,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerKey {
    Account(LedgerKeyAccount),
    Trustline(LedgerKeyTrustLine),
    Offer(LedgerKeyOffer),
    Data(LedgerKeyData),
    ClaimableBalance(LedgerKeyClaimableBalance),
    LiquidityPool(LedgerKeyLiquidityPool),
    ContractData(LedgerKeyContractData),
    ContractCode(LedgerKeyContractCode),
    ConfigSetting(LedgerKeyConfigSetting),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerKeyAccount {
    pub account_id: AccountId,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerKeyTrustLine {
    pub account_id: AccountId,
    pub asset: TrustLineAsset,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum TrustLineAsset {
    Native,
    CreditAlphanum4(AlphaNum4),
    CreditAlphanum12(AlphaNum12),
    PoolShare(PoolId),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct AlphaNum4 {
    pub asset_code: AssetCode4,
    pub issuer: AccountId,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
pub struct AssetCode4(pub [u8; 4]);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct AlphaNum12 {
    pub asset_code: AssetCode12,
    pub issuer: AccountId,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct PoolId(pub crate::types::Hash);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
pub struct AssetCode12(pub [u8; 12]);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerKeyOffer {
    pub seller_id: AccountId,
    pub offer_id: i64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerKeyData {
    pub account_id: AccountId,
    pub data_name: String64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerKeyClaimableBalance {
    pub balance_id: ClaimableBalanceId,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimableBalanceId {
    ClaimableBalanceIdTypeV0(crate::types::Hash),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerKeyLiquidityPool {
    pub liquidity_pool_id: PoolId,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerKeyContractData {
    pub contract: ScAddress,
    pub key: ScVal,
    pub durability: ContractDataDurability,
    pub body_type: ContractEntryBodyType,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(i32)]
pub enum ContractDataDurability {
    Temporary = 0,
    Persistent = 1,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(i32)]
pub enum ContractEntryBodyType {
    DataEntry = 0,
    ExpirationExtension = 1,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerKeyContractCode {
    pub hash: crate::types::Hash,
    pub body_type: ContractEntryBodyType,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerKeyConfigSetting {
    pub config_setting_id: ConfigSettingId,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(i32)]
pub enum ConfigSettingId {
    ContractMaxSizeBytes = 0,
    ContractComputeV0 = 1,
    ContractLedgerCostV0 = 2,
    ContractHistoricalDataV0 = 3,
    ContractMetaDataV0 = 4,
    ContractBandwidthV0 = 5,
    ContractCostParamsCpuInstructions = 6,
    ContractCostParamsMemoryBytes = 7,
    ContractDataKeySizeBytes = 8,
    ContractDataEntrySizeBytes = 9,
    StateExpiration = 10,
    ContractExecutionLanes = 11,
    BucketlistSizeWindow = 12,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerEntry {
    pub last_modified_ledger_seq: u32,
    pub data: LedgerEntryData,
    pub ext: LedgerEntryExt,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum LedgerEntryExt {
    V0,
    V1(LedgerEntryExtensionV1),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LedgerEntryExtensionV1 {
    pub sponsoring_id: SponsorshipDescriptor,
    pub ext: LedgerEntryExtensionV1Ext,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct SponsorshipDescriptor(pub Option<AccountId>);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum LedgerEntryExtensionV1Ext {
    V0,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum LedgerEntryData {
    Account(AccountEntry),
    Trustline(TrustLineEntry),
    Offer(OfferEntry),
    Data(DataEntry),
    ClaimableBalance(ClaimableBalanceEntry),
    LiquidityPool(LiquidityPoolEntry),
    ContractData(ContractDataEntry),
    ContractCode(ContractCodeEntry),
    ConfigSetting(ConfigSettingEntry),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct AccountEntry {
    pub account_id: AccountId,
    pub balance: i64,
    pub seq_num: SequenceNumber,
    pub num_sub_entries: u32,
    pub inflation_dest: Option<AccountId>,
    pub flags: u32,
    pub home_domain: String32,
    pub thresholds: Thresholds,
    pub signers: VecM<Signer, 20>,
    pub ext: AccountEntryExt,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct SequenceNumber(pub i64);

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
pub struct Thresholds(pub [u8; 4]);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct Signer {
    pub key: SignerKey,
    pub weight: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum SignerKey {
    Ed25519(Uint256),
    PreAuthTx(Uint256),
    HashX(Uint256),
    Ed25519SignedPayload(SignerKeyEd25519SignedPayload),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum AccountEntryExt {
    V0,
    V1(AccountEntryExtensionV1),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct AccountEntryExtensionV1 {
    pub liabilities: Liabilities,
    pub ext: AccountEntryExtensionV1Ext,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct Liabilities {
    pub buying: i64,
    pub selling: i64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum AccountEntryExtensionV1Ext {
    V0,
    V2(AccountEntryExtensionV2),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct AccountEntryExtensionV2 {
    pub num_sponsored: u32,
    pub num_sponsoring: u32,
    pub signer_sponsoring_i_ds: VecM<SponsorshipDescriptor, 20>,
    pub ext: AccountEntryExtensionV2Ext,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum AccountEntryExtensionV2Ext {
    V0,
    V3(AccountEntryExtensionV3),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct AccountEntryExtensionV3 {
    pub ext: ExtensionPoint,
    pub seq_ledger: u32,
    pub seq_time: TimePoint,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ExtensionPoint {
    V0,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct TrustLineEntry {
    pub account_id: AccountId,
    pub asset: TrustLineAsset,
    pub balance: i64,
    pub limit: i64,
    pub flags: u32,
    pub ext: TrustLineEntryExt,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum TrustLineEntryExt {
    V0,
    V1(TrustLineEntryV1),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct TrustLineEntryV1 {
    pub liabilities: Liabilities,
    pub ext: TrustLineEntryV1Ext,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum TrustLineEntryV1Ext {
    V0,
    V2(TrustLineEntryExtensionV2),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct TrustLineEntryExtensionV2 {
    pub liquidity_pool_use_count: i32,
    pub ext: TrustLineEntryExtensionV2Ext,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum TrustLineEntryExtensionV2Ext {
    V0,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum Asset {
    Native,
    CreditAlphanum4(AlphaNum4),
    CreditAlphanum12(AlphaNum12),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct OfferEntry {
    pub seller_id: AccountId,
    pub offer_id: i64,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: i64,
    pub price: Price,
    pub flags: u32,
    pub ext: OfferEntryExt,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct Price {
    pub n: i32,
    pub d: i32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum OfferEntryExt {
    V0,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct DataEntry {
    pub account_id: AccountId,
    pub data_name: String64,
    pub data_value: DataValue,
    pub ext: DataEntryExt,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum DataEntryExt {
    V0,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct DataValue(pub BytesM<64>);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ClaimableBalanceEntry {
    pub balance_id: ClaimableBalanceId,
    pub claimants: VecM<Claimant, 10>,
    pub asset: Asset,
    pub amount: i64,
    pub ext: ClaimableBalanceEntryExt,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum Claimant {
    ClaimantTypeV0(ClaimantV0),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ClaimantV0 {
    pub destination: AccountId,
    pub predicate: ClaimPredicate,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimPredicate {
    Unconditional,
    And(VecM<ClaimPredicate, 2>),
    Or(VecM<ClaimPredicate, 2>),
    Not(Option<Box<ClaimPredicate>>),
    BeforeAbsoluteTime(i64),
    BeforeRelativeTime(i64),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimableBalanceEntryExt {
    V0,
    V1(ClaimableBalanceEntryExtensionV1),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ClaimableBalanceEntryExtensionV1 {
    pub ext: ClaimableBalanceEntryExtensionV1Ext,
    pub flags: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimableBalanceEntryExtensionV1Ext {
    V0,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LiquidityPoolEntry {
    pub liquidity_pool_id: PoolId,
    pub body: LiquidityPoolEntryBody,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum LiquidityPoolEntryBody {
    LiquidityPoolConstantProduct(LiquidityPoolEntryConstantProduct),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LiquidityPoolEntryConstantProduct {
    pub params: LiquidityPoolConstantProductParameters,
    pub reserve_a: i64,
    pub reserve_b: i64,
    pub total_pool_shares: i64,
    pub pool_shares_trust_line_count: i64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct LiquidityPoolConstantProductParameters {
    pub asset_a: Asset,
    pub asset_b: Asset,
    pub fee: i32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ContractDataEntry {
    pub contract: ScAddress,
    pub key: ScVal,
    pub durability: ContractDataDurability,
    pub body: ContractDataEntryBody,
    pub expiration_ledger_seq: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ContractDataEntryBody {
    DataEntry(ContractDataEntryData),
    ExpirationExtension,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ContractDataEntryData {
    pub flags: u32,
    pub val: ScVal,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ContractCodeEntry {
    pub ext: ExtensionPoint,
    pub hash: Hash,
    pub body: ContractCodeEntryBody,
    pub expiration_ledger_seq: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ContractCodeEntryBody {
    DataEntry(BytesM),
    ExpirationExtension,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ConfigSettingEntry {
    ContractMaxSizeBytes(u32),
    ContractComputeV0(ConfigSettingContractComputeV0),
    ContractLedgerCostV0(ConfigSettingContractLedgerCostV0),
    ContractHistoricalDataV0(ConfigSettingContractHistoricalDataV0),
    ContractMetaDataV0(ConfigSettingContractMetaDataV0),
    ContractBandwidthV0(ConfigSettingContractBandwidthV0),
    ContractCostParamsCpuInstructions(ContractCostParams),
    ContractCostParamsMemoryBytes(ContractCostParams),
    ContractDataKeySizeBytes(u32),
    ContractDataEntrySizeBytes(u32),
    StateExpiration(StateExpirationSettings),
    ContractExecutionLanes(ConfigSettingContractExecutionLanesV0),
    BucketlistSizeWindow(VecM<u64>),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ConfigSettingContractComputeV0 {
    pub ledger_max_instructions: i64,
    pub tx_max_instructions: i64,
    pub fee_rate_per_instructions_increment: i64,
    pub tx_memory_limit: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ConfigSettingContractLedgerCostV0 {
    pub ledger_max_read_ledger_entries: u32,
    pub ledger_max_read_bytes: u32,
    pub ledger_max_write_ledger_entries: u32,
    pub ledger_max_write_bytes: u32,
    pub tx_max_read_ledger_entries: u32,
    pub tx_max_read_bytes: u32,
    pub tx_max_write_ledger_entries: u32,
    pub tx_max_write_bytes: u32,
    pub fee_read_ledger_entry: i64,
    pub fee_write_ledger_entry: i64,
    pub fee_read1_kb: i64,
    pub bucket_list_target_size_bytes: i64,
    pub write_fee1_kb_bucket_list_low: i64,
    pub write_fee1_kb_bucket_list_high: i64,
    pub bucket_list_write_fee_growth_factor: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ConfigSettingContractHistoricalDataV0 {
    pub fee_historical1_kb: i64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ConfigSettingContractMetaDataV0 {
    pub tx_max_extended_meta_data_size_bytes: u32,
    pub fee_extended_meta_data1_kb: i64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ConfigSettingContractBandwidthV0 {
    pub ledger_max_propagate_size_bytes: u32,
    pub tx_max_size_bytes: u32,
    pub fee_propagate_data1_kb: i64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ContractCostParamEntry {
    pub ext: ExtensionPoint,
    pub const_term: i64,
    pub linear_term: i64,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ContractCostParams(pub VecM<ContractCostParamEntry, 1024>);

impl From<ContractCostParams> for VecM<ContractCostParamEntry, 1024> {
    #[must_use]
    fn from(x: ContractCostParams) -> Self {
        x.0
    }
}

impl From<VecM<ContractCostParamEntry, 1024>> for ContractCostParams {
    #[must_use]
    fn from(x: VecM<ContractCostParamEntry, 1024>) -> Self {
        ContractCostParams(x)
    }
}

impl AsRef<VecM<ContractCostParamEntry, 1024>> for ContractCostParams {
    #[must_use]
    fn as_ref(&self) -> &VecM<ContractCostParamEntry, 1024> {
        &self.0
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ConfigSettingContractExecutionLanesV0 {
    pub ledger_max_tx_count: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct StateExpirationSettings {
    pub max_entry_expiration: u32,
    pub min_temp_entry_expiration: u32,
    pub min_persistent_entry_expiration: u32,
    pub auto_bump_ledgers: u32,
    pub persistent_rent_rate_denominator: i64,
    pub temp_rent_rate_denominator: i64,
    pub max_entries_to_expire: u32,
    pub bucket_list_size_window_sample_size: u32,
    pub eviction_scan_size: u64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct InvokeContractArgs {
    pub contract_address: ScAddress,
    pub function_name: ScSymbol,
    pub args: VecM<ScVal>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ContractIdPreimageFromAddress {
    pub address: ScAddress,
    pub salt: Uint256,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ContractIdPreimage {
    Address(ContractIdPreimageFromAddress),
    Asset(Asset),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum ContractExecutable {
    Wasm(Hash),
    Token,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct CreateContractArgs {
    pub contract_id_preimage: ContractIdPreimage,
    pub executable: ContractExecutable,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(i32)]
pub enum HostFunctionType {
    InvokeContract = 0,
    CreateContract = 1,
    UploadContractWasm = 2,
}

impl HostFunctionType {
    pub const VARIANTS: [HostFunctionType; 3] = [
        HostFunctionType::InvokeContract,
        HostFunctionType::CreateContract,
        HostFunctionType::UploadContractWasm,
    ];
    pub const VARIANTS_STR: [&'static str; 3] =
        ["InvokeContract", "CreateContract", "UploadContractWasm"];

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::InvokeContract => "InvokeContract",
            Self::CreateContract => "CreateContract",
            Self::UploadContractWasm => "UploadContractWasm",
        }
    }

    #[must_use]
    pub const fn variants() -> [HostFunctionType; 3] {
        Self::VARIANTS
    }
}

impl fmt::Display for HostFunctionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl From<HostFunctionType> for i32 {
    #[must_use]
    fn from(e: HostFunctionType) -> Self {
        e as Self
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[allow(clippy::large_enum_variant)]
pub enum HostFunction {
    InvokeContract(InvokeContractArgs),
    CreateContract(CreateContractArgs),
    UploadContractWasm(BytesM),
}

impl HostFunction {
    pub const VARIANTS: [HostFunctionType; 3] = [
        HostFunctionType::InvokeContract,
        HostFunctionType::CreateContract,
        HostFunctionType::UploadContractWasm,
    ];
    pub const VARIANTS_STR: [&'static str; 3] =
        ["InvokeContract", "CreateContract", "UploadContractWasm"];

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::InvokeContract(_) => "InvokeContract",
            Self::CreateContract(_) => "CreateContract",
            Self::UploadContractWasm(_) => "UploadContractWasm",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> HostFunctionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::InvokeContract(_) => HostFunctionType::InvokeContract,
            Self::CreateContract(_) => HostFunctionType::CreateContract,
            Self::UploadContractWasm(_) => HostFunctionType::UploadContractWasm,
        }
    }

    #[must_use]
    pub const fn variants() -> [HostFunctionType; 3] {
        Self::VARIANTS
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(i32)]
pub enum ScErrorType {
    Contract = 0,
    WasmVm = 1,
    Context = 2,
    Storage = 3,
    Object = 4,
    Crypto = 5,
    Events = 6,
    Budget = 7,
    Value = 8,
    Auth = 9,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[repr(i32)]
pub enum ScErrorCode {
    ArithDomain = 0,
    IndexBounds = 1,
    InvalidInput = 2,
    MissingValue = 3,
    ExistingValue = 4,
    ExceededLimit = 5,
    InvalidAction = 6,
    InternalError = 7,
    UnexpectedType = 8,
    UnexpectedSize = 9,
}
