// use sovereign_sdk:
use sovereign_sdk::{
    Chain, ChainProof, Da, DataBlob, ExecutionProof, Header, InclusionProof, Proof,
    StateCommitment, Stf,
};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct H160([u8; 20]);

#[derive(Default, Debug, PartialEq, Clone)]
pub struct H256([u8; 32]);

#[derive(Debug, PartialEq, Clone)]
pub struct ExchequerState {
    root: jmt::RootHash,
}
impl StateCommitment for ExchequerState {
    type Key = Vec<u8>;

    type Value = Vec<u8>;

    fn get(key: Self::Key) -> Self::Value {
        todo!()
    }

    fn put(key: Self::Key, value: Self::Value) -> Self {
        todo!()
    }
}

pub struct Exchequer {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct MockDaTx {
    pub sender: H160,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MockDaHeader {
    transactions_root: jmt::RootHash,
    height: u64,
    prev_root: H256,
}
impl Header for MockDaHeader {
    type Hash = H256;

    fn height(&self) -> u64 {
        self.height
    }

    fn hash(&self) -> Self::Hash {
        todo!()
    }
}
pub struct MockDaLayer {}
impl sovereign_sdk::DataBlob for MockDaTx {
    type Metadata = H160;

    type Data = Vec<u8>;

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn metadata(&self) -> &Self::Metadata {
        &self.sender
    }
}

pub struct AuthenticatedMockDaTx {}
pub enum MockInclusionError {
    InvalidProof,
}
impl InclusionProof for AuthenticatedMockDaTx {
    type SignedData = MockDaTx;

    type BlockHash = H256;

    type Error = MockInclusionError;

    fn verify(self, blockhash: &Self::BlockHash) -> Result<Self::SignedData, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub enum MockDaError {}
impl Da for MockDaLayer {
    type BlockHash = H256;

    type Header = MockDaHeader;

    type SignedDataWithInclusionProof = AuthenticatedMockDaTx;

    type SignedData = MockDaTx;

    type CompletenessProof = jmt::proof::SparseMerkleRangeProof;

    type Error = MockDaError;

    fn verify_potential_block_list(
        da_header: Self::Header,
        potential_blocks: Vec<Self::SignedDataWithInclusionProof>,
        completeness_proof: Self::CompletenessProof,
    ) -> Result<Vec<Self::SignedData>, Self::Error> {
        todo!()
    }
}

pub struct ExchequerOverMockData {}
#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {}
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub transactions: Vec<Transaction>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum ExchequerError {}
impl Stf for ExchequerOverMockData {
    type Block = Block;

    type StateRoot = ExchequerState;

    type SignedData = MockDaTx;

    type Misbehavior = ();

    type Error = ExchequerError;

    fn validate_opaque_blob(blob: &Self::SignedData, prev_state: &Self::StateRoot) -> bool {
        todo!()
    }

    fn prepare_block(
        blob: Self::SignedData,
        prev_state: &Self::StateRoot,
        misbehavior_hint: Option<Self::Misbehavior>,
    ) -> Result<Self::Block, Self::StateRoot> {
        todo!()
    }

    fn apply_block(blk: Self::Block, prev_state: &Self::StateRoot) -> Self::StateRoot {
        todo!()
    }
}

pub struct ExchequerExecutionProof {}
impl Proof for ExchequerChainProofOverMockDa {
    type VerificationError = ();

    type MethodId = H256;
    const MethodId: Self::MethodId = H256([0u8; 32]);

    fn authenicated_log(&self) -> &[u8] {
        todo!()
    }

    fn verify(&self) -> Result<(), Self::VerificationError> {
        todo!()
    }
}
impl ExecutionProof for ExchequerExecutionProof {
    type Rollup = ExchequerOverMockData;

    type DaLayer = MockDaLayer;

    fn blobs_applied(&self) -> &[<<Self as ExecutionProof>::DaLayer as Da>::SignedData] {
        todo!()
    }

    fn pre_state_root(&self) -> <<Self as ExecutionProof>::Rollup as Stf>::StateRoot {
        todo!()
    }

    fn post_state_root(&self) -> <<Self as ExecutionProof>::Rollup as Stf>::StateRoot {
        todo!()
    }
}
impl ChainProof for ExchequerChainProofOverMockDa {
    type DaLayer = MockDaLayer;

    type Rollup = ExchequerOverMockData;

    type SignedData = MockDaTx;

    fn da_hash(&self) -> <<Self as ChainProof>::DaLayer as Da>::BlockHash {
        todo!()
    }

    fn state_root(&self) -> <<Self as ChainProof>::Rollup as Stf>::StateRoot {
        todo!()
    }
}
impl Proof for ExchequerExecutionProof {
    type VerificationError = ();

    type MethodId = H256;
    const MethodId: Self::MethodId = H256([0u8; 32]);

    fn authenicated_log(&self) -> &[u8] {
        todo!()
    }

    fn verify(&self) -> Result<(), Self::VerificationError> {
        todo!()
    }
}
pub struct ExchequerChainProofOverMockDa {}

impl Chain for ExchequerOverMockData {
    type DataBlob = MockDaTx;

    type DaLayer = MockDaLayer;

    type Rollup = ExchequerOverMockData;

    type ChainProof = ExchequerChainProofOverMockDa;

    type ExecutionProof = ExchequerExecutionProof;
}
fn main() {
    println!("Hello, world!");
}
