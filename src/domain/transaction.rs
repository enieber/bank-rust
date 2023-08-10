pub struct Transaction {
    pub amount: Amount,
    pub accoutToTransfer: AccountToTransfer,
    pub accountToReceive: AccountPersonal,
    pub date: Date,
    pub type_account: TypeAccount,
}

pub enum TypeAccount {
    Current,
}
