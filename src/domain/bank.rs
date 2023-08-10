pub struct Bank {
    pub compe: COMPE,
    pub ispb: ISPB,
    pub document: DocumentCompany,
    pub long_name: String,
    pub short_name: String,
    pub network: Option<String>,
    pub type: Option<String>,
    pub pix_type: Option<String>,
    pub charge: bool,
    pub credit_document: bool,
    pub legal_cheque: bool,
    pub detecta_flow: bool,
    pub salary_portability: String,
    pub products: Vec<String>,
    pub url: Option<String>,
    pub date_operation_started: String,
    pub date_pix_started: String,
    pub date_registered: String,
    pub date_updated: String,
}

pub struct DocumentCompany {
    str: String,
}

impl DocumentCompany {
    pub fn new(str: String) -> Option<Self> {
        if str.chars().count() == 3 {
            Some(DocumentCompany { str })
        } else {
            None
        }
    }
}

pub struct BankToTransfer {
    pub compe: COMPE,
}

pub struct COMPE {
    str: String,
}

impl COMPE {
    pub fn new(str: String) -> Option<Self> {
        if str.chars().count() == 3 {
            Some(COMPE { str })
        } else {
            None
        }
    }
}

pub struct ISPB {
    str: String,
}

impl ISPB {
    pub fn new(str: String) -> Option<Self> {
        if str.chars().count() == 8 {
            Some(COMPE)
        } else {
            None
        }
    }
}